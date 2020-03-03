/// Process the FIT profile excel workook and generate the profile module files.
///
/// The entries in the "Types" sheet name and describe the possible values for any Enum field types.
/// The entries in the "Messages" sheet name and number the fields within a given message and specify
/// the type of a field which can be an enum or a primitive. Primitive types may also have units,
/// scale and offset defined which describe how to transform the primitive integer type into a floating
/// point value.
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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

/// Output an enum variant to the field types output file
fn write_variant_line(
    out: &mut File,
    variant_name: &str,
    comment: Option<&str>,
) -> Result<(), std::io::Error> {
    if let Some(v) = comment {
        write!(out, "    {}, // {}\n", variant_name, v)?;
    } else {
        write!(out, "    {},\n", variant_name)?;
    }

    Ok(())
}

/// Create an implementation to match enums to their numeric equivalent
fn generate_enum_impl(
    enum_name: &str,
    rust_type: &str,
    variant_map: &BTreeMap<i64, String>,
    out: &mut File,
) -> Result<(), std::io::Error> {
    write!(out, "impl {} {{\n", enum_name)?;
    write!(
        out,
        "    pub fn from_{0}(value: {0}) -> {1} {{\n",
        rust_type, enum_name
    )?;
    write!(out, "        match value {{\n")?;
    for (enum_val, var_name) in variant_map.iter() {
        write!(
            out,
            "            {} => {}::{},\n",
            enum_val, enum_name, var_name
        )?;
    }
    write!(
        out,
        "            _ => {}::UnknownVariant(value)\n",
        enum_name
    )?;
    write!(out, "        }}\n")?;
    write!(out, "    }}\n")?;

    write!(out, "    pub fn from_i64(value: i64) -> {} {{\n", enum_name)?;
    write!(out, "        {0}::from_{1}(value as {1})\n", enum_name, rust_type)?;
    write!(out, "    }}\n")?;

    write!(out, "    pub fn as_{0}(&self) -> {0} {{\n", rust_type)?;
    write!(out, "        match &self {{\n")?;
    for (enum_val, var_name) in variant_map.iter() {
        write!(
            out,
            "            {}::{} => {},\n",
            enum_name, var_name, enum_val
        )?;
    }
    write!(
        out,
        "            {}::UnknownVariant(value) => *value\n",
        enum_name
    )?;
    write!(out, "        }}\n")?;
    write!(out, "    }}\n")?;

    write!(out, "}}\n\n")?;

    Ok(())
}

fn end_field_type_enum(enum_name: &str, rust_type: &str, variant_map: &BTreeMap<i64, String>, out: &mut File) -> Result<(), std::io::Error> {
    write!(out, "    UnknownVariant({}),\n", rust_type)?;
    write!(out, "}}\n\n")?;
    generate_enum_impl(&enum_name, &rust_type, &variant_map, out)?;

    Ok(())
}

fn generate_main_field_type_enum(field_enums: &[String], out: &mut File)  -> Result<(), std::io::Error> {
    write!(out, "
/// Describe all possible data types within of a field
///
/// The Enum type's value is actually an enum of enums.
#[derive(Clone, Copy, Debug)]
pub enum FieldDataType {{
    Bool,
    SInt8,
    UInt8,
    SInt16,
    UInt16,
    SInt32,
    UInt32,
    String,
    Float32,
    Float64,
    UInt8z,
    UInt16z,
    UInt32z,
    Byte,
    SInt64,
    UInt64,
    UInt64z,\n")?;
    for enum_name in field_enums {
        write!(out, "    {0},\n", enum_name)?;
    }
    write!(out, "}}\n\n")?;

    Ok(())
}

fn process_types(sheet: Range<DataType>, out: &mut File) -> Result<(), std::io::Error> {
    let mut rust_type = "";
    let mut enum_name = String::new();
    let mut add_close_brace = false;
    let mut variant_map = BTreeMap::new();
    let mut field_enums = Vec::new();

    for row in sheet.rows().skip(1) {
        // begin new enum
        if !row[0].is_empty() {
            if add_close_brace {
                end_field_type_enum(&enum_name, &rust_type, &variant_map, out)?;
                variant_map.clear();
            }

            // extract enum name
            if let Some(v) = row[0].get_string() {
                enum_name = titlecase_string(v);
                field_enums.push(enum_name.clone());
            } else {
                panic!(format!("Enum type name must be a string row={:?}.", row));
            }

            // extract base type and convert to it's rust equivalent
            if let Some(v) = row[1].get_string() {
                rust_type = base_type_to_rust_type(v);
            } else {
                panic!(format!("Base type name must be a string row={:?}.", row));
            }

            write!(out, "#[derive(Clone, Copy, Debug)]\n")?;
            write!(out, "pub enum {} {{\n", enum_name)?;
            add_close_brace = true;
        }
        // add enum variant
        if !row[2].is_empty() {
            let mut variant_name: String;

            // extract enum name and possible comment
            if let Some(v) = row[2].get_string() {
                variant_name = titlecase_string(v);

                // First letter isn't between A-Z in ASCII
                let first_let = variant_name.as_bytes()[0];
                if first_let < 65 || first_let > 90 {
                    variant_name = format!("Name{}", variant_name);
                }
            } else {
                panic!(format!("Enum variant name must be a string row={:?}.", row));
            }
            let comment = row[4].get_string();

            // handle mix of numeric and hex string data values
            let val_str = match &row[3] {
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
            if !variant_map.contains_key(&val_str) {
                write_variant_line(out, &variant_name, comment)?;
                variant_map.insert(val_str, variant_name);
            }
        }
    }
    // ouput info for final enum
    end_field_type_enum(&enum_name, &rust_type, &variant_map, out)?;

    generate_main_field_type_enum(&field_enums, out)?;

    Ok(())
}

fn end_message_fn(mesg_name: &str, out: &mut File) -> Result<(), std::io::Error> {
    write!(out, "    MessageInfo {{\n")?;
    write!(out, "        name: \"{}\",\n", mesg_name)?;
    write!(out, "        fields: fields\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n\n")?;

    Ok(())
}

fn create_field_info_struct(row: &[DataType], out: &mut File) -> Result<(), std::io::Error> {
    if row[1].is_empty() {
        // TODO subfield definition
        return Ok(());
    }
    let def_num = match row[1] {
        DataType::Float(v) => v as u8,
        DataType::Int(v) => v as u8,
        _ => panic!(format!(
            "Field defintiton number must be an interger, row={:?}.",
            row
        )),
    };
    let field_type = match row[3].get_string() {
        Some(v) => field_type_str_to_field_type(v),
        None => panic!(format!("Field type must be a string, row={:?}.", row))
    };


    write!(
        out,
        "    let field = FieldInfo {{
        name: \"{}\",
        field_type: {},
        def_number: {},
        scale: {:.6},
        offset: {:.6},
        units: \"{}\"
    }};\n",
        row[2]
            .get_string()
            .expect(&format!("Field name must be a string, row={:?}.", row)),
        field_type,
        def_num,
        row[6].get_float().unwrap_or(1.0),
        row[7].get_float().unwrap_or(0.0),
        row[8].get_string().unwrap_or(""),
    )?;
    write!(out, "fields.insert({}, field);\n\n", def_num)?;

    Ok(())
}

fn create_mesg_num_to_mesg_info_fn(
    mesg_fns: &BTreeMap<String, String>,
    out: &mut File,
) -> Result<(), std::io::Error> {
    write!(out, "impl MesgNum {{\n")?;
    write!(
        out,
        "    pub fn message_info(&self) -> Option<MessageInfo> {{\n"
    )?;
    write!(out, "        match self {{\n")?;
    for (mesg_variant, mesg_fn) in mesg_fns.iter() {
        write!(
            out,
            "            MesgNum::{} => Some({}),\n",
            mesg_variant, mesg_fn
        )?;
    }
    write!(out, "            _ => None,\n")?;
    write!(out, "        }}\n")?;
    write!(out, "    }}\n")?;
    write!(out, "}}\n")?;

    Ok(())
}

fn process_messages(sheet: Range<DataType>, out: &mut File) -> Result<(), std::io::Error> {
    let mut mesg_name = String::new();
    let mut mesg_fns: BTreeMap<String, String> = BTreeMap::new();
    let mut add_close_brace = false;

    for row in sheet.rows().skip(1) {
        // begin new message function
        if !row[0].is_empty() {
            if add_close_brace {
                end_message_fn(&mesg_name, out)?;
            }

            if let Some(v) = row[0].get_string() {
                mesg_name = v.to_string();
                mesg_fns.insert(titlecase_string(v), format!("{}_message()", v));
                write!(out, "fn {}_message() -> MessageInfo {{\n", v)?;
                write!(out, "    let mut fields = HashMap::new();\n\n")?;
                add_close_brace = true;
            } else {
                panic!(format!("Message name must be a string row={:?}.", row));
            }
        } else {
            create_field_info_struct(row, out)?;
        }
    }
    end_message_fn(&mesg_name, out)?;
    create_mesg_num_to_mesg_info_fn(&mesg_fns, out)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile_fname = "vendor/FitSDK/Profile.xlsx";
    let field_types_fname = "src/profile/field_types.rs";
    let messages_fname = "src/profile/messages.rs";

    // only re-run if this file changes since the code we generate depends on it.
    println!("cargo:rerun-if-changed={}", &profile_fname);

    let mut excel: Xlsx<_> = open_workbook(&profile_fname)?;
    let mut file = File::create(&field_types_fname)?;
    write!(
        file,
        "/// Auto generated profile from FIT SDK Release: XXX\n\n"
    )?;
    if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet, &mut file)?;
    } else {
        panic!("Could not access workbook sheet 'Types'");
    }

    let mut file = File::create(&messages_fname)?;
    write!(
        file,
        "/// Auto generated profile from FIT SDK Release: XXX\n"
    )?;
    write!(file, "use std::collections::HashMap;\n")?;
    write!(file, "use super::{{MessageInfo, FieldDataType, FieldInfo}};\n")?;
    write!(file, "use super::field_types::*;\n\n")?;
    if let Some(Ok(sheet)) = excel.worksheet_range("Messages") {
        process_messages(sheet, &mut file)?;
    } else {
        panic!("Could not access workbook sheet 'Messages'");
    }

    // call rustfmt on the two generated files to cleanup auto-gen code
    Command::new("rustfmt")
        .arg(&field_types_fname)
        .status()
        .expect(&format!(
            "failed to execute rustfmt on {}",
            field_types_fname
        ));
    Command::new("rustfmt")
        .arg(&messages_fname)
        .status()
        .expect(&format!("failed to execute rustfmt on {}", messages_fname));

    Ok(())
}
