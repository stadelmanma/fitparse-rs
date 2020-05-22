//! Defines the FIT profile used to convert an AST into final values
use std::collections::HashMap;

pub mod field_types;
pub use field_types::{get_field_variant_as_string, FieldDataType, MesgNum};

pub mod messages;

/// Describes a single message pulled from the FIT profile.
#[derive(Clone, Debug)]
pub struct MessageInfo {
    name: &'static str,
    global_message_number: u16,
    fields: HashMap<u8, FieldInfo>,
}

impl MessageInfo {
    /// Return message name as defined in FIT profile
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Return global message number as defined in FIT profile
    pub fn global_message_number(&self) -> u16 {
        self.global_message_number
    }

    /// Return reference to defined set of fields
    pub fn fields(&self) -> &HashMap<u8, FieldInfo> {
        &self.fields
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
    accumulate: bool,
    subfields: Vec<(u8, i64, FieldInfo)>,
    components: Vec<ComponentFieldInfo>,
}

impl FieldInfo {
    /// Get field type
    pub fn field_type(&self) -> FieldDataType {
        self.field_type
    }

    /// Get definition number as defined in the FIT profile
    pub fn def_number(&self) -> u8 {
        self.def_number
    }

    /// Get scale used to convert field value into a float
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Get offset to shift field value by
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Get units of field
    pub fn units(&self) -> &'static str {
        &self.units
    }

    /// Check whether or not this field is accumlated across multiple messages
    pub fn accumulate(&self) -> bool {
        self.accumulate
    }

    /// Get components of field if any
    pub fn components(&self) -> &[ComponentFieldInfo] {
        &self.components
    }

    /// Get subfields of field if any, these are stored as a tuple of values
    /// (referenced field defition num, expected referenced field value, subfield field info)
    pub fn subfields(&self) -> &[(u8, i64, FieldInfo)] {
        &self.subfields
    }
}

/// Describes a componet field within a larger field pulled from the FIT profile
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
    /// Destination definition number to use once the component is expanded
    pub fn dest_def_number(&self) -> u8 {
        self.dest_def_number
    }

    /// Get scale used to convert field value into a float
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Get offset to shift field value by
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Get units of field
    pub fn units(&self) -> &'static str {
        &self.units
    }

    /// Get bitmask needed for this field during expansion
    pub fn bits(&self) -> u8 {
        self.bits
    }

    /// Check whether or not this field is accumlated across multiple messages
    pub fn accumulate(&self) -> bool {
        self.accumulate
    }
}
