pub mod field_types;

/// Describes a single message pulled from the FIT profile.
pub struct MessageInfo {
    name: &'static str,
    fields: Vec<FieldInfo>
    // comment - I don't want to save it but I do want to display it in the output file
}

/// Describes a single field within a message pulled from the FIT profile
pub struct FieldInfo {
    name: &'static str,
    //type - I don't know what to do here it's either a field type enum variant or a base type,
    //       maybe I need to enumerate all field Types but that sounds like a matching nightmare
    field_def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str
    // components - TODO
    // subfields - TODO
    // comment - I don't want to save it but I do want to display it in the output file
}


// TODO how to handle subfields and reference fields?
