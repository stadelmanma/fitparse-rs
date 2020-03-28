/// Parse FIT files
///
/// Logic largely based on: https://github.com/dtcooper/python-fitparse.
use crate::objects::*;
use crate::profile::field_types::MesgNum;
use crate::profile::{FieldInfo, MessageInfo};
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

#[derive(Debug)]
pub struct Ast {}


#[derive(Debug)]
pub struct FitDataRecordNode {}

/// Parse a collection of bytes into a Fit File AST
pub fn parse(input: &[u8]) -> IResult<&[u8], Ast> {
    unimplemented!("parsing bytes into ast")
    //fit_file(input)
}

#[derive(Clone, Copy, Debug)]
enum FitMessageType {
    Data,
    Definition,
}

/// FIT message headers are a single byte long and come in two forms, Normal and CompressedTimestamp.
///
/// The value of the bits inside is different for the two message header types but for simplicity
/// we treat them the same here and make the time_offset optionnal.
#[derive(Clone, Debug)]
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
struct DeveloperFieldDefinition {
    pub field_number: u8,
    pub size: u8,
    pub developer_data_index: u8,
}

/// Developer data ID messages are used to uniquely identify developer data field sources, a FIT
/// file can contain data for up to 255 unique developers. These messages must occur before any
/// related field description messages.
#[derive(Clone, Debug)]
struct DeveloperDataIdMessage {
    pub application_id: [u8; 16],
    pub developer_data_index: u8,
}

/// Field description messages define the meaning of data within a dev field, a FIT file can
/// contain up to 255 unique fields per developer. These messages must occur in the file before
/// any related data is added.
#[derive(Clone, Debug)]
struct DeveloperFieldDescription {
    pub developer_data_index: u8,
    pub field_definition_number: u8,
    pub fit_base_type_id: u8,
    pub field_name: String, // max size 64 bytes
    pub units: String,      // max size 16 bytes
    pub native_field_num: u8,
}

#[derive(Clone, Copy, Debug)]
enum BaseType {
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
            0x0D => BaseType::Byte,
            0x8E => BaseType::SInt64,
            0x8F => BaseType::UInt64,
            0x90 => BaseType::UInt64z,
            _ => BaseType::Byte,
        }
    }
}

/// Parse a FIT file into AST
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
        match fit_message(input.clone(), &mut definitions) {
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
fn fit_message<'a>(
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
            fit_message(input.clone(), definitions)
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
    // filter data message into a mapping of HashMap<def_num, DataField> for easier processing
    // since we don't export empty/invalid fields
    let mut data_map: HashMap<u8, DataFieldValue> = def_mesg
        .field_definitions
        .iter()
        .map(|f| f.field_definition_number)
        .zip(data_mesg.fields.into_iter())
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect();
    let mut data_fields = Vec::new();

    // populate data field vector with initial set of parsed fields
    let mut process_queue: Vec<u8> = data_map.keys().map(|k| k.clone()).collect();
    build_data_fields_from_map(&mesg_info, &mut process_queue, &mut data_map, &mut data_fields);

    if def_mesg.number_of_developer_fields != 0u8 {
        panic!("Not Implemented: number_of_developer_fields > 0")
    }

    data_fields
}

/// Recursive function to add processed data fields from raw values in the data mapping
fn build_data_fields_from_map(
    mesg_info: &MessageInfo,
    process_queue: &mut Vec<u8>,
    data_map: &mut HashMap<u8, DataFieldValue>,
    data_fields: &mut Vec<DataField>,
) {
    while !process_queue.is_empty() {
        let def_num = process_queue.remove(0);
        let value = &data_map[&def_num];

        if let Some(field_info) = mesg_info.get_field(def_num, &data_map) {
            // check for components, the decomposition is profile specific so
            // we dont store the parent field because we want the JSON to be
            // profile agnostic
            if field_info.components().is_empty() {
                data_fields.push(data_field_with_info(field_info, &value));
            } else {
                for (comp_def_num, comp_value) in field_info.expand_components(&value) {
                    // TODO modify fieldinfo with component scale,offset,units
                    // or replace it somehow, or do something otherwise my stuff is wrong
                    data_map.insert(comp_def_num, comp_value);
                    process_queue.push(comp_def_num);
                }
            }
        } else {
            data_fields.push(unknown_field(def_num, &value));
        }
    }
}


/// Build a data field using the provided FIT profile information
fn data_field_with_info(field_info: &FieldInfo, value: &DataFieldValue) -> DataField {
    DataField {
        name: field_info.name().to_string(),
        units: field_info.units().to_string(),
        scale: field_info.scale(),
        offset: field_info.offset(),
        value: field_info.convert_value(value),
        raw_value: value.clone(),
    }
}

/// Create an "unknown" field as a placeholder if we don't have any field information
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
            base_type: BaseType::from_u8(base_type_field),
        },
    ))
}

macro_rules! parse_f32 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f32($i) } else { nom::number::complete::le_f32($i) } } ););
macro_rules! parse_f64 ( ($i:expr, $e:expr) => ( {if nom::number::Endianness::Big == $e { nom::number::complete::be_f64($i) } else { nom::number::complete::le_f64($i) } } ););

fn data_field_value(
    input: &[u8],
    base_type: BaseType,
    byte_order: Endianness,
    size: u8,
) -> IResult<&[u8], Option<DataFieldValue>> {
    let mut input = input;
    let mut bytes_consumed = 0;
    let mut values: Vec<DataFieldValue> = Vec::new();

    while bytes_consumed < size {
        let (i, value) = match base_type {
            BaseType::Enum => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, DataFieldValue::Enum(v)))?
            }
            BaseType::SInt8 => {
                bytes_consumed += 1;
                le_i8(input).map(|(i, v)| (i, DataFieldValue::SInt8(v)))?
            }
            BaseType::UInt8 => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, DataFieldValue::UInt8(v)))?
            }
            BaseType::SInt16 => {
                bytes_consumed += 2;
                i16!(input, byte_order).map(|(i, v)| (i, DataFieldValue::SInt16(v)))?
            }
            BaseType::UInt16 => {
                bytes_consumed += 2;
                u16!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt16(v)))?
            }
            BaseType::SInt32 => {
                bytes_consumed += 4;
                i32!(input, byte_order).map(|(i, v)| (i, DataFieldValue::SInt32(v)))?
            }
            BaseType::UInt32 => {
                bytes_consumed += 4;
                u32!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt32(v)))?
            }
            BaseType::String => {
                bytes_consumed += size;
                let (input, value) = take(size as usize)(input)?;
                if let Ok(value) = String::from_utf8(value.to_vec()) {
                    (input, DataFieldValue::String(value))
                } else {
                    return Ok((input, None));
                }
            }
            BaseType::Float32 => {
                bytes_consumed += 4;
                parse_f32!(input, byte_order).map(|(i, v)| (i, DataFieldValue::Float32(v)))?
            }
            BaseType::Float64 => {
                bytes_consumed += 8;
                parse_f64!(input, byte_order).map(|(i, v)| (i, DataFieldValue::Float64(v)))?
            }
            BaseType::UInt8z => {
                bytes_consumed += 1;
                le_u8(input).map(|(i, v)| (i, DataFieldValue::UInt8z(v)))?
            }
            BaseType::UInt16z => {
                bytes_consumed += 2;
                u16!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt16z(v)))?
            }
            BaseType::UInt32z => {
                bytes_consumed += 4;
                u32!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt32z(v)))?
            }
            BaseType::Byte => {
                bytes_consumed += size;
                take(size as usize)(input).map(|(i, v)| (i, DataFieldValue::Byte(Vec::from(v))))?
            }
            BaseType::SInt64 => {
                bytes_consumed += 8;
                i64!(input, byte_order).map(|(i, v)| (i, DataFieldValue::SInt64(v)))?
            }
            BaseType::UInt64 => {
                bytes_consumed += 8;
                u64!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt64(v)))?
            }
            BaseType::UInt64z => {
                bytes_consumed += 8;
                u64!(input, byte_order).map(|(i, v)| (i, DataFieldValue::UInt64z(v)))?
            }
        };
        values.push(value);
        input = i;
    }

    // Return either a regular DataFieldValue or an Array of them
    let value = if values.len() == 1 {
        values[0].clone()
    } else {
        DataFieldValue::Array(values)
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
