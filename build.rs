/// Process the FIT profile excel workook and generate a profile.rs file.
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

/// Output an enum variant to the field types output file
fn write_variant_line(out: &mut File, variant_name: &str, comment: Option<&str>) {
    if let Some(v) = comment {
        out.write(format!("    {}, // {}\n", variant_name, v).as_bytes())
            .unwrap();
    } else {
        out.write(format!("    {},\n", variant_name).as_bytes())
            .unwrap();
    }
}

/// Create an implementation to match enums to their numeric equivalent
fn generate_enum_impl(
    enum_name: &str,
    rust_type: &str,
    variant_map: &BTreeMap<i64, String>,
    out: &mut File,
) {
    write!(out, "impl {} {{\n", enum_name).unwrap();
    write!(
        out,
        "    pub fn from_{0}(value: {0}) -> {1} {{\n",
        rust_type, enum_name
    )
    .unwrap();
    out.write(b"        match value {\n").unwrap();
    for (enum_val, var_name) in variant_map.iter() {
        write!(
            out,
            "            {} => {}::{},\n",
            enum_val, enum_name, var_name
        )
        .unwrap();
    }
    write!(
        out,
        "            _ => {}::UnknownVariant(value)\n",
        enum_name
    )
    .unwrap();
    out.write(b"        }\n").unwrap();
    out.write(b"    }\n").unwrap();

    write!(out, "    pub fn as_{0}(&self) -> {0} {{\n", rust_type).unwrap();
    out.write(b"        match &self {\n").unwrap();
    for (enum_val, var_name) in variant_map.iter() {
        write!(
            out,
            "            {}::{} => {},\n",
            enum_name, var_name, enum_val
        )
        .unwrap();
    }
    write!(
        out,
        "            {}::UnknownVariant(value) => *value\n",
        enum_name
    )
    .unwrap();
    out.write(b"        }\n").unwrap();
    out.write(b"    }\n").unwrap();

    out.write(b"}\n\n").unwrap();
}

fn process_types(sheet: Range<DataType>, out: &mut File) {
    let mut rust_type = "";
    let mut enum_name = String::new();
    let mut add_close_brace = false;
    let mut variant_map = BTreeMap::new();

    for row in sheet.rows().skip(1) {
        // begin new enum
        if !row[0].is_empty() {
            if add_close_brace {
                write!(out, "    UnknownVariant({}),\n", rust_type).unwrap();
                out.write(b"}\n\n").unwrap();
                generate_enum_impl(&enum_name, &rust_type, &variant_map, out);
                variant_map.clear();
            }

            // extract enum name
            if let Some(v) = row[0].get_string() {
                enum_name = titlecase_string(v);
            } else {
                panic!(format!("Enum type name must be a string row={:?}.", row));
            }

            // extract base type and convert to it's rust equivalent
            if let Some(v) = row[1].get_string() {
                rust_type = base_type_to_rust_type(v);
            } else {
                panic!(format!("Base type name must be a string row={:?}.", row));
            }

            out.write(b"#[derive(Clone, Copy, Debug)]\n").unwrap();
            out.write(format!("pub enum {} {{\n", enum_name).as_bytes())
                .unwrap();
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
                write_variant_line(out, &variant_name, comment);
                variant_map.insert(val_str, variant_name);
            }
        }
    }
    // ouput info for final enum
    write!(out, "    UnknownVariant({}),\n", rust_type).unwrap();
    out.write(b"}\n\n").unwrap();
    generate_enum_impl(&enum_name, &rust_type, &variant_map, out);
}

fn main() {
    println!("cargo:rerun-if-changed=vendor/FitSDK/Profile.xlsx");

    let mut excel: Xlsx<_> = open_workbook("vendor/FitSDK/Profile.xlsx").unwrap();
    let mut file = File::create("src/profile/field_types.rs").unwrap();
    file.write(b"/// Auto generated profile from FIT SDK Release: XXX\n")
        .unwrap();

    if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet, &mut file);
    } else {
        panic!("Could not access workbook sheet Types");
    }
}
