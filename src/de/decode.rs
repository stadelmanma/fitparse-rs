//! Helper functions and structures needed to decode a FIT file using the defined profile.
use super::parser::{FitDataMessage, FitMessageHeader};
use crate::error::{ErrorKind, Result};
use crate::profile::{FieldInfo, MessageInfo};
use crate::{FieldValue, FitDataRecord, Value};
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use std::collections::HashMap;
use std::convert::{From, TryInto};

/// Stores the timestamp offset from the FIT reference date in seconds
#[derive(Debug, Copy, Clone)]
enum TimestampField {
    Local(i64),
    Utc(i64),
}

impl TimestampField {
    /// Return the timestamp as an i64
    fn as_i64(&self) -> i64 {
        match self {
            Self::Local(value) => *value,
            Self::Utc(value) => *value,
        }
    }

    /// converts offset value into a proper timestamp
    fn to_date_time(self) -> DateTime<Local> {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date = NaiveDate::from_ymd(1989, 12, 31).and_hms(0, 0, 0);
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
        return Value::Timestamp(timestamp.to_date_time());
    }
}

/// Decodes a raw FitDataMessage using the defined profile. Additional logic is used to handle
/// values that need to accumlate across multiple messages as well as applying the
/// time offset to the current base timestamp.
pub struct Decoder {
    base_timestamp: TimestampField,
    accumulate_fields: HashMap<u32, Value>,
}

impl Decoder {
    /// Create a new accumulator
    pub fn new() -> Self {
        Decoder {
            base_timestamp: TimestampField::Utc(0),
            accumulate_fields: HashMap::new(),
        }
    }

    /// Decode a raw FIT data message by applying the defined profile
    pub fn decode_message(
        &mut self,
        header: FitMessageHeader,
        message: FitDataMessage,
    ) -> Result<FitDataRecord> {
        todo!("process raw data messages into data records");
        //Ok(FitDataRecord::new("todo".to_string()))
    }

    /// Increment the stored field value
    fn accumlate_value(&mut self, msg_num: u16, def_num: u8, value: Value) -> Result<Value> {
        // use macro to duplicate same type only addition logic
        macro_rules! only_add_like_values {
            ($key:ident, $val:ident, $stored_value:ident, $variant:ident) => {
                if let Value::$variant(other) = value {
                    let value = Value::$variant($val + other);
                    self.accumulate_fields.insert($key, value.clone());
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
        if let Some(stored_value) = self.accumulate_fields.get(&key) {
            match stored_value {
                Value::Timestamp(_) => Err(ErrorKind::ValueError(
                    "Cannot accumlate timestamp fields".to_string(),
                )
                .into()),
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
                Value::Array(_) => {
                    Err(ErrorKind::ValueError("Cannot accumlate array fields".to_string()).into())
                }
            }
        } else {
            self.accumulate_fields.insert(key, value.clone());
            Ok(value)
        }
    }

    /// Update the timestamp with a new offset and return the value
    fn update_timestamp(&mut self, offset: u8) -> Value {
        let offset: i64 = offset as i64;
        let mask: i64 = 31; // last 5 significant bits of value
        let mut value = offset + (self.base_timestamp.as_i64() & !mask);
        // account for rollover if needed
        if offset < (self.base_timestamp.as_i64() & mask) {
            value += 32;
        }

        // update stored value and return
        match self.base_timestamp {
            TimestampField::Local(_) => {
                self.base_timestamp = TimestampField::Local(value);
            }
            TimestampField::Utc(_) => {
                self.base_timestamp = TimestampField::Utc(value);
            }
        }

        Value::from(self.base_timestamp.clone())
    }
}

/// Fetch the information for a specific field, if the field contains subfields then
/// we use the data values provided to try and de-reference it and return the subfield
/// info instead
fn get_message_field<'a>(
    msg: &'a MessageInfo,
    key: u8,
    data_map: &HashMap<u8, Value>,
) -> Option<&'a FieldInfo> {
    if let Some(field) = msg.fields().get(&key) {
        // check against subfields
        for (num, val, sub_info) in field.subfields() {
            if let Some(v) = data_map.get(num) {
                if v.clone().try_into().map_or(false, |v: i64| v == *val) {
                    return Some(sub_info);
                }
            }
        }
        // fallback to initial field info as default return
        return Some(field);
    }
    None
}

/// Build a data field using the provided FIT profile information
fn data_field_with_info(field_info: &FieldInfo, value: Value) -> (String, FieldValue) {
    (
        field_info.name().to_string(),
        FieldValue::new(value, field_info.units().to_string()),
    )
}

/// Create an "unknown" field as a placeholder if we don't have any field information
fn unknown_field(field_def_num: u8, value: Value) -> (String, FieldValue) {
    (
        format!("unknown_field_{}", field_def_num),
        FieldValue::new(value, String::new()),
    )
}
