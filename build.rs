/// Process the FIT profile excel workook and generate a profile.rs file.
///
/// The entries in the "Types" sheet name and describe the possible values for any Enum field types.
/// The entries in the "Messages" sheet name and number the fields within a given message and specify
/// the type of a field which can be an enum or a primitive. Primitive types may also have units,
/// scale and offset defined which describe how to transform the primitive integer type into a floating
/// point value.
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

fn titlecase_string(value: &str) -> String {
    let mut words: Vec<String> = value.split('_').map(|v| v.to_string()).collect();

    for word in &mut words {
        if let Some(l) = word.get_mut(0..1) {
            l.make_ascii_uppercase();
        }
    }

    String::from(words.join(""))
}

fn write_variant_line<T: Display>(
    out: &mut File,
    variant_name: String,
    variant_value: T,
    comment: Option<&str>,
) {
    if let Some(v) = comment {
        out.write(format!("  {} = {}, // {}\n", variant_name, variant_value, v).as_bytes())
            .unwrap();
    } else {
        out.write(format!("  {} = {},\n", variant_name, variant_value).as_bytes())
            .unwrap();
    }
}

fn process_types(sheet: Range<DataType>, out: &mut File) {
    let mut enum_name: String;
    let mut add_close_brace = false;

    for row in sheet.rows().skip(1) {
        // begin new enum
        if !row[0].is_empty() {
            if add_close_brace {
                out.write(b"}\n\n").unwrap();
            }

            // extract enum name
            if let Some(v) = row[0].get_string() {
                enum_name = titlecase_string(v);
            } else {
                panic!(format!("Enum type name must be a string row={:?}.", row));
            }

            out.write(format!("pub enum {} {{\n", enum_name).as_bytes())
                .unwrap();
            add_close_brace = true;
        }
        // add enum variant
        if !row[2].is_empty() {
            let variant_name: String;

            // extract enum name and possible comment
            if let Some(v) = row[2].get_string() {
                variant_name = titlecase_string(v);
            } else {
                panic!(format!("Enum variant name must be a string row={:?}.", row));
            }
            let comment = row[4].get_string();

            // handle mix of numeric and hex string data values
            match &row[3] {
                DataType::Float(v) => {
                    write_variant_line(out, variant_name, v, comment);
                }
                DataType::Int(v) => {
                    write_variant_line(out, variant_name, v, comment);
                }
                DataType::String(v) => {
                    write_variant_line(out, variant_name, v, comment);
                }
                _ => {
                    panic!(format!(
                        "Unsupported enum variant value data type row={:?}.",
                        row
                    ));
                }
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=vendor/FitSDK/temp.txt");

    let mut excel: Xlsx<_> = open_workbook("vendor/FitSDK/Profile.xlsx").unwrap();
    let mut file = File::create("src/profile.rs").unwrap();
    file.write(b"/// Auto generated profile from FIT SDK Release: XXX\n")
        .unwrap();

    if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet, &mut file);
    } else {
        panic!("Could not access workbook sheet Types");
    }
}
