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
/// throughout the file. This contiues until we reach the 2 byte CRC at the end of the input stream
fn parse_messages(input: &[u8]) -> IResult<&[u8], Vec<FitDataRecord>> {
    let mut definitions: HashMap<u8, FitMessage> = HashMap::new();
    let mut messages = Vec::new();
    let mut input = input.clone();
    while input.len() > 2 {
        match fit_data_record(input.clone(), &mut definitions) {
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
    definitions: &mut HashMap<u8, FitMessage>,
) -> IResult<&'a [u8], FitDataRecord> {
    let (input, header) = message_header(input)?;
    let (input, message) = match &header {
        FitMessageHeader::Normal {
            message_type,
            contains_developer_data,
            local_message_type,
        } => match message_type {
            FitMessageType::Definition => definition_message(input, *contains_developer_data)?,
            FitMessageType::Data => data_message(input, definitions.get(local_message_type))?,
        },
        FitMessageHeader::CompressedTimestamp {
            local_message_type, ..
        } => data_message(input, definitions.get(local_message_type))?,
    };
    let record = FitDataRecord { header, message };
    if let FitMessageHeader::Normal {
        message_type: FitMessageType::Definition,
        local_message_type,
        ..
    } = &record.header
    {
        definitions.insert(*local_message_type, record.message.clone());
    }

    Ok((input, record))
}

/// Parse the header of a single FIT message
fn message_header(input: &[u8]) -> IResult<&[u8], FitMessageHeader> {
    let (input, msg_header_byte) = le_u8(input)?;

    if msg_header_byte & 0x80 == 0x80 {
        Ok((
            &input[1..],
            FitMessageHeader::CompressedTimestamp {
                local_message_type: (msg_header_byte >> 5) & 0x3, // bits 5-6,
                time_offset: msg_header_byte & 0x1F,
            },
        ))
    } else {
        let msg_type = if (msg_header_byte & 0x40) == 0x40 {
            FitMessageType::Definition
        } else {
            FitMessageType::Data
        };
        Ok((
            input,
            FitMessageHeader::Normal {
                message_type: msg_type,
                contains_developer_data: msg_header_byte & 0x20 == 0x20,
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
    definition: Option<&FitMessage>,
) -> IResult<&'a [u8], FitMessage> {
    if let Some(def_mesg) = definition {
        if let FitMessage::Definition {
            field_definitions,
            number_of_developer_fields,
            byte_order,
            ..
        } = def_mesg
        {
            let mut data_fields = Vec::new();
            let mut input = input;
            for field_def in field_definitions {
                let (i, value) =
                    data_field_value(input, field_def.base_type, *byte_order, field_def.size)?;
                data_fields.push(value);
                input = i;
            }
            if *number_of_developer_fields != 0u8 {
                panic!("Not Implemented: number_of_developer_fields > 0")
            }

            return Ok((input, FitMessage::Data { data_fields }));
        }
    }

    Err(nom::Err::Failure((input, ErrorKind::Verify)))
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
