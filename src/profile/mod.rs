use crate::objects::DataFieldValue;
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
}

// TODO how to handle subfields and reference fields?
