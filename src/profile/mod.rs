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

    pub fn rescale_value(&self, value: i64) -> f64 {
        (value as f64) * self.scale - self.offset
    }
}

// TODO how to handle subfields and reference fields?
