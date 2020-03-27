use crate::objects::DataFieldValue;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, TimeZone};
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

    /// Fetch the information for a specific field, if the field contains subfields then
    /// we use the data values provided to try and de-reference it and return the subfield
    /// info instead
    pub fn get_field(&self, key: u8, data_map: &HashMap<u8, DataFieldValue>) -> Option<&FieldInfo> {
        if let Some(field) = self.fields.get(&key) {
            // check against subfields
            for (num, val, sub_info) in &field.subfields {
                if let Some(v) = data_map.get(num) {
                    if v.as_i64().map_or(false, |v| v == *val) {
                        return Some(sub_info);
                    }
                }
            }
            // fallback to initial field info as default return
            return Some(field);
        }
        None
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
    units: &'static str,
    subfields: Vec<(u8, i64, FieldInfo)>, // ref_def_num, ref_value, subfield_info
    components: Vec<ComponentFieldInfo>,
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

    pub fn subfields(&self) -> &[(u8, i64, FieldInfo)] {
        &self.subfields
    }

    pub fn components(&self) -> &[ComponentFieldInfo] {
        &self.components
    }

    pub fn expand_components(&self, value: &DataFieldValue) -> HashMap<u8, DataFieldValue> {
        // extract out each field by masking specific bits, spanning 1 or more bytes
        let bit_mask = [1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8, 128u8];
        let mut bytes = value.to_ne_bytes().into_iter();
        let mut data_map = HashMap::new();
        let mut byte = bytes.next().unwrap_or(0);
        let mut bit_pos = 0;
        for comp_fld in &self.components {
            let mut tmp: u64 = 0;
            for pos in 0..comp_fld.bits {
                tmp |= (((byte & bit_mask[bit_pos]) >> bit_pos) as u64) << pos;
                if bit_pos == 7 {
                    byte = bytes.next().unwrap_or(0);
                    bit_pos = 0;
                }
                else {
                    bit_pos += 1;
                }
            }
            data_map.insert(comp_fld.dest_def_number, DataFieldValue::UInt64(tmp));
        }

        data_map
    }

    /// convert the value into a "output" form applying any scaling or enum conversions
    pub fn convert_value(&self, value: &DataFieldValue) -> DataFieldValue {
        // for array types just map and return
        if let DataFieldValue::Array(vals) = value {
            return DataFieldValue::Array(vals.iter().map(|v| self.convert_value(v)).collect());
        }

        // handle time types specially
        match self.field_type {
            FieldDataType::DateTime | FieldDataType::LocalDateTime => {
                return self.parse_timestamp(value)
            }
            _ => (),
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
        } else {
            // return raw value as fallback if we can't get seconds as int
            return value.clone();
        }

        // process convert ref date to UTC/local TZ and add Duration, we want to return a local
        // timestamp with TZ info
        let ref_date = match self.field_type {
            FieldDataType::DateTime => TimeZone::from_utc_datetime(&Local, &ref_date),
            FieldDataType::LocalDateTime => {
                TimeZone::from_local_datetime(&Local, &ref_date).unwrap()
            }
            _ => panic!("Invalid field type in self.parse_timestamp!"),
        };

        DataFieldValue::Timestamp(ref_date + sec_since)
    }
}

/// Describes a componet field within a largest field pulled from the FIT profile
#[derive(Clone, Debug)]
pub struct ComponentFieldInfo {
    dest_def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str,
    bits: u8,
    accumulate: bool,
}

impl ComponentFieldInfo {
    pub fn dest_def_number(&self) -> u8 {
        self.dest_def_number
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn units(&self) -> &str {
        self.units
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn accumulate(&self) -> bool {
        self.accumulate
    }
}
