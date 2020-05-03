//! Defines the data structures needed to represent a parsed FIT file.
use crate::parser::Ast;
use crate::profile::apply_data_profile;
use chrono::{DateTime, Local};
use serde::ser::{Serialize as SerializeTrait, SerializeSeq, Serializer};
use serde::Serialize;
use std::collections::BTreeMap;
use std::ops::Add;
use std::ops::AddAssign;

/// Defines a FIT file's contents
#[derive(Clone, Debug)]
pub struct FitFile {
    /// Array of raw FIT data messages
    pub records: Vec<FitDataRecord>,
}

impl FitFile {
    /// convert the AST into a FitFile by applying the defined profile.
    pub fn from_ast(ast: Ast) -> Self {
        FitFile {
            records: apply_data_profile(ast.records),
        }
    }
}

impl SerializeTrait for FitFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.records.len()))?;
        for msg in &self.records {
            let mut field_map: BTreeMap<&str, &DataField> = BTreeMap::new();
            for field in &msg.fields {
                field_map.insert(&field.name, field);
            }
            seq.serialize_element(&FitDataRecordSerde {
                kind: &msg.kind,
                fields: field_map,
            })?;
        }
        seq.end()
    }
}

/// Internal struct used to serialize a data record into a cleaner format
#[derive(Debug, Serialize)]
struct FitDataRecordSerde<'a> {
    kind: &'a str,
    fields: BTreeMap<&'a str, &'a DataField>,
}

/// Defines a set of data derived from a FIT Data message.
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecord {
    /// The kind of message the data came from, the FIT profile defines several message kinds
    pub kind: String,
    /// A vector containing all the fields present in this message
    pub fields: Vec<DataField>,
}

/// Describes arbitary data field within a FitDataRecord.
///
/// The FIT profile defines the name, units, scale and offset for each field. Scale and offset are
/// used to convert a float value stored as an integer back into a float. The `value` of the field
/// is the final quantity derived from the raw_value by applying any necessary type conversions.
#[derive(Clone, Debug, Serialize)]
pub struct DataField {
    /// name of the data field
    #[serde(skip)]
    pub name: String,
    /// units of the data field
    pub units: String,
    /// scale applied to the raw field, used when converting an integer value to floating point
    #[serde(skip)]
    pub scale: f64,
    /// offset applied to the raw field, used when converting an integer value to floating point
    #[serde(skip)]
    pub offset: f64,
    /// final converted value of the data field after applying any needed manipulations
    pub value: DataFieldValue,
    /// raw value read from the FIT file data source
    #[serde(skip)]
    pub raw_value: DataFieldValue,
}

/// Contains arbitrary data in the defined format. These types match the base types defined in the
/// FIT profile so while some fields have overlapping Rust types the invalid value differs.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum DataFieldValue {
    /// Timestamp field converted to the local timezone
    Timestamp(DateTime<Local>),
    /// Unsigned 8bit integer data
    Byte(u8),
    /// Unsigned 8bit integer that gets mapped to a FieldType enum
    Enum(u8),
    /// Signed 8bit integer data
    SInt8(i8),
    /// Unsigned 8bit integer data
    UInt8(u8),
    /// Signed 16bit integer data
    SInt16(i16),
    /// Unsigned 16bit integer data
    UInt16(u16),
    /// Signed 32bit integer data
    SInt32(i32),
    /// Unsigned 32bit integer data
    UInt32(u32),
    /// UTF-8 format string data
    String(String),
    /// 32bit floating point data
    Float32(f32),
    /// 64bit floating point data
    Float64(f64),
    /// Unsigned 8bit integer data where the invalid value is 0x0 instead of 0xFF
    UInt8z(u8),
    /// Unsigned 16bit integer data where the invalid value is 0x0 instead of 0xFFFF
    UInt16z(u16),
    /// Unsigned 16bit integer data where the invalid value is 0x0 instead of 0xFFFFFFFF
    UInt32z(u32),
    /// Signed 64bit integer data
    SInt64(i64),
    /// Unsigned 64bit integer data
    UInt64(u64),
    /// Unsigned 64bit integer data where the invalid value is 0x0 instead of 0xFFFFFFFFFFFFFFFF
    UInt64z(u64),
    /// Array of DataFieldValue, while this allows nested arrays and mixed types this is not possible
    /// in a properly formatted FIT file
    Array(Vec<Self>),
}

impl DataFieldValue {
    /// Check if the value contained is valid according to the FIT profile
    pub fn is_valid(&self) -> bool {
        match self {
            DataFieldValue::Enum(val) => *val != 0xFF,
            DataFieldValue::SInt8(val) => *val != 0x7F,
            DataFieldValue::UInt8(val) => *val != 0xFF,
            DataFieldValue::SInt16(val) => *val != 0x7FFF,
            DataFieldValue::UInt16(val) => *val != 0xFFFF,
            DataFieldValue::SInt32(val) => *val != 0x7FFFFFFF,
            DataFieldValue::UInt32(val) => *val != 0xFFFFFFFF,
            DataFieldValue::String(val) => !val.contains("\0"),
            DataFieldValue::Timestamp(_) => true, // timestamps are always valid
            DataFieldValue::Float32(val) => val.is_finite(),
            DataFieldValue::Float64(val) => val.is_finite(),
            DataFieldValue::UInt8z(val) => *val != 0x0,
            DataFieldValue::UInt16z(val) => *val != 0x0,
            DataFieldValue::UInt32z(val) => *val != 0x0,
            DataFieldValue::Byte(val) => *val != 0xFF,
            DataFieldValue::SInt64(val) => *val != 0x7FFFFFFFFFFFFFFF,
            DataFieldValue::UInt64(val) => *val != 0xFFFFFFFFFFFFFFFF,
            DataFieldValue::UInt64z(val) => *val != 0x0,
            DataFieldValue::Array(vals) => !vals.is_empty() && vals.iter().all(|v| v.is_valid()),
        }
    }

    /// Try and coerce the value into a floating point number
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataFieldValue::Byte(val) => Some(*val as f64),
            DataFieldValue::SInt8(val) => Some(*val as f64),
            DataFieldValue::UInt8(val) => Some(*val as f64),
            DataFieldValue::SInt16(val) => Some(*val as f64),
            DataFieldValue::UInt16(val) => Some(*val as f64),
            DataFieldValue::SInt32(val) => Some(*val as f64),
            DataFieldValue::UInt32(val) => Some(*val as f64),
            DataFieldValue::Float32(val) => Some(*val as f64),
            DataFieldValue::Float64(val) => Some(*val as f64),
            DataFieldValue::UInt8z(val) => Some(*val as f64),
            DataFieldValue::UInt16z(val) => Some(*val as f64),
            DataFieldValue::UInt32z(val) => Some(*val as f64),
            DataFieldValue::SInt64(val) => Some(*val as f64),
            DataFieldValue::UInt64(val) => Some(*val as f64),
            DataFieldValue::UInt64z(val) => Some(*val as f64),
            _ => None,
        }
    }

    /// Try and coerce the value into an integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DataFieldValue::Byte(val) => Some(*val as i64),
            DataFieldValue::Enum(val) => Some(*val as i64),
            DataFieldValue::SInt8(val) => Some(*val as i64),
            DataFieldValue::UInt8(val) => Some(*val as i64),
            DataFieldValue::SInt16(val) => Some(*val as i64),
            DataFieldValue::UInt16(val) => Some(*val as i64),
            DataFieldValue::SInt32(val) => Some(*val as i64),
            DataFieldValue::UInt32(val) => Some(*val as i64),
            DataFieldValue::Timestamp(val) => Some(val.timestamp()),
            DataFieldValue::Float32(val) => Some(*val as i64),
            DataFieldValue::Float64(val) => Some(*val as i64),
            DataFieldValue::UInt8z(val) => Some(*val as i64),
            DataFieldValue::UInt16z(val) => Some(*val as i64),
            DataFieldValue::UInt32z(val) => Some(*val as i64),
            DataFieldValue::SInt64(val) => Some(*val as i64),
            DataFieldValue::UInt64(val) => Some(*val as i64),
            DataFieldValue::UInt64z(val) => Some(*val as i64),
            _ => None,
        }
    }

    /// Convert the value into a vector of bytes
    pub fn to_ne_bytes(&self) -> Vec<u8> {
        match self {
            DataFieldValue::Byte(val) => vec![*val as u8],
            DataFieldValue::Enum(val) => vec![*val as u8],
            DataFieldValue::SInt8(val) => vec![*val as u8],
            DataFieldValue::UInt8(val) => vec![*val as u8],
            DataFieldValue::SInt16(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt16(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::SInt32(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt32(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::String(val) => val.as_bytes().to_vec(),
            DataFieldValue::Timestamp(val) => val.timestamp().to_ne_bytes().to_vec(),
            DataFieldValue::Float32(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::Float64(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt8z(val) => vec![*val as u8],
            DataFieldValue::UInt16z(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt32z(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::SInt64(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt64(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::UInt64z(val) => val.to_ne_bytes().to_vec(),
            DataFieldValue::Array(vals) => vals.iter().flat_map(|v| v.to_ne_bytes()).collect(),
        }
    }
}

impl Add for DataFieldValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            DataFieldValue::Byte(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::Byte(val + other as u8)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::Enum(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::Enum(val + other as u8)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::SInt8(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::SInt8(val + other as i8)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt8(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt8(val + other as u8)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::SInt16(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::SInt16(val + other as i16)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt16(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt16(val + other as u16)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::SInt32(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::SInt32(val + other as i32)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt32(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt32(val + other as u32)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::String(val) => {
                if let DataFieldValue::String(other) = other {
                    DataFieldValue::String(val + &other)
                } else {
                    panic!("Cannot add non-string to string");
                }
            }
            DataFieldValue::Timestamp(_) => panic!("Cannot add timestamps"),
            DataFieldValue::Float32(val) => {
                if let Some(other) = other.as_f64() {
                    DataFieldValue::Float32(val + other as f32)
                } else {
                    panic!("Cannot coerce value to float");
                }
            }
            DataFieldValue::Float64(val) => {
                if let Some(other) = other.as_f64() {
                    DataFieldValue::Float64(val + other)
                } else {
                    panic!("Cannot coerce value to float");
                }
            }
            DataFieldValue::UInt8z(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt8z(val + other as u8)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt16z(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt16z(val + other as u16)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt32z(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt32z(val + other as u32)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::SInt64(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::SInt64(val + other as i64)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt64(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt64(val + other as u64)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::UInt64z(val) => {
                if let Some(other) = other.as_i64() {
                    DataFieldValue::UInt64z(val + other as u64)
                } else {
                    panic!("Cannot coerce value to integer");
                }
            }
            DataFieldValue::Array(mut vals) => {
                if let DataFieldValue::Array(mut other_vals) = other {
                    if vals.len() > other_vals.len() {
                        for (v, o) in vals.iter_mut().zip(other_vals.into_iter()) {
                            *v += o;
                        }
                        DataFieldValue::Array(vals)
                    } else {
                        for (o, v) in other_vals.iter_mut().zip(vals.into_iter()) {
                            *o += v;
                        }
                        DataFieldValue::Array(other_vals)
                    }
                } else {
                    DataFieldValue::Array(vals.into_iter().map(|v| v + other.clone()).collect())
                }
            }
        }
    }
}

impl AddAssign for DataFieldValue {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}
