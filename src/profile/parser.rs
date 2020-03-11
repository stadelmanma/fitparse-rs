use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct FitProfile {
    field_types: Vec<FieldTypeDefintion>
}

#[derive(Clone, Debug, Serialize)]
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
}

#[derive(Clone, Debug, Serialize)]
struct FieldTypeVariant {
    name: String,
    value: i64,
    comment: Option<String>,
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

pub fn parse_profile(profile_fname: &PathBuf) -> Result<FitProfile, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(&profile_fname)?;
    let field_types;

    // process Types sheet
    if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        field_types = process_types(sheet);
    } else {
        panic!("Could not access workbook sheet 'Types'");
    }

    // process Messages sheet
    // if let Some(Ok(sheet)) = excel.worksheet_range("Messages") {
    //     process_messages(sheet, &mut file)?;
    // } else {
    //     panic!("Could not access workbook sheet 'Messages'");
    // }

    Ok(FitProfile{field_types})
}
