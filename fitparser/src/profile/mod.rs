//! Defines the FIT profile used to convert raw parser output into final values that can be
//! interpreted without using the FIT profile.
use crate::de::DecodeOption;
use crate::error::{ErrorKind, Result};
use crate::{FitDataField, FitDataRecord, Value};
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::error::Error;
use std::f64::EPSILON;
use std::fmt;

pub mod field_types;
pub use field_types::{get_field_variant_as_string, FieldDataType, MesgNum};

pub mod decode;
pub use decode::VERSION;

pub mod messages;
pub use messages::Message;

impl Value {
    /// Convert the value into a vector of bytes
    fn to_ne_bytes(&self) -> Vec<u8> {
        match self {
            Value::Byte(val) => vec![*val],
            Value::Enum(val) => vec![*val],
            Value::SInt8(val) => vec![*val as u8],
            Value::UInt8(val) => vec![*val],
            Value::SInt16(val) => val.to_ne_bytes().to_vec(),
            Value::UInt16(val) => val.to_ne_bytes().to_vec(),
            Value::SInt32(val) => val.to_ne_bytes().to_vec(),
            Value::UInt32(val) => val.to_ne_bytes().to_vec(),
            Value::String(val) => val.as_bytes().to_vec(),
            Value::Timestamp(val) => val.timestamp().to_ne_bytes().to_vec(),
            Value::Float32(val) => val.to_ne_bytes().to_vec(),
            Value::Float64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt8z(val) => vec![*val],
            Value::UInt16z(val) => val.to_ne_bytes().to_vec(),
            Value::UInt32z(val) => val.to_ne_bytes().to_vec(),
            Value::SInt64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt64z(val) => val.to_ne_bytes().to_vec(),
            Value::Array(vals) => vals.iter().flat_map(|v| v.to_ne_bytes()).collect(),
        }
    }
}

/// Stores the timestamp offset from the FIT reference date in seconds
#[derive(Debug, Copy, Clone)]
pub enum TimestampField {
    /// TimestampField generated from Value of type FieldDataType::LocalDateTime
    Local(i64),
    /// TimestampField generated from Value of type FieldDataType::DateTime
    Utc(i64),
}

impl TimestampField {
    /// Return the timestamp as an i64
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::Local(value) => *value,
            Self::Utc(value) => *value,
        }
    }

    /// converts offset value into a proper timestamp
    fn to_date_time(self) -> DateTime<Local> {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date = NaiveDate::from_ymd_opt(1989, 12, 31)
            .map(|d: NaiveDate| d.and_hms_opt(0, 0, 0))
            .flatten()
            .unwrap();
        match self {
            Self::Local(value) => {
                TimeZone::from_local_datetime(&Local, &ref_date).unwrap() + Duration::seconds(value)
            }
            Self::Utc(value) => {
                TimeZone::from_utc_datetime(&Local, &ref_date) + Duration::seconds(value)
            }
        }
    }
}

impl From<TimestampField> for Value {
    fn from(timestamp: TimestampField) -> Value {
        Value::Timestamp(timestamp.to_date_time())
    }
}

/// Extracts a component of a defined size from the provided byte slice
/// Returns an updated slice, new starting offset and the extracted value.
fn extract_component(input: &[u8], mut offset: usize, nbits: usize) -> ((&[u8], usize), Value) {
    let bit_mask = [1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8, 128u8];
    let mut bytes = input.iter().copied();
    let mut idx = 0;
    let mut byte = bytes.next().unwrap_or(0);
    let mut acc: u64 = 0;

    for pos in 0..nbits {
        acc |= (((byte & bit_mask[offset]) >> offset) as u64) << pos;
        if offset == 7 {
            byte = bytes.next().unwrap_or(0);
            idx += 1;
            offset = 0;
        } else {
            offset += 1;
        }
    }
    if input.len() > idx {
        ((&input[idx..], offset), Value::UInt64(acc))
    } else {
        ((&[], offset), Value::UInt64(acc))
    }
}

/// Increment the stored field value
pub fn calculate_cumulative_value(
    accumulate_fields: &mut HashMap<u32, Value>,
    msg_num: u16,
    def_num: u8,
    value: Value,
) -> Result<Value> {
    // use macro to duplicate same type only addition logic
    macro_rules! only_add_like_values {
        ($key:ident, $val:ident, $stored_value:ident, $variant:ident) => {
            if let Value::$variant(other) = value {
                let value = Value::$variant($val + other);
                accumulate_fields.insert($key, value.clone());
                Ok(value)
            } else {
                Err(ErrorKind::ValueError(format!(
                    "Mixed type addition {} and {} cannot be combined",
                    $stored_value, value
                ))
                .into())
            }
        };
    }

    let key = (msg_num as u32) << 8 | def_num as u32;
    if let Some(stored_value) = accumulate_fields.get(&key) {
        match stored_value {
            Value::Timestamp(_) => {
                // TODO: fix this, probably done as u32 math but I probably need to keep timestamps
                // as u32 values until the "11th hour" so to speak to deal with them more easily.
                Err(ErrorKind::ValueError("Cannot accumlate timestamp fields".to_string()).into())
            }
            Value::Byte(val) => only_add_like_values!(key, val, stored_value, Byte),
            Value::Enum(_) => {
                Err(ErrorKind::ValueError("Cannot accumlate enum fields".to_string()).into())
            }
            Value::SInt8(val) => only_add_like_values!(key, val, stored_value, SInt8),
            Value::UInt8(val) => only_add_like_values!(key, val, stored_value, UInt8),
            Value::UInt8z(val) => only_add_like_values!(key, val, stored_value, UInt8z),
            Value::SInt16(val) => only_add_like_values!(key, val, stored_value, SInt16),
            Value::UInt16(val) => only_add_like_values!(key, val, stored_value, UInt16),
            Value::UInt16z(val) => only_add_like_values!(key, val, stored_value, UInt16z),
            Value::SInt32(val) => only_add_like_values!(key, val, stored_value, SInt32),
            Value::UInt32(val) => only_add_like_values!(key, val, stored_value, UInt32),
            Value::UInt32z(val) => only_add_like_values!(key, val, stored_value, UInt32z),
            Value::SInt64(val) => only_add_like_values!(key, val, stored_value, SInt64),
            Value::UInt64(val) => only_add_like_values!(key, val, stored_value, UInt64),
            Value::UInt64z(val) => only_add_like_values!(key, val, stored_value, UInt64z),
            Value::Float32(val) => only_add_like_values!(key, val, stored_value, Float32),
            Value::Float64(val) => only_add_like_values!(key, val, stored_value, Float64),
            Value::String(_) => {
                Err(ErrorKind::ValueError("Cannot accumlate string fields".to_string()).into())
            }
            // add arrays by value if they are equal in length, we'll have to find FIT files
            // to validate this behavior against the SDK. The Java SDK also always does this
            // with longs, which makes sense I supppose since floats are large enough to store
            // the explicit value so were always going to be working with integers for accumlated
            // fields.
            Value::Array(vals) => {
                if let Value::Array(other_vals) = value {
                    if vals.len() == other_vals.len() {
                        let mut new_vals = Vec::with_capacity(vals.len());
                        for (v1, v2) in vals.iter().zip(other_vals.iter()) {
                            let v1_i64: i64 = v1.try_into()?;
                            let v2_i64: i64 = v2.try_into()?;
                            new_vals.push(Value::SInt64(v1_i64 + v2_i64));
                        }
                        accumulate_fields.insert(key, Value::Array(new_vals.clone()));
                        Ok(Value::Array(new_vals))
                    } else {
                        Err(ErrorKind::ValueError(format!(
                            "Array lengths differ, {} != {}",
                            vals.len(),
                            other_vals.len()
                        ))
                        .into())
                    }
                } else {
                    Err(ErrorKind::ValueError(format!(
                        "Mixed type addition {} and {} cannot be combined",
                        stored_value, value
                    ))
                    .into())
                }
            }
        }
    } else {
        accumulate_fields.insert(key, value.clone());
        Ok(value)
    }
}

/// Build a data field using the provided FIT profile information
#[allow(clippy::too_many_arguments)]
pub fn data_field_with_info(
    def_number: u8,
    name: &str,
    data_type: FieldDataType,
    scale: f64,
    offset: f64,
    units: &str,
    value: Value,
    options: &HashSet<DecodeOption>,
) -> Result<FitDataField> {
    let value = convert_value(data_type, scale, offset, value, options)?;
    Ok(FitDataField::new(
        name.to_string(),
        def_number,
        value,
        units.to_string(),
    ))
}

/// Create an "unknown" field as a placeholder if we don't have any field information
pub fn unknown_field(field_def_num: u8, value: Value) -> FitDataField {
    FitDataField::new(
        format!("unknown_field_{}", field_def_num),
        field_def_num,
        value,
        String::new(),
    )
}

/// Applies any necessary value conversions based on the field specification
fn convert_value(
    field_type: FieldDataType,
    scale: f64,
    offset: f64,
    value: Value,
    options: &HashSet<DecodeOption>,
) -> Result<Value> {
    // for array types return inner vector unmodified
    if let Value::Array(vals) = value {
        let vals: Result<Vec<Value>> = vals
            .into_iter()
            .map(|v| apply_scale_and_offset(v, scale, offset))
            .collect();
        return vals.map(Value::Array);
    }

    // handle time types specially, if for some reason I can't convert to an integer we will
    // just dump the reference timestamp by passing it a 0
    match field_type {
        FieldDataType::DateTime => {
            return Ok(Value::from(TimestampField::Utc(
                value.try_into().unwrap_or(0),
            )));
        }
        FieldDataType::LocalDateTime => {
            return Ok(Value::from(TimestampField::Local(
                value.try_into().unwrap_or(0),
            )));
        }
        _ => (),
    }

    // convert enum or rescale integer value into floating point
    if field_type.is_enum_type() {
        let val: i64 = value.try_into()?;
        if options.contains(&DecodeOption::ReturnNumericEnumValues) {
            Ok(Value::SInt64(val))
        } else if field_type.is_named_variant(val) {
            Ok(Value::String(get_field_variant_as_string(field_type, val)))
        } else {
            Ok(Value::SInt64(val))
        }
    } else {
        apply_scale_and_offset(value, scale, offset)
    }
}

fn apply_scale_and_offset(value: Value, scale: f64, offset: f64) -> Result<Value> {
    if ((scale - 1.0).abs() > EPSILON) || ((offset - 0.0).abs() > EPSILON) {
        let val: f64 = value.try_into()?;
        Ok(Value::Float64(val / scale - offset))
    } else {
        Ok(value)
    }
}

/// A message type.
pub trait FitMessage {
    /// The name of this message type.
    const NAME: &'static str;
    /// The message kind used in [`FitDataRecord`][].
    const KIND: MesgNum;

    /// Parse a message from a [`FitDataRecord`][] using the default options.
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError>
    where
        Self: Sized,
    {
        Self::parse_with_options(record, Default::default())
    }

    /// Parse a message from a [`FitDataRecord`][] using the given options.
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError>
    where
        Self: Sized;
}

/// Options for [`FitMessage::parse_with_options`][].
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct MessageParseOptions {
    /// Don’t return an error if the record contains an unexpected field.
    pub ignore_unexpected_fields: bool,
}

/// An error when parsing a [`FitMessage`][] implementation from a [`FitDataRecord`][].
#[derive(Debug)]
pub enum TryFromRecordError {
    /// The record contains an unexpected field.
    UnexpectedField {
        /// The name of the unexpected field.
        name: String,
        /// The number of the unexpected field.
        number: u8,
    },
    /// The record has an unexpected message kind.
    UnexpectedMessageKind {
        /// The expected message kind.
        expected: MesgNum,
        /// The actual message kind.
        actual: MesgNum,
    },
    /// The record has an unsupported message kind.
    UnsupportedMessageKind(MesgNum),
}

impl TryFromRecordError {
    fn unexpected_field(field: &FitDataField) -> Self {
        Self::UnexpectedField {
            name: field.name().to_owned(),
            number: field.number(),
        }
    }

    fn unexpected_message_kind<M: FitMessage>(record: &FitDataRecord) -> Self {
        Self::UnexpectedMessageKind {
            expected: M::KIND,
            actual: record.kind(),
        }
    }
}

impl fmt::Display for TryFromRecordError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedField { name, number } => write!(
                fmt,
                "the record contains an unexpected field: {name} ({number})"
            ),
            Self::UnexpectedMessageKind { expected, actual } => write!(
                fmt,
                "the record has an unexpected message kind: expected {expected}, got {actual}"
            ),
            Self::UnsupportedMessageKind(kind) => {
                write!(fmt, "the record has an unsupported message kind: {kind}")
            }
        }
    }
}

impl Error for TryFromRecordError {}
