use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// the fields in these structs are mostly duplicated from code in src/profile/parser.rs
#[derive(Clone, Debug)]
pub struct FitProfile {
    field_types: Vec<FieldTypeDefintion>,
    messages: Vec<MessageDefinition>,
}

#[derive(Clone, Debug)]
struct FieldTypeDefintion {
    name: String,
    titlized_name: String,
    base_type: &'static str,
    variant_map: BTreeMap<i64, FieldTypeVariant>,
}

impl FieldTypeDefintion {
    fn new(name: String, base_type: &'static str) -> Self {
        FieldTypeDefintion {
            name: name.clone(),
            titlized_name: titlecase_string(&name),
            base_type,
            variant_map: BTreeMap::new(),
        }
    }

    /// Generate an enum from the field type variants
    fn generate_enum(&self, out: &mut File) -> Result<(), std::io::Error> {
        write!(out, "#[derive(Clone, Copy, Debug, Serialize)]\n")?;
        write!(out, "pub enum {} {{\n", titlecase_string(&self.name))?;
        for variant in self.variant_map.values() {
            variant.write_variant_line(out)?;
        }
        write!(out, "UnknownVariant({}),\n", self.base_type)?;
        write!(out, "}}\n")?;

        self.generate_impl(out)
    }

    /// generate to/from integer and to_string implementation for field type enum
    fn generate_impl(&self, out: &mut File) -> Result<(), std::io::Error> {
        write!(out, "impl {} {{\n", titlecase_string(&self.name))?;
        write!(
            out,
            "pub fn from_{0}(value: {0}) -> {1} {{\n",
            self.base_type,
            titlecase_string(&self.name)
        )?;
        write!(out, "match value {{\n")?;
        for variant in self.variant_map.values() {
            write!(
                out,
                "{} => {}::{},\n",
                variant.value,
                titlecase_string(&self.name),
                variant.titlized_name()
            )?;
        }
        write!(
            out,
            " _ => {}::UnknownVariant(value)\n",
            titlecase_string(&self.name)
        )?;
        write!(out, "}}\n")?;
        write!(out, "}}\n")?;

        write!(
            out,
            "pub fn from_i64(value: i64) -> {} {{\n",
            titlecase_string(&self.name)
        )?;
        write!(
            out,
            "{0}::from_{1}(value as {1})\n",
            titlecase_string(&self.name),
            self.base_type
        )?;
        write!(out, "}}\n")?;

        write!(out, "pub fn as_{0}(&self) -> {0} {{\n", self.base_type)?;
        write!(out, "match &self {{\n")?;
        for variant in self.variant_map.values() {
            write!(
                out,
                "{}::{} => {},\n",
                titlecase_string(&self.name),
                variant.titlized_name(),
                variant.value
            )?;
        }
        write!(
            out,
            "{}::UnknownVariant(value) => *value\n",
            titlecase_string(&self.name)
        )?;
        write!(out, "}}\n")?;
        write!(out, "}}\n")?;

        write!(out, "pub fn to_string(&self) -> String {{\n")?;
        write!(out, "match &self {{\n")?;
        for variant in self.variant_map.values() {
            write!(
                out,
                "{}::{} => \"{}\".to_string(),\n",
                titlecase_string(&self.name),
                variant.titlized_name(),
                variant.name
            )?;
        }
        write!(
            out,
            "{}::UnknownVariant(value) => format!(\"unknown_variant_{{}}\", *value)\n",
            titlecase_string(&self.name)
        )?;
        write!(out, "}}\n")?;
        write!(out, "}}\n")?;
        write!(out, "}}\n\n")?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct FieldTypeVariant {
    name: String,
    value: i64,
    comment: Option<String>,
}

impl FieldTypeVariant {
    fn titlized_name(&self) -> String {
        let mut titlized_name = titlecase_string(&self.name);

        // First letter isn't between A-Z in ASCII
        let first_let = titlized_name.as_bytes()[0];
        if first_let < 65 || first_let > 90 {
            titlized_name = format!("Name{}", titlized_name);
        }

        titlized_name
    }

    fn write_variant_line(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = &self.comment {
            write!(out, "{}, // {}\n", self.titlized_name(), v)
        } else {
            write!(out, "{},\n", self.titlized_name())
        }
    }
}

#[derive(Clone, Debug)]
struct MessageDefinition {
    name: String,
    field_map: BTreeMap<u8, MessageFieldDefinition>,
}

impl MessageDefinition {
    fn new(name: &str) -> Self {
        MessageDefinition {
            name: name.to_string(),
            field_map: BTreeMap::new(),
        }
    }

    fn function_name(&self) -> String {
        format!("{}_message", self.name)
    }

    fn write_function_def(&self, out: &mut File) -> Result<(), std::io::Error> {
        write!(out, "fn {}() -> MessageInfo {{\n", self.function_name())?;
        write!(out, "    let mut fields = HashMap::new();\n\n")?;
        for field in self.field_map.values() {
            field.generate_field_info_struct(out, "field")?;
            write!(out, "fields.insert({}, field);\n\n", field.def_number)?;
        }
        write!(out, "    MessageInfo {{\n")?;
        write!(out, "        name: \"{}\",\n", self.name)?;
        write!(out, "        fields: fields\n")?;
        write!(out, "    }}\n")?;
        write!(out, "}}\n\n")?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct MessageFieldDefinition {
    def_number: u8,
    name: String,
    field_type: String,
    scale: f64,
    offset: f64,
    units: String,
    // TODO components and reference fields
    comment: Option<String>,
}

impl MessageFieldDefinition {
    pub fn generate_field_info_struct(
        &self,
        out: &mut File,
        var_name: &str,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = &self.comment {
            write!(out, "// {}\n", v)?;
        }
        write!(
            out,
            "    let {} = FieldInfo {{
            name: \"{}\",
            field_type: {},
            def_number: {},
            scale: {:.6},
            offset: {:.6},
            units: \"{}\"
        }};\n",
            var_name,
            self.name,
            self.field_type,
            self.def_number,
            self.scale,
            self.offset,
            self.units,
        )?;

        Ok(())
    }
}

/// Convert an ASCII string to title/camel case
fn titlecase_string(value: &str) -> String {
    let mut words: Vec<String> = value.split('_').map(|v| v.to_string()).collect();

    for word in &mut words {
        if let Some(l) = word.get_mut(0..1) {
            l.make_ascii_uppercase();
        }
    }

    String::from(words.join(""))
}

/// Match a base type string to a rust type for enum generation
fn base_type_to_rust_type(base_type_str: &str) -> &'static str {
    match base_type_str {
        "enum" => "u8",
        "sint8" => "i8",
        "uint8" => "u8",
        "uint8z" => "u8",
        "sint16" => "i16",
        "uint16" => "u16",
        "uint16z" => "u16",
        "sint32" => "i32",
        "uint32" => "u32",
        "uint32z" => "u32",
        _ => panic!(format!(
            "unsupported base_type for enum field: {}",
            base_type_str
        )),
    }
}

/// match the field type string to a simple type or an enum
fn field_type_str_to_field_type(field_type_str: &str) -> String {
    match field_type_str {
        "sint8" => "FieldDataType::SInt8".to_string(),
        "uint8" => "FieldDataType::UInt8".to_string(),
        "sint16" => "FieldDataType::SInt16".to_string(),
        "uint16" => "FieldDataType::UInt16".to_string(),
        "sint32" => "FieldDataType::SInt32".to_string(),
        "uint32" => "FieldDataType::UInt32".to_string(),
        "string" => "FieldDataType::String".to_string(),
        "float32" => "FieldDataType::Float32".to_string(),
        "float64" => "FieldDataType::Float64".to_string(),
        "uint8z" => "FieldDataType::UInt8z".to_string(),
        "uint16z" => "FieldDataType::UInt16z".to_string(),
        "uint32z" => "FieldDataType::UInt32z".to_string(),
        "byte" => "FieldDataType::Byte".to_string(),
        "sint64" => "FieldDataType::SInt64".to_string(),
        "uint64" => "FieldDataType::UInt64".to_string(),
        "uint64z" => "FieldDataType::UInt64z".to_string(),
        _ => format!("FieldDataType::{}", titlecase_string(field_type_str)),
    }
}

fn generate_main_field_type_enum(
    field_types: &[FieldTypeDefintion],
    out: &mut File,
) -> Result<(), std::io::Error> {
    let base_types = vec![
        "Bool", "SInt8", "UInt8", "SInt16", "UInt16", "SInt32", "UInt32", "String", "Float32",
        "Float64", "UInt8z", "UInt16z", "UInt32z", "Byte", "SInt64", "UInt64", "UInt64z",
    ];
    let mut is_enum_force_false = HashSet::new();
    is_enum_force_false.insert("date_time".to_string());
    is_enum_force_false.insert("local_date_time".to_string());

    write!(
        out,
        "
/// Describe all possible data types of a field
///
/// The Enum type's value is actually an enum of enums.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum FieldDataType {{
"
    )?;
    for type_name in base_types {
        write!(out, " {},\n", type_name)?;
    }
    for field_type in field_types {
        write!(out, "{},\n", field_type.titlized_name)?;
    }

    write!(out, "}}\n\n")?;

    write!(out, "impl FieldDataType {{\n")?;
    write!(out, "    pub fn is_enum_type(&self) -> bool {{\n")?;
    write!(out, "        match self {{\n")?;
    for field_type in field_types {
        if !field_type.variant_map.is_empty() && !is_enum_force_false.contains(&field_type.name) {
            write!(
                out,
                "FieldDataType::{} => true,\n",
                field_type.titlized_name
            )?;
        }
    }
    write!(out, "            _ => false,\n")?;
    write!(out, "        }}\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n")?;

    write!(
        out,
        "pub fn get_field_variant_as_string(field_type: FieldDataType , value: i64) -> String {{\n"
    )?;
    write!(out, "    match field_type {{\n")?;
    for field_type in field_types {
        if !field_type.variant_map.is_empty() && !is_enum_force_false.contains(&field_type.name) {
            write!(
                out,
                "        FieldDataType::{0} => {0}::from_i64(value).to_string(),\n",
                field_type.titlized_name
            )?;
        }
    }
    write!(out, "        _ => format!(\"Undefined{{}}\", value),\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n")?;

    Ok(())
}

fn process_types(sheet: Range<DataType>) -> Vec<FieldTypeDefintion> {
    let mut field_types: Vec<FieldTypeDefintion> = Vec::new();

    for row in sheet.rows().skip(1) {
        if !row[0].is_empty() {
            // extract enum name
            let enum_name = match row[0].get_string() {
                Some(v) => v.to_string(),
                None => panic!(format!("Enum type name must be a string row={:?}.", row)),
            };

            // extract base type and convert to it's rust equivalent
            let rust_type = match row[1].get_string() {
                Some(v) => base_type_to_rust_type(v),
                None => panic!(format!("Base type name must be a string row={:?}.", row)),
            };
            field_types.push(FieldTypeDefintion::new(enum_name, rust_type));
        } else if !row[2].is_empty() {
            let field_type = match field_types.last_mut() {
                Some(v) => v,
                None => panic!("field_types vector was empty!"),
            };
            // add enum variant
            // extract enum name
            let name = match row[2].get_string() {
                Some(v) => v.to_string(),
                None => panic!(format!("Enum variant name must be a string row={:?}.", row)),
            };

            // handle mix of numeric and hex string data values
            let value = match &row[3] {
                DataType::Float(v) => *v as i64,
                DataType::Int(v) => *v,
                DataType::String(v) => i64::from_str_radix(&v[2..], 16).unwrap(),
                _ => {
                    panic!(format!(
                        "Unsupported enum variant value data type row={:?}.",
                        row
                    ));
                }
            };
            let comment = match row[4].get_string() {
                Some(v) => Some(v.to_string()),
                None => None,
            };
            field_type.variant_map.insert(
                value,
                FieldTypeVariant {
                    name,
                    value,
                    comment,
                },
            );
        }
    }

    field_types
}

fn new_message_field_definition(row: &[DataType]) -> MessageFieldDefinition {
    let def_number = match row[1] {
        DataType::Float(v) => v as u8,
        DataType::Int(v) => v as u8,
        _ => panic!(format!(
            "Field defintiton number must be an integer, row={:?}.",
            row
        )),
    };
    let name = row[2]
        .get_string()
        .expect(&format!("Field name must be a string, row={:?}.", row));
    let ftype = row[3]
        .get_string()
        .expect(&format!("Field type must be a string, row={:?}.", row));
    let comment = match row[4].get_string() {
        Some(v) => Some(v.to_string()),
        None => None,
    };

    MessageFieldDefinition {
        def_number,
        name: name.to_string(),
        field_type: field_type_str_to_field_type(ftype),
        scale: row[6].get_float().unwrap_or(1.0),
        offset: row[7].get_float().unwrap_or(0.0),
        units: row[8].get_string().unwrap_or("").to_string(),
        comment,
    }
}

fn process_messages(sheet: Range<DataType>) -> Vec<MessageDefinition> {
    let mut rows = sheet.rows().skip(2);
    let mut messages: Vec<MessageDefinition> = Vec::new();
    let mut msg: MessageDefinition;
    let mut field: MessageFieldDefinition;
    let mut last_def_number: u8 = 0;

    // parse first message row to initialize first message to prevent unitialized compile error in loop
    let row = rows.next().unwrap();
    if let Some(v) = row[0].get_string() {
        msg = MessageDefinition::new(v);
    } else {
        panic!(format!("Message name must be a string row={:?}.", row));
    }

    // process messages and fields
    for row in rows {
        // begin new message function
        if !row[0].is_empty() {
            if let Some(v) = row[0].get_string() {
                messages.push(msg);
                msg = MessageDefinition::new(v);
            } else {
                panic!(format!("Message name must be a string row={:?}.", row));
            }
        } else if !row[1].is_empty() {
            field = new_message_field_definition(row);
            last_def_number = field.def_number;
            msg.field_map.insert(field.def_number, field);
        } else {
            // TODO handle subfield using the last_def_number
        }
    }
    messages.push(msg);

    messages
}

pub fn parse_profile(profile_fname: &str) -> Result<FitProfile, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(&profile_fname)?;

    // process Types sheet
    let field_types = if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet)
    } else {
        panic!("Could not access workbook sheet 'Types'");
    };

    // process Messages sheet
    let messages = if let Some(Ok(sheet)) = excel.worksheet_range("Messages") {
        process_messages(sheet)
    } else {
        panic!("Could not access workbook sheet 'Messages'");
    };

    Ok(FitProfile {
        field_types,
        messages,
    })
}

fn create_mesg_num_to_mesg_info_fn(
    messages: &Vec<MessageDefinition>,
    out: &mut File,
) -> Result<(), std::io::Error> {
    write!(out, "impl MesgNum {{\n")?;
    write!(out, "    pub fn message_info(&self) -> MessageInfo {{\n")?;
    write!(out, "        match self {{\n")?;
    for msg in messages {
        write!(
            out,
            "            MesgNum::{} => {}(),\n",
            titlecase_string(&msg.name),
            msg.function_name()
        )?;
    }
    write!(out, "            _ => unknown_message(),\n")?;
    write!(out, "        }}\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n")?;

    Ok(())
}

/// call rustfmt on a generated file to cleanup auto-gen code
fn rustfmt(fname: &str) {
    Command::new("rustfmt")
        .arg(&fname)
        .status()
        .expect(&format!("failed to execute rustfmt on {}", fname));
}

fn write_types_files(profile: &FitProfile) -> Result<(), std::io::Error> {
    let fname = "src/profile/field_types.rs";
    let mut out = File::create(&fname)?;

    write!(
        out,
        "/// Auto generated profile from FIT SDK Release: XXX\n\n"
    )?;
    write!(out, "use serde::Serialize;\n")?;

    // output enums and implementations
    for field_type in &profile.field_types {
        if !field_type.variant_map.is_empty() {
            field_type.generate_enum(&mut out)?;
        }
    }
    generate_main_field_type_enum(&profile.field_types, &mut out)?;

    rustfmt(&fname);
    Ok(())
}

fn write_messages_file(profile: &FitProfile) -> Result<(), std::io::Error> {
    let fname = "src/profile/messages.rs";
    let mut out = File::create(&fname)?;

    write!(
        out,
        "/// Auto generated profile from FIT SDK Release: XXX\n"
    )?;
    write!(out, "use std::collections::HashMap;\n")?;
    write!(
        out,
        "use super::{{MessageInfo, FieldDataType, FieldInfo}};\n"
    )?;
    write!(out, "use super::field_types::*;\n\n")?;

    // output all message functions
    for msg in &profile.messages {
        msg.write_function_def(&mut out)?;
    }

    // output an unknown_message() fn that has no fields
    write!(out, "fn unknown_message() -> MessageInfo {{\n")?;
    write!(out, "    MessageInfo {{\n")?;
    write!(out, "        name: \"unknown\",\n")?;
    write!(out, "        fields: HashMap::new()\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n\n")?;

    // output MesgNum implementation to allow parser to fetch info
    create_mesg_num_to_mesg_info_fn(&profile.messages, &mut out)?;

    rustfmt(&fname);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile_fname = "vendor/FitSDK/Profile.xlsx";
    // only re-run if this file changes since the code we generate depends on it.
    // todo figure out a better way to trigger the build, maybe using an env var?
    println!("cargo:rerun-if-changed={}", &profile_fname);

    // process excel file and output
    let profile = parse_profile(&profile_fname).unwrap();
    write_types_files(&profile)?;
    write_messages_file(&profile)?;

    Ok(())
}
