/// Parse FIT files
///
/// Logic largely based on: https://github.com/dtcooper/python-fitparse
///
/// Notes:
///   I'll want to eventually determine a way to check CRCs if it's not too messy
///   Apparently FIT files can be chained? If so I'll need to conditionally rerun the parser
///   Also profiles are configurable and vary on SDK release I think so I'll need a way to customize
///   that
use crate::objects::*;
use crate::profile::field_types::MesgNum;
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
use std::fmt::Display;

pub enum FitMessageType {
    Data,
    Definition,
}

/// FIT message headers are a single byte long and come in two forms, Normal and CompressedTimestamp.
///
/// The value of the bits inside is different for the two message header types but for simplicity
/// we treat them the same here and make the time_offset optionnal.
struct FitMessageHeader {
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
struct FitDefinitionMessage {
    byte_order: Endianness,
    global_message_number: MesgNum,
    number_of_fields: u8,
    field_definitions: Vec<FieldDefinition>,
    number_of_developer_fields: u8,
    developer_field_definitions: Vec<DeveloperFieldDefinition>,
}

/// Stores a vector of raw fields described by the preceding Definition message, a Definition message
/// must come before any Data message. The data here will be transfomed into a FitDataRecord using
/// the information from it's defintion message and the MessageInfo struct from the FIT profile
#[derive(Clone, Debug)]
struct FitDataMessage {
    fields: Vec<Option<DataFieldValue>>,
}

/// The Field Definition bytes are used to specify which FIT fields of the global FIT message are to
/// be included in the upcoming data message in this instance. Any subsequent data messages of a
/// particular local message type are considered to be using the format described by the definition
/// message of matching local message type. All FIT messages and their respective FIT fields are
/// listed in the global FIT profile. Each Field Definition consists of 3 bytes.
#[derive(Clone, Debug)]
struct FieldDefinition {
    pub field_definition_number: u8, //  could possibly be an enum (ie. field_type) but this is per-message type
    pub size: u8, // which might make things messy (i.e. umpteen different enums of enums)
    pub base_type: BaseType,
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

pub fn parse_file(input: &[u8]) -> IResult<&[u8], FitFile> {
    fit_file(input)
}

/// Parse a FIT file
fn fit_file(input: &[u8]) -> IResult<&[u8], FitFile> {
    let (input, header) = fit_file_header(input)?;
    let (input, records) = parse_records(input)?;
    let (input, crc) = le_u16(input)?;

    Ok((
        input,
        FitFile {
            header,
            records,
            crc,
        },
    ))
}

/// Parse the FIT file header
fn fit_file_header(input: &[u8]) -> IResult<&[u8], FitFileHeader> {
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

/// Parse each message inside the FIT file
///
/// All messages are returned in the order that they were written in but we use a local definition
/// message mapping to decode the data messages since local_message_type numbers can be redefined
/// throughout the file. This contiues until we reach the 2 byte CRC at the end of the input stream
fn parse_records(input: &[u8]) -> IResult<&[u8], Vec<FitDataRecord>> {
    let mut definitions: HashMap<u8, FitDefinitionMessage> = HashMap::new();
    let mut records = Vec::new();
    let mut input = input.clone();
    while input.len() > 2 {
        match fit_data_record(input.clone(), &mut definitions) {
            Err(e) => return Err(e),
            Ok((new_inp, rec)) => {
                if new_inp == input {
                    return Err(nom::Err::Error((input, ErrorKind::Many1)));
                }

                input = new_inp;
                records.push(rec);
            }
        }
    }

    Ok((input, records))
}

/// parse a single FIT data record which can define further fields or actaully cotain data itself
///
/// This function recurses if the message read was a defintion message since there should always be
/// data after a defintion message in a complete FIT file because all defintion messages must occur
/// before the data messages that use them are read. A defintion message at the end wouldn't serve
/// a purpose.
fn fit_data_record<'a>(
    input: &'a [u8],
    definitions: &mut HashMap<u8, FitDefinitionMessage>,
) -> IResult<&'a [u8], FitDataRecord> {
    let (input, header) = message_header(input)?;
    match &header.message_type {
        FitMessageType::Data => {
            if let Some(def_mesg) = definitions.get(&header.local_message_type) {
                let (input, data_mesg) = data_message(input, def_mesg)?;
                let fields = process_data_fields(data_mesg, def_mesg);
                Ok((
                    input,
                    FitDataRecord {
                        kind: def_mesg.global_message_number,
                        time_offset: header.time_offset,
                        fields,
                    },
                ))
            } else {
                Err(nom::Err::Failure((input, ErrorKind::Verify)))
            }
        }
        FitMessageType::Definition => {
            let (input, message) = definition_message(input, header.contains_developer_data)?;
            definitions.insert(header.local_message_type, message);
            fit_data_record(input.clone(), definitions)
        }
    }
}

/// Parse the header of a single FIT message
fn message_header(input: &[u8]) -> IResult<&[u8], FitMessageHeader> {
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
fn definition_message(
    input: &[u8],
    contains_developer_data: bool,
) -> IResult<&[u8], FitDefinitionMessage> {
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
        if contains_developer_data {
            let (input, number_of_developer_fields) = le_u8(input)?;
            (input, number_of_developer_fields, Vec::new())
        } else {
            (input, 0, Vec::new())
        };

    Ok((
        input,
        FitDefinitionMessage {
            byte_order,
            global_message_number: MesgNum::from_u16(global_message_number),
            number_of_fields,
            field_definitions,
            number_of_developer_fields,
            developer_field_definitions,
        },
    ))
}

fn data_message<'a>(
    input: &'a [u8],
    def_mesg: &FitDefinitionMessage,
) -> IResult<&'a [u8], FitDataMessage> {
    let mut fields = Vec::new();
    let mut input = input;
    for field_def in &def_mesg.field_definitions {
        let (i, value) = data_field_value(
            input,
            field_def.base_type,
            def_mesg.byte_order,
            field_def.size,
        )?;
        fields.push(value);
        input = i;
    }
    if def_mesg.number_of_developer_fields != 0u8 {
        panic!("Not Implemented: number_of_developer_fields > 0")
    }

    return Ok((input, FitDataMessage { fields }));
}

fn process_data_fields(
    data_mesg: FitDataMessage,
    def_mesg: &FitDefinitionMessage,
) -> Vec<DataField> {
    let mesg_info = def_mesg.global_message_number.message_info();
    let mut data_fields = Vec::new();
    for (def_field, dat_value) in def_mesg
        .field_definitions
        .iter()
        .zip(data_mesg.fields.iter())
    {
        if let Some(value) = dat_value {
            if let Some(field_info) = mesg_info.get_field(def_field.field_definition_number) {
                data_fields.push(DataField {
                    name: field_info.name().to_string(),
                    units: field_info.units().to_string(),
                    scale: field_info.scale(),
                    offset: field_info.offset(),
                    value: field_info.convert_value(value),
                    raw_value: value.clone(),
                });
            } else {
                data_fields.push(unknown_field(def_field.field_definition_number, &value));
            }
        }
    }

    if def_mesg.number_of_developer_fields != 0u8 {
        panic!("Not Implemented: number_of_developer_fields > 0")
    }

    data_fields
}

// Create an "unknown" field as a placeholder if we don't have any message information
fn unknown_field(field_def_num: u8, value: &DataFieldValue) -> DataField {
    DataField {
        name: format!("unknown_field_{}", field_def_num),
        units: "".to_string(),
        scale: 1.0,
        offset: 0.0,
        value: value.clone(),
        raw_value: value.clone(),
    }
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
            base_type: parse_base_type(base_type_field),
        },
    ))
}

/// Check the value of the last 5 bits to determine the base type.
///
/// Bits 5 and 6 are reserved so we don't check them and to avoid issues it's easier to
/// simply zero them out, 0x9f == 159 == 0b10011111. When the type can't be determined we default
/// to a byte vector.
fn parse_base_type(base_type_field: u8) -> BaseType {
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
        0x0D => BaseType::Byte,
        0x8E => BaseType::SInt64,
        0x8F => BaseType::UInt64,
        0x90 => BaseType::UInt64z,
        _ => BaseType::Byte,
    }
}

macro_rules! parse_f32 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f32($i) } else { nom::number::complete::le_f32($i) } } ););
macro_rules! parse_f64 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f64($i) } else { nom::number::complete::le_f64($i) } } ););

fn data_field_value(
    input: &[u8],
    base_type: BaseType,
    byte_order: Endianness,
    size: u8,
) -> IResult<&[u8], Option<DataFieldValue>> {
    let (input, value) = match base_type {
        BaseType::Enum => {
            let (input, value) = le_u8(input)?;
            (input, DataFieldValue::Enum(value))
        }
        BaseType::SInt8 => {
            let (input, value) = le_i8(input)?;
            (input, DataFieldValue::SInt8(value))
        }
        BaseType::UInt8 => {
            let (input, value) = le_u8(input)?;
            (input, DataFieldValue::UInt8(value))
        }
        BaseType::SInt16 => {
            let (input, value) = i16!(input, byte_order)?;
            (input, DataFieldValue::SInt16(value))
        }
        BaseType::UInt16 => {
            let (input, value) = u16!(input, byte_order)?;
            (input, DataFieldValue::UInt16(value))
        }
        BaseType::SInt32 => {
            let (input, value) = i32!(input, byte_order)?;
            (input, DataFieldValue::SInt32(value))
        }
        BaseType::UInt32 => {
            let (input, value) = u32!(input, byte_order)?;
            (input, DataFieldValue::UInt32(value))
        }
        BaseType::String => {
            let (input, value) = take(size as usize)(input)?;
            if let Ok(value) = String::from_utf8(value.to_vec()) {
                (input, DataFieldValue::String(value))
            } else {
                return Ok((input, None));
            }
        }
        BaseType::Float32 => {
            let (input, value) = parse_f32!(input, byte_order)?;
            (input, DataFieldValue::Float32(value))
        }
        BaseType::Float64 => {
            let (input, value) = parse_f64!(input, byte_order)?;
            (input, DataFieldValue::Float64(value))
        }
        BaseType::UInt8z => {
            let (input, value) = le_u8(input)?;
            (input, DataFieldValue::UInt8z(value))
        }
        BaseType::UInt16z => {
            let (input, value) = u16!(input, byte_order)?;
            (input, DataFieldValue::UInt16z(value))
        }
        BaseType::UInt32z => {
            let (input, value) = u32!(input, byte_order)?;
            (input, DataFieldValue::UInt32z(value))
        }
        BaseType::Byte => {
            let (input, value) = take(size as usize)(input)?;
            (input, DataFieldValue::Byte(Vec::from(value)))
        }
        BaseType::SInt64 => {
            let (input, value) = i64!(input, byte_order)?;
            (input, DataFieldValue::SInt64(value))
        }
        BaseType::UInt64 => {
            let (input, value) = u64!(input, byte_order)?;
            (input, DataFieldValue::UInt64(value))
        }
        BaseType::UInt64z => {
            let (input, value) = u64!(input, byte_order)?;
            (input, DataFieldValue::UInt64z(value))
        }
    };

    // Only return "something" if it's in the valid range
    if value.is_valid() {
        Ok((input, Some(value)))
    } else {
        Ok((input, None))
    }
}

/// Convert a split decimal style value with fix precision into a single floating point value
///
/// this function should never fail as long as integer values are passed in as the arguments
fn split_decimal_to_float<T: Display>(left: T, right: T) -> f32 {
    format!("{}.{}", left, right).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // I could define static byte slices for these basic functions to operate on
    // without loading an actual fixture file. I'll still want full file tests
    // down the road

    #[test]
    fn header_test() {
        let data = include_bytes!("../test/fixtures/Activity.fit");
        let hdr = fit_file_header(data).unwrap().1;
        assert_eq!(hdr.header_size, 12);
        assert_eq!(hdr.protocol_ver_enc, 1.0);
        assert_eq!(hdr.profile_ver_enc, 1.0);
        assert_eq!(hdr.data_size, 757);
        assert_eq!(hdr.crc, None);
    }

    #[test]
    fn message_header_test() {
        let data = include_bytes!("../test/fixtures/Activity.fit");
        let sl = &data[12..];
        let hdr = message_header(sl).unwrap().1;
        // need to asert that this is what I'm returning
        //Normal { message_type: Definition, contains_developer_data: false, local_message_type: 0 }
        // also need to test a CompressedTimestamp version and a Normal, Data version
    }
}
