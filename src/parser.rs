use nom::bytes::complete::{tag, take};
use nom::combinator::cond;
use nom::error::ErrorKind;
use nom::multi::{count, many1};
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::number::Endianness;
use nom::sequence::tuple;
use nom::u16;
use nom::IResult;

/// Parse FIT files
///
/// Logic largely based on: https://github.com/dtcooper/python-fitparse
///
/// Notes:
///   I'll want to eventually determine a way to check CRCs if it's not too messy
///   Apparently FIT files can be chained? If so I'll need to conditionally rerun the parser
///   Also profiles are configurable and vary on SDK release I think so I'll need a way to customize
///   that
use std::collections::HashMap;
use std::fmt::Display;

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

#[derive(Clone, Debug)]
pub struct DataField {}

pub fn parse_file(input: &[u8]) -> IResult<&[u8], FitFile> {
    fit_file(input)
}

/// Parse a FIT file
fn fit_file(input: &[u8]) -> IResult<&[u8], FitFile> {
    let (input, header) = fit_file_header(input)?;
    let (input, messages) = parse_messages(input)?;
    let (input, crc) = le_u16(input)?;

    Ok((
        input,
        FitFile {
            header,
            messages,
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
/// throughout the file.
fn parse_messages(input: &[u8]) -> IResult<&[u8], Vec<FitDataRecord>> {
    let mut definitions: HashMap<u8, FitDataRecord> = HashMap::new();
    let mut messages = Vec::new();
    let mut input = input.clone();
    loop {
        match fit_data_record(input.clone(), &mut definitions) {
            Err(nom::Err::Error(_)) => return Ok((input, messages)),
            Err(e) => return Err(e),
            Ok((new_inp, o)) => {
                if new_inp == input {
                    return Err(nom::Err::Error((input, ErrorKind::Many1)));
                }

                input = new_inp;
                messages.push(o);
            }
        }
    }

    Ok((input, messages))
}

/// parse a single FIT data record which can define further fields or actaully cotain data itself
fn fit_data_record<'a>(
    input: &'a [u8],
    definitions: &mut HashMap<u8, FitDataRecord>,
) -> IResult<&'a [u8], FitDataRecord> {
    let (input, header) = message_header(input)?;
    let (input, message) = match &header {
        FitMessageHeader::Normal {
            message_type,
            contains_developer_data,
            local_message_type,
        } => match message_type {
            Definition => definition_message(input, *contains_developer_data)?,
            Data => panic!("Not implemented yet"),
        },
        CompressedTimestamp => panic!("Not implemented yet"),
    };
    let record = FitDataRecord { header, message };
    if let FitMessageHeader::Normal {
        message_type: FitMessageType::Definition,
        local_message_type,
        ..
    } = &record.header
    {
        definitions.insert(*local_message_type, record.clone());
    }

    Ok((input, record))
}

/// Parse the header of a single FIT message
fn message_header(input: &[u8]) -> IResult<&[u8], FitMessageHeader> {
    let msg_header_byte: u8 = le_u8(input)?.1;

    if msg_header_byte & 0x80 == 1 {
        Ok((
            &input[1..],
            FitMessageHeader::CompressedTimestamp {
                local_message_type: (msg_header_byte >> 5) & 0x3, // bits 5-6,
                time_offset: msg_header_byte & 0x1F,
            },
        ))
    } else {
        let msg_type = if msg_header_byte & 0x40 == 1 {
            FitMessageType::Data
        } else {
            FitMessageType::Definition
        };
        Ok((
            &input[1..],
            FitMessageHeader::Normal {
                message_type: msg_type,
                contains_developer_data: msg_header_byte & 0x20 == 1,
                local_message_type: msg_header_byte & 0xF,
            },
        ))
    }
}

/// parse a definition message
fn definition_message(input: &[u8], contains_developer_data: bool) -> IResult<&[u8], FitMessage> {
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
        FitMessage::Definition {
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
    let (input, base_type) = le_u8(input)?;

    Ok((
        input,
        FieldDefinition {
            field_definition_number,
            size,
            base_type,
        },
    ))
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
