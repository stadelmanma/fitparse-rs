use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// the fields in these structs are mostly duplicated from code in src/profile/parser.rs
#[derive(Clone, Debug)]
pub struct FitProfile {
    field_types: Vec<FieldTypeDefintion>,
    messages: Vec<MessageDefinition>
}

#[derive(Clone, Debug)]
struct FieldTypeDefintion {
    name: String,
    base_type: &'static str,
    variant_map: BTreeMap<i64, FieldTypeVariant>,
}

impl FieldTypeDefintion {
    fn new(name: String, base_type: &'static str) -> Self {
        FieldTypeDefintion {
            name,
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
            self.base_type, titlecase_string(&self.name)
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
        write!(out, " _ => {}::UnknownVariant(value)\n", titlecase_string(&self.name))?;
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
            titlecase_string(&self.name), self.base_type
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
    field_map: BTreeMap<u8, MessageFieldDefinition>
}

impl MessageDefinition {
    fn new(name: &str) -> Self {
        MessageDefinition {
            name: name.to_string(),
            field_map: BTreeMap::new(),
        }
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
    let name = row[2].get_string().expect(&format!("Field name must be a string, row={:?}.", row));
    let ftype = row[3].get_string().expect(&format!("Field type must be a string, row={:?}.", row));
    let comment = match row[4].get_string() {
        Some(v) => Some(v.to_string()),
        None => None,
    };

    MessageFieldDefinition{
        def_number,
        name: name.to_string(),
        field_type: ftype.to_string(),
        scale: row[6].get_float().unwrap_or(1.0),
        offset: row[7].get_float().unwrap_or(0.0),
        units: row[8].get_string().unwrap_or("").to_string(),
        comment
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
        }
        else if !row[1].is_empty() {
            field = new_message_field_definition(row);
            last_def_number = field.def_number;
            msg.field_map.insert(field.def_number, field);
        }
        else {
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

    Ok(FitProfile{field_types, messages})
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
    write_types_files(&profile);
    write_messages_file(&profile)?;

    Ok(())
}
