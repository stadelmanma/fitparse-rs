/// Defines the data structures needed to represent a parsed FIT file.
use crate::parser::Ast;
use crate::profile::apply_data_profile;
use chrono::{DateTime, Local};
use serde::Serialize;
use serde::ser::{Serialize as SerializeTrait, Serializer, SerializeSeq};
use std::collections::{BTreeMap};
use std::ops::Add;
use std::ops::AddAssign;

/// Defines a FIT file's contents
#[derive(Clone, Debug)]
pub struct FitFile {
    pub header: FitFileHeader,
    pub records: Vec<FitDataRecord>,
    pub crc: u16,
}

impl FitFile {
    /// convert the AST into a FitFile by applying the defined profile.
    pub fn from_ast(ast: Ast) -> Self {
        FitFile {
            header: ast.header,
            records: apply_data_profile(ast.records),
            crc: ast.crc,
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
            seq.serialize_element(&FitDataRecordSerde{kind: &msg.kind, fields: field_map})?;
        }
        seq.end()    }
}

/// Internal struct used to serialize a data record into a cleaner format
#[derive(Debug, Serialize)]
struct FitDataRecordSerde<'a> {
    pub kind: &'a str,
    pub fields: BTreeMap<&'a str, &'a DataField>
}

/// The file header provides information about the FIT File. The minimum size of the file header is
/// 12 bytes including protocol and profile version numbers, the amount of data contained in the
/// file and data type signature. The 12 byte header is considered legacy, using the 14 byte header
/// is preferred. The header size should always be decoded before attempting to interpret a FIT
/// file, Dynastream may extend the header as necessary. Computing the CRC is optional when using a
/// 14 byte file header, it is permissible to set it to 0x0000.
///
/// header_size = u8,
/// protocol_ver_enc = u8,
/// profile_ver_enc = u16
/// data_size = u32
/// literal ".FIT" = [u8; 4]
/// CRC = u16 (if the header_size is 14 bytes)
#[derive(Clone, Debug)]
pub struct FitFileHeader {
    pub header_size: u8,
    pub protocol_ver_enc: f32,
    pub profile_ver_enc: f32,
    pub data_size: u32,
    pub crc: Option<u16>,
}

/// Defines a set of data derived from a FIT Data message.
///
/// If a time offset is present the data message had a CompressedTimestamp header.
/// This allows for time information to be conveyed without the need for a full 4 byte timestamp
/// data field.
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecord {
    pub kind: String,
    pub fields: Vec<DataField>,
}

/// Describe arbitary data field within a FitDataRecord.
#[derive(Clone, Debug, Serialize)]
pub struct DataField {
    #[serde(skip)]
    pub name: String,
    pub units: String,
    #[serde(skip)]
    pub scale: f64,
    #[serde(skip)]
    pub offset: f64,
    pub value: DataFieldValue,
    #[serde(skip)]
    pub raw_value: DataFieldValue,
}

/// Contains arbitrary data in the defined format.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum DataFieldValue {
    Timestamp(DateTime<Local>),
    Byte(u8),
    Enum(u8),
    SInt8(i8),
    UInt8(u8),
    SInt16(i16),
    UInt16(u16),
    SInt32(i32),
    UInt32(u32),
    String(String),
    Float32(f32),
    Float64(f64),
    UInt8z(u8),
    UInt16z(u16),
    UInt32z(u32),
    SInt64(i64),
    UInt64(u64),
    UInt64z(u64),
    Array(Vec<Self>),
}

impl DataFieldValue {
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
