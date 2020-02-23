/// Defines the data structures needed to represent a parsed FIT file.
use nom::number::Endianness;

/// Defines a FIT file's contents
#[derive(Clone, Debug)]
pub struct FitFile {
    pub header: FitFileHeader,
    pub messages: Vec<FitDataRecord>,
    pub crc: u16,
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

#[derive(Clone, Debug)]
pub struct FitDataRecord {
    pub header: FitMessageHeader,
    pub message: FitMessage,
}

/// Fit messages contain data or a definition of the data in the next message(s)
#[derive(Clone, Debug)]
pub enum FitMessageType {
    Data,
    Definition,
}

/// FIT message headers are a single byte long and come in two forms.
///
/// The value of the bits inside is different for the two message header types
#[derive(Clone, Debug)]
pub enum FitMessageHeader {
    Normal {
        message_type: FitMessageType,
        contains_developer_data: bool, //This might be better as a separate enum variant
        local_message_type: u8,
    },
    CompressedTimestamp {
        local_message_type: u8,
        time_offset: u8,
    },
}

/// The value of the bits inside is different for the two message header types
#[derive(Clone, Debug)]
pub enum FitMessage {
    /// Stores a vector of fields described by the preceding Definition message, a Definition message
    /// must come before any Data message.
    Data { data_fields: Vec<DataField> },
    /// The definition message is used to create an association between the local message type
    /// contained in the record header, and a Global Message Number (mesg_num) that relates to the
    /// global FIT message. Although 1 byte is available for the number of fields and 1 byte is
    /// available for the field size, no single message may be defined that is larger than 255 bytes.
    Definition {
        byte_order: Endianness,
        global_message_number: u16,
        number_of_fields: u8,
        field_definitions: Vec<FieldDefinition>,
        number_of_developer_fields: u8,
        developer_field_definitions: Vec<DeveloperFieldDefinition>,
    },
}

/// The Field Definition bytes are used to specify which FIT fields of the global FIT message are to
/// be included in the upcoming data message in this instance. Any subsequent data messages of a
/// particular local message type are considered to be using the format described by the definition
/// message of matching local message type. All FIT messages and their respective FIT fields are
/// listed in the global FIT profile. Each Field Definition consists of 3 bytes.
#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub field_definition_number: u8, //  could possibly be an enum (ie. field_type) but this is per-message type
    pub size: u8, // which might make things messy (i.e. umpteen different enums of enums)
    pub base_type: u8, // probably needs to be an enum of types, I think this is fairly limited
}

/// Developer data fields allow for files to define the meaning of data without requiring changes to
/// the FIT profile being used. Rather than having information like Field Name, Units, and Base Type
/// encoded into the profile this information is included in 2 special global messages that act as
/// meta-data for the decode process. The developer data field description is used to map data
/// within a data message to the appropriate meta-data.
#[derive(Clone, Debug)]
pub struct DeveloperFieldDefinition {
    pub field_number: u8,
    pub size: u8,
    pub developer_data_index: u8,
}

/// Developer data ID messages are used to uniquely identify developer data field sources, a FIT
/// file can contain data for up to 255 unique developers. These messages must occur before any
/// related field description messages.
#[derive(Clone, Debug)]
pub struct DeveloperDataIdMessage {
    pub application_id: [u8; 16],
    pub developer_data_index: u8,
}

/// Field description messages define the meaning of data within a dev field, a FIT file can
/// contain up to 255 unique fields per developer. These messages must occur in the file before
/// any related data is added.
#[derive(Clone, Debug)]
pub struct DeveloperFieldDescription {
    pub developer_data_index: u8,
    pub field_definition_number: u8,
    pub fit_base_type_id: u8,
    pub field_name: String, // max size 64 bytes
    pub units: String,      // max size 16 bytes
    pub native_field_num: u8,
}

/// Contains arbitrary data that needs converted to a value based on the base_type defined in the
/// Definition message describing the Data message.
#[derive(Clone, Debug)]
pub struct DataField {
    pub value: Vec<u8>,
}
