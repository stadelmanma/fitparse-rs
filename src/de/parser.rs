//! Helper functions and structures needed to deserialize a FIT file.
use crate::Value;
use nom::bytes::complete::{tag, take};
use nom::combinator::cond;
use nom::error::ErrorKind;
use nom::multi::count;
use nom::number::complete::{le_i8, le_u16, le_u32, le_u8};
use nom::number::Endianness;
use nom::sequence::tuple;
use nom::IResult;
use nom::{i16, i32, i64, u16, u32, u64};
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Display;

/// Define an is_valid function needed for parsing here, this function is not needed for normal use
impl Value {
    fn is_valid(&self) -> bool {
        match self {
            Value::Enum(val) => *val != 0xFF,
            Value::SInt8(val) => *val != 0x7F,
            Value::UInt8(val) => *val != 0xFF,
            Value::SInt16(val) => *val != 0x7FFF,
            Value::UInt16(val) => *val != 0xFFFF,
            Value::SInt32(val) => *val != 0x7FFFFFFF,
            Value::UInt32(val) => *val != 0xFFFFFFFF,
            Value::String(val) => !val.contains("\0"),
            Value::Timestamp(_) => true, // timestamps are always valid
            Value::Float32(val) => val.is_finite(),
            Value::Float64(val) => val.is_finite(),
            Value::UInt8z(val) => *val != 0x0,
            Value::UInt16z(val) => *val != 0x0,
            Value::UInt32z(val) => *val != 0x0,
            Value::Byte(val) => *val != 0xFF,
            Value::SInt64(val) => *val != 0x7FFFFFFFFFFFFFFF,
            Value::UInt64(val) => *val != 0xFFFFFFFFFFFFFFFF,
            Value::UInt64z(val) => *val != 0x0,
            Value::Array(vals) => !vals.is_empty() && vals.iter().all(|v| v.is_valid()),
        }
    }
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

impl FitMessageHeader {
    pub fn contains_developer_data(&self) -> bool {
        self.contains_developer_data
    }

    pub fn local_message_type(&self) -> u8 {
        self.local_message_type
    }

    pub fn message_type(&self) -> FitMessageType {
        self.message_type
    }

    pub fn time_offset(&self) -> Option<u8> {
        self.time_offset
    }
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

/// Stores a vector of raw fields described by the preceding Definition message, a Definition message
/// must come before any Data message. The data here will be transfomed into a FitDataRecord using
/// the information from it's defintion message and the MessageInfo struct from the FIT profile
#[derive(Clone, Debug)]
pub struct FitDataMessage {
    global_message_number: u16,
    fields: HashMap<u8, Option<Value>>,
    developer_fields: Vec<Option<Value>>,
}

impl FitDataMessage {
    pub fn global_message_number(&self) -> u16 {
        self.global_message_number
    }

    pub fn get_fields(&self) -> &HashMap<u8, Option<Value>> {
        &self.fields
    }

    pub fn get_developer_fields(&self) -> &[Option<Value>] {
        &self.developer_fields
    }
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

impl From<u8> for BaseType {
    /// Check the value of the last 5 bits to determine the base type.
    ///
    /// Bits 5 and 6 are reserved so we don't check them and to avoid issues it's easier to
    /// simply zero them out, 0x9f == 159 == 0b10011111. When the type can't be determined we default
    /// to a byte vector.
    fn from(value: u8) -> Self {
        match value & 0x9f {
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

/// Parse the header of a single FIT message
pub fn message_header(input: &[u8]) -> IResult<&[u8], FitMessageHeader> {
    let (input, msg_header_byte) = le_u8(input)?;
    let contains_developer_data: bool;
    let local_message_type: u8;
    let message_type: FitMessageType;
    let time_offset: Option<u8>;

    if msg_header_byte & 0x80 == 0x80 {
        // compressed timestamp header
        contains_developer_data = false;
        local_message_type = (msg_header_byte >> 5) & 0x3; // bits 5-6
        message_type = FitMessageType::Data;
        time_offset = Some(msg_header_byte & 0x1F);
    } else if (msg_header_byte & 0x40) == 0x40 {
        contains_developer_data = msg_header_byte & 0x20 == 0x20;
        local_message_type = msg_header_byte & 0xF;
        message_type = FitMessageType::Definition;
        time_offset = None;
    } else {
        // developer data bit is reserved for Data messages and should be 0, so we don't check it
        contains_developer_data = false;
        local_message_type = msg_header_byte & 0xF;
        message_type = FitMessageType::Data;
        time_offset = None;
    }

    Ok((
        input,
        FitMessageHeader {
            contains_developer_data,
            local_message_type,
            message_type,
            time_offset,
        },
    ))
}

/// parse a definition message
pub fn definition_message<'a>(
    input: &'a[u8],
    header: &FitMessageHeader,
) -> IResult<&'a[u8], FitDefinitionMessage> {
    let (input, _) = take(1usize)(input)?;
    let (input, arch_byte) = le_u8(input)?;
    let byte_order = if arch_byte == 1 {
        Endianness::Big
    } else {
        Endianness::Little
    };
    let (input, global_message_number) = u16!(input, byte_order)?;
    let (input, number_of_fields) = le_u8(input)?;
    let (input, field_definitions) = count(field_definition, number_of_fields as usize)(input)?;
    let (input, number_of_developer_fields, developer_field_definitions) =
        if header.contains_developer_data() {
            let (input, nflds) = le_u8(input)?;
            let (input, dev_fld_defs) = count(developer_field_definition, nflds as usize)(input)?;
            (input, nflds, dev_fld_defs)
        } else {
            (input, 0, Vec::new())
        };

    Ok((
        input,
        FitDefinitionMessage {
            byte_order,
            global_message_number,
            number_of_fields,
            field_definitions,
            number_of_developer_fields,
            developer_field_definitions,
        },
    ))
}

fn field_definition(input: &[u8]) -> IResult<&[u8], FieldDefinition> {
    let (input, field_definition_number) = le_u8(input)?;
    let (input, size) = le_u8(input)?;
    let (input, base_type_field) = le_u8(input)?;

    Ok((
        input,
        FieldDefinition {
            field_definition_number,
            size,
            base_type: BaseType::from(base_type_field),
        },
    ))
}

fn developer_field_definition(input: &[u8]) -> IResult<&[u8], DeveloperFieldDefinition> {
    let (input, field_number) = le_u8(input)?;
    let (input, size) = le_u8(input)?;
    let (input, developer_data_index) = le_u8(input)?;

    Ok((
        input,
        DeveloperFieldDefinition {
            field_number,
            size,
            developer_data_index,
        },
    ))
}

/// Parse a data message
pub fn data_message<'a>(
    input: &'a [u8],
    def_mesg: &FitDefinitionMessage,
) -> IResult<&'a [u8], FitDataMessage> {
    let mut fields = HashMap::new();
    let mut input = input;
    for field_def in &def_mesg.field_definitions {
        let (i, value) = data_field_value(
            input,
            field_def.base_type,
            def_mesg.byte_order,
            field_def.size,
        )?;
        fields.insert(field_def.field_definition_number, value);
        input = i;
    }
    // store developer data as a byte array since we don't handle these fields in the final data
    // output yet
    let mut developer_fields = Vec::new();
    for field_def in &def_mesg.developer_field_definitions {
        let (i, value) = data_field_value(
            input,
            BaseType::Byte,
            def_mesg.byte_order, // TODO: I don't know how to handle this since byte swapping
            field_def.size,     // the whole thing as needed might not be valid if the field isn't
        )?; // a single integer value.
        developer_fields.push(value);
        input = i;
    }

    Ok((
        input,
        FitDataMessage {
            global_message_number: def_mesg.global_message_number,
            fields,
            developer_fields,
        },
    ))
}

macro_rules! parse_f32 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f32($i) } else { nom::number::complete::le_f32($i) } } ););
macro_rules! parse_f64 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f64($i) } else { nom::number::complete::le_f64($i) } } ););

/// Parse a single raw data value
fn data_field_value(
    input: &[u8],
    base_type: BaseType,
    byte_order: Endianness,
    size: u8,
) -> IResult<&[u8], Option<Value>> {
    let mut input = input;
    let mut bytes_consumed = 0;
    let mut values: Vec<Value> = Vec::new();

    while bytes_consumed < size {
        let (i, value) = match base_type {
            BaseType::Enum => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, Value::Enum(v)))?
            }
            BaseType::SInt8 => {
                bytes_consumed += 1;
                le_i8(input).map(|(i, v)| (i, Value::SInt8(v)))?
            }
            BaseType::UInt8 => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, Value::UInt8(v)))?
            }
            BaseType::SInt16 => {
                bytes_consumed += 2;
                i16!(input, byte_order).map(|(i, v)| (i, Value::SInt16(v)))?
            }
            BaseType::UInt16 => {
                bytes_consumed += 2;
                u16!(input, byte_order).map(|(i, v)| (i, Value::UInt16(v)))?
            }
            BaseType::SInt32 => {
                bytes_consumed += 4;
                i32!(input, byte_order).map(|(i, v)| (i, Value::SInt32(v)))?
            }
            BaseType::UInt32 => {
                bytes_consumed += 4;
                u32!(input, byte_order).map(|(i, v)| (i, Value::UInt32(v)))?
            }
            BaseType::String => {
                bytes_consumed += size;
                // consume the field as defined by it's size and then locate the first NUL byte
                // and ignore everything after it when converting to a string
                let (input, field_value) = take(size as usize)(input)?;
                let mut value = Vec::new();
                for char in field_value {
                    if *char == 0u8 {
                        break;
                    }
                    value.push(*char);
                }
                if let Ok(value) = String::from_utf8(value) {
                    (input, Value::String(value))
                } else {
                    return Ok((input, None));
                }
            }
            BaseType::Float32 => {
                bytes_consumed += 4;
                parse_f32!(input, byte_order).map(|(i, v)| (i, Value::Float32(v)))?
            }
            BaseType::Float64 => {
                bytes_consumed += 8;
                parse_f64!(input, byte_order).map(|(i, v)| (i, Value::Float64(v)))?
            }
            BaseType::UInt8z => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, Value::UInt8z(v)))?
            }
            BaseType::UInt16z => {
                bytes_consumed += 2;
                u16!(input, byte_order).map(|(i, v)| (i, Value::UInt16z(v)))?
            }
            BaseType::UInt32z => {
                bytes_consumed += 4;
                u32!(input, byte_order).map(|(i, v)| (i, Value::UInt32z(v)))?
            }
            BaseType::Byte => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, Value::UInt8(v)))?
            }
            BaseType::SInt64 => {
                bytes_consumed += 8;
                i64!(input, byte_order).map(|(i, v)| (i, Value::SInt64(v)))?
            }
            BaseType::UInt64 => {
                bytes_consumed += 8;
                u64!(input, byte_order).map(|(i, v)| (i, Value::UInt64(v)))?
            }
            BaseType::UInt64z => {
                bytes_consumed += 8;
                u64!(input, byte_order).map(|(i, v)| (i, Value::UInt64z(v)))?
            }
        };
        values.push(value);
        input = i;
    }

    // Return either a regular Value or an Array of them
    let value = if values.len() == 1 {
        values.swap_remove(0)
    } else {
        Value::Array(values)
    };

    // Only return "something" if it's in the valid range
    if value.is_valid() {
        Ok((input, Some(value)))
    } else {
        Ok((input, None))
    }
}
