use crate::objects::DataFieldValue;
use std::collections::HashMap;

pub mod field_types;
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
    // I don't know what to do here it's either a field type enum variant or a base type,
    // maybe I need to enumerate all field Types but that sounds like a matching nightmare
    field_type: &'static str,
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

    /// Rescale value using the scale and offset, only numeric fields get rescaled
    pub fn rescale_value(&self, value: &DataFieldValue) -> DataFieldValue {
        match value {
            DataFieldValue::Enum(_) => value.clone(),
            DataFieldValue::SInt8(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt8(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::SInt16(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt16(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::SInt32(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt32(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::String(_) => value.clone(),
            DataFieldValue::Float32(val) =>  DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::Float64(val) => DataFieldValue::Float64(*val / self.scale - self.offset),
            DataFieldValue::UInt8z(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt16z(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt32z(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::Byte(_) => value.clone(),
            DataFieldValue::SInt64(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt64(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
            DataFieldValue::UInt64z(val) => DataFieldValue::Float64((*val as f64) / self.scale - self.offset),
        }
    }
}

// TODO how to handle subfields and reference fields?
