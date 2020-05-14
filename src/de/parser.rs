//! Helper functions and structures needed to deserialize a FIT file.
use nom::bytes::complete::{tag, take};
use nom::combinator::cond;
use nom::error::ErrorKind;
use nom::multi::count;
use nom::number::complete::{le_i8, le_u16, le_u32, le_u8};
use nom::number::Endianness;
use nom::sequence::tuple;
use nom::IResult;
use nom::{i16, i32, i64, u16, u32, u64};
use std::fmt::Display;

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
    /// Length of header in bytes, should be either 12 or 14
    header_size: u8,
    /// Protocol version number as provided in SDK
    protocol_ver_enc: f32,
    /// Profile version number as provided in SDK
    profile_ver_enc: f32,
    /// Length of the Data Records section in bytes
    data_size: u32,
    /// Contains the value of the CRC of Bytes 0 through 13 CRC MSB 11, or may be set to 0x0000.
    crc: Option<u16>,
}

impl FitFileHeader {
    /// Return the header size of the FIT file
    pub fn header_size(&self) -> u8 {
        self.header_size
    }

    /// Return the header size of the FIT file
    pub fn protocol_ver_enc(&self) -> f32 {
        self.protocol_ver_enc
    }

    /// Return the header size of the FIT file
    pub fn profile_ver_enc(&self) -> f32 {
        self.profile_ver_enc
    }

    /// Return the header size of the FIT file
    pub fn data_size(&self) -> u32 {
        self.data_size
    }
}

/// Type of FIT message being read as specified by the header byte
#[derive(Clone, Copy, Debug)]
pub enum FitMessageType {
    Data,
    Definition,
}

/// FIT message headers are a single byte long and come in two forms, Normal and CompressedTimestamp.
///
/// The value of the bits inside is different for the two message header types but for simplicity
/// we treat them the same here and make the time_offset optionnal.
#[derive(Clone, Debug)]
pub struct FitMessageHeader {
    contains_developer_data: bool,
    local_message_type: u8,
    message_type: FitMessageType,
    time_offset: Option<u8>,
}

/// The definition message is used to create an association between the local message type
/// contained in the record header, and a Global Message Number (mesg_num) that relates to the
/// global FIT message. Although 1 byte is available for the number of fields and 1 byte is
/// available for the field size, no single message may be defined that is larger than 255 bytes.
#[derive(Clone, Debug)]
pub struct FitDefinitionMessage {
    byte_order: Endianness,
    global_message_number: u16,
    number_of_fields: u8,
    field_definitions: Vec<FieldDefinition>,
    number_of_developer_fields: u8,
    developer_field_definitions: Vec<DeveloperFieldDefinition>,
}

/// The Field Definition bytes are used to specify which FIT fields of the global FIT message are to
/// be included in the upcoming data message in this instance. Any subsequent data messages of a
/// particular local message type are considered to be using the format described by the definition
/// message of matching local message type. All FIT messages and their respective FIT fields are
/// listed in the global FIT profile. Each Field Definition consists of 3 bytes.
#[derive(Clone, Debug)]
pub struct FieldDefinition {
    field_definition_number: u8, //  could possibly be an enum (ie. field_type) but this is per-message type
    size: u8, // which might make things messy (i.e. umpteen different enums of enums)
    base_type: BaseType,
}

/// Developer data fields allow for files to define the meaning of data without requiring changes to
/// the FIT profile being used. Rather than having information like Field Name, Units, and Base Type
/// encoded into the profile this information is included in 2 special global messages that act as
/// meta-data for the decode process. The developer data field description is used to map data
/// within a data message to the appropriate meta-data.
#[derive(Clone, Debug)]
pub struct DeveloperFieldDefinition {
    field_number: u8,
    size: u8,
    developer_data_index: u8,
}

/// Base types defined by the FIT protocol. The "z" variants have a different invalid value
/// than the versions without the suffix.
#[derive(Clone, Copy, Debug)]
pub enum BaseType {
    Enum = 0x00,
    SInt8 = 0x01,
    UInt8 = 0x02,
    SInt16 = 0x83,
    UInt16 = 0x84,
    SInt32 = 0x85,
    UInt32 = 0x86,
    String = 0x07,
    Float32 = 0x88,
    Float64 = 0x89,
    UInt8z = 0x0A,
    UInt16z = 0x8B,
    UInt32z = 0x8C,
    Byte = 0x0D,
    SInt64 = 0x8E,
    UInt64 = 0x8F,
    UInt64z = 0x90,
}

impl BaseType {
    /// Check the value of the last 5 bits to determine the base type.
    ///
    /// Bits 5 and 6 are reserved so we don't check them and to avoid issues it's easier to
    /// simply zero them out, 0x9f == 159 == 0b10011111. When the type can't be determined we default
    /// to a byte vector.
    fn from_u8(base_type_field: u8) -> Self {
        match base_type_field & 0x9f {
            0x00 => BaseType::Enum,
            0x01 => BaseType::SInt8,
            0x02 => BaseType::UInt8,
            0x83 => BaseType::SInt16,
            0x84 => BaseType::UInt16,
            0x85 => BaseType::SInt32,
            0x86 => BaseType::UInt32,
            0x07 => BaseType::String,
            0x88 => BaseType::Float32,
            0x89 => BaseType::Float64,
            0x0A => BaseType::UInt8z,
            0x8B => BaseType::UInt16z,
            0x8C => BaseType::UInt32z,
            0x8E => BaseType::SInt64,
            0x8F => BaseType::UInt64,
            0x90 => BaseType::UInt64z,
            0x0D => BaseType::Byte,
            _ => BaseType::Byte,
        }
    }
}

/// Parse the FIT file header
pub fn fit_file_header(input: &[u8]) -> IResult<&[u8], FitFileHeader> {
    let (input, (header_size, proto, prof, data_size)) =
        tuple((le_u8, le_u8, le_u16, le_u32))(input)?;
    let (input, _) = tag(".FIT")(input)?;
    let (input, crc) = cond(header_size - 12 >= 2, le_u16)(input)?;
    let protocol_ver_enc = split_decimal_to_float(proto >> 4, proto & ((1 << 4) - 1));
    let profile_ver_enc = split_decimal_to_float(prof / 100, prof % 100);

    Ok((
        input,
        FitFileHeader {
            header_size,
            protocol_ver_enc,
            profile_ver_enc,
            data_size,
            crc,
        },
    ))
}

/// Convert a split decimal style value with fix precision into a single floating point value
///
/// This function should never fail as long as integer values are passed in as the arguments
fn split_decimal_to_float<T: Display>(left: T, right: T) -> f32 {
    format!("{}.{}", left, right).parse().unwrap()
}
