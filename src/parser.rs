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
use nom::bytes::complete::{tag, take};
use nom::combinator::cond;
use nom::error::ErrorKind;
use nom::multi::count;
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::number::Endianness;
use nom::sequence::tuple;
use nom::u16;
use nom::IResult;
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
/// throughout the file.
fn parse_messages(input: &[u8]) -> IResult<&[u8], Vec<FitDataRecord>> {
    let mut definitions: HashMap<u8, FitMessage> = HashMap::new();
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
                println!("{:#?}", o);
                messages.push(o);
            }
        }
    }
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
            global_message_number,
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
            ..
        } = def_mesg
        {
            let mut data_fields = Vec::new();
            for field_def in field_definitions {
                let (input, value) = take(field_def.size as usize)(input)?;
                data_fields.push(DataField {
                    value: Vec::from(value),
                });
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
