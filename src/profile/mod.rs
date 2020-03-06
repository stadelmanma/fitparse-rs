use crate::objects::DataFieldValue;
use chrono::{Duration, TimeZone, NaiveDate, NaiveDateTime, Local};
use std::collections::HashMap;

pub mod field_types;
use field_types::{get_field_variant_as_string, FieldDataType};

pub mod messages;

/// Describes a single message pulled from the FIT profile.
#[derive(Clone, Debug)]
pub struct MessageInfo {
    name: &'static str,
    fields: HashMap<u8, FieldInfo>,
}

impl MessageInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn get_field(&self, key: u8) -> Option<&FieldInfo> {
        self.fields.get(&key)
    }
}

/// Describes a single field within a message pulled from the FIT profile
#[derive(Clone, Debug)]
pub struct FieldInfo {
    name: &'static str,
    field_type: FieldDataType,
    def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str, // TODO components and subfields
}

impl FieldInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn units(&self) -> &'static str {
        self.units
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// convert the value into a "output" form applying any scaling or enum conversions
    pub fn convert_value(&self, value: &DataFieldValue) -> DataFieldValue {
        // handle time types specially
        match self.field_type {
            FieldDataType::DateTime | FieldDataType::LocalDateTime => return self.parse_timestamp(value),
            _ => ()
        }

        // convert enum or rescale integer value into floating point
        if self.field_type.is_enum_type() {
            if let Some(val) = value.as_i64() {
                DataFieldValue::String(get_field_variant_as_string(self.field_type, val))
            } else {
                panic!(format!(
                    "Cannot convert non-integer data type to enum variant - {:?}",
                    value
                ));
            }
        } else if self.scale != 1.0 || self.offset != 0.0 {
            if let Some(val) = value.as_f64() {
                self.rescale_value(val)
            } else {
                panic!(format!(
                    "Cannot convert non-numeric data type to float value - {:?}",
                    value
                ));
            }
        } else {
            value.clone()
        }
    }

    /// Rescale value using the scale and offset into a floating point number
    fn rescale_value(&self, value: f64) -> DataFieldValue {
        DataFieldValue::Float64(value / self.scale - self.offset)
    }

    /// converts value into a proper timestamp
    fn parse_timestamp(&self, value: &DataFieldValue) -> DataFieldValue {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date: NaiveDateTime = NaiveDate::from_ymd(1989, 12, 31).and_hms(0, 0, 0);
        let sec_since: Duration;
        if let Some(val) = value.as_i64() {
            sec_since = Duration::seconds(val);
        }
        else {
            // return raw value as fallback if we can't get seconds as int
            return value.clone();
        }

        // process convert ref date to UTC/local TZ and add Duration, we want to return a local
        // timestamp with TZ info
        let ref_date = match self.field_type {
            FieldDataType::DateTime => TimeZone::from_utc_datetime(&Local, &ref_date),
            FieldDataType::LocalDateTime => TimeZone::from_local_datetime(&Local, &ref_date).unwrap(),
            _ => panic!("Invalid field type in self.parse_timestamp!")
        };

        DataFieldValue::Timestamp(ref_date + sec_since)
    }
}

// TODO how to handle subfields and reference fields?
// TODO what about array types?
