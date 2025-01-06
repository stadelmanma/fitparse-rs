//! Helper functions and structures needed to parse a FIT file.
use crate::profile::field_types::FitBaseType;
use crate::{DeveloperFieldDescription, Value};
use nom::bytes::streaming::{tag, take};
use nom::combinator::cond;
use nom::multi::count;
use nom::number::streaming::{
    f32, f64, i16, i32, i64, le_i8, le_u16, le_u32, le_u8, u16, u32, u64,
};
use nom::number::Endianness;
use nom::sequence::tuple;
use nom::{Err, IResult, Needed};
use std::collections::HashMap;
use std::convert::From;
use std::sync::Arc;

/// Define an is_valid function needed for parsing here, this function is not needed for normal use
impl Value {
    fn is_valid(&self) -> bool {
        match self {
            Value::Enum(val) => *val != 0xFF,
            Value::SInt8(val) => *val != 0x7F,
            Value::UInt8(val) => *val != 0xFF,
            Value::SInt16(val) => *val != 0x7FFF,
            Value::UInt16(val) => *val != 0xFFFF,
            Value::SInt32(val) => *val != 0x7FFF_FFFF,
            Value::UInt32(val) => *val != 0xFFFF_FFFF,
            Value::String(val) => !val.contains('\0'),
            Value::Timestamp(_) => true, // timestamps are always valid
            Value::Float32(val) => val.is_finite(),
            Value::Float64(val) => val.is_finite(),
            Value::UInt8z(val) => *val != 0x0,
            Value::UInt16z(val) => *val != 0x0,
            Value::UInt32z(val) => *val != 0x0,
            Value::Byte(val) => *val != 0xFF,
            Value::SInt64(val) => *val != 0x7FFF_FFFF_FFFF_FFFF,
            Value::UInt64(val) => *val != 0xFFFF_FFFF_FFFF_FFFF,
            Value::UInt64z(val) => *val != 0x0,
            Value::Array(vals) => !vals.is_empty() && vals.iter().any(|v| v.is_valid()),
            Value::Invalid => false,
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

    /// Return the header CRC for the FIT file if present
    pub fn crc(&self) -> Option<u16> {
        self.crc
    }
}

/// Type of FIT message being read as specified by the header byte
#[derive(Clone, Copy, Debug, PartialEq)]
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
    local_message_number: u8,
    message_type: FitMessageType,
    time_offset: Option<u8>,
}

/// Enum used to return either a data message or a new definition message when parsing the body of
/// the FIT file.
pub enum FitMessage {
    Data(FitDataMessage),
    Definition(FitDefinitionMessage),
    /// Used to pass an error up the chain so we can emit a proper error of our own type
    MissingDefinitionMessage(u8),
}

/// The definition message is used to create an association between the local message type
/// contained in the record header, and a Global Message Number (mesg_num) that relates to the
/// global FIT message. Although 1 byte is available for the number of fields and 1 byte is
/// available for the field size, no single message may be defined that is larger than 255 bytes.
#[derive(Clone, Debug)]
pub struct FitDefinitionMessage {
    byte_order: Endianness,
    local_message_number: u8,
    global_message_number: u16,
    field_definitions: Vec<FieldDefinition>,
    developer_field_definitions: Vec<DeveloperFieldDefinition>,
}

impl FitDefinitionMessage {
    /// The byte order of the data fields in the associated FIT data message
    pub fn byte_order(&self) -> Endianness {
        self.byte_order
    }

    /// Local message number used when decoding a file
    pub fn local_message_number(&self) -> u8 {
        self.local_message_number
    }

    /// Global message number defined in FIT profile
    pub fn global_message_number(&self) -> u16 {
        self.global_message_number
    }

    /// Definitions for each data field encoded in the associated FIT data message
    pub fn field_definitions(&self) -> &[FieldDefinition] {
        &self.field_definitions
    }

    /// Developer field definitions
    pub fn developer_field_definitions(&self) -> &[DeveloperFieldDefinition] {
        &self.developer_field_definitions
    }

    /// Calculate and return the size of the data message described.
    /// The max theoretical size of a data message is 1 + (255 * 255) + (255 * 255) = 130KB
    /// which would be 255 regular and 255 developer fields each 255 bytes long plus
    /// the header byte.
    pub fn data_message_size(&self) -> usize {
        // start accumlator at one to account for the message header
        self.field_definitions
            .iter()
            .fold(1, |l, f| l + f.size as usize)
            + self
                .developer_field_definitions
                .iter()
                .fold(0, |l, f| l + f.size as usize)
    }
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
    base_type: FitBaseType,
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
/// the information from its defintion message and the MessageInfo struct from the FIT profile
#[derive(Clone, Debug)]
pub struct FitDataMessage {
    global_message_number: u16,
    time_offset: Option<u8>,
    /// Data field mapping of <(dev_data_idx, field_number), Value>
    pub fields: HashMap<u8, Value>, //indexed by field_nr
    /// Mutable Data field mapping of <(dev_data_idx, field_number), Value>
    pub developer_fields: HashMap<(u8, u8), Value>, //indexed by (dev_data_idx, field_nr)
}

impl FitDataMessage {
    /// Global message number defined in the FIT profile, set by definition message
    pub fn global_message_number(&self) -> u16 {
        self.global_message_number
    }

    /// Time offset used to generate a full timestamp from a reference time
    pub fn time_offset(&self) -> Option<u8> {
        self.time_offset
    }

    /// Data field mapping of <(dev_data_idx, field_number), Value>
    pub fn fields(&self) -> &HashMap<u8, Value> {
        &self.fields
    }

    /// Mutable Data field mapping of <(dev_data_idx, field_number), Value>
    pub fn fields_mut(&mut self) -> &mut HashMap<u8, Value> {
        &mut self.fields
    }

    /// Data field mapping of <(dev_data_idx, field_number), Value>
    pub fn developer_fields(&self) -> &HashMap<(u8, u8), Value> {
        &self.developer_fields
    }
}

impl FitBaseType {
    /// The size for fixed width numeric values, for variable Llength types it's the smallest chunk size.
    /// FitBaseType is defined in the autogenerated profile/field_types.rs, but we add this extra size
    /// information outside of the autogenerated code.
    fn size(&self) -> u8 {
        match *self {
            FitBaseType::Enum => 1,
            FitBaseType::Sint8 => 1,
            FitBaseType::Uint8 => 1,
            FitBaseType::Sint16 => 2,
            FitBaseType::Uint16 => 2,
            FitBaseType::Sint32 => 4,
            FitBaseType::Uint32 => 4,
            FitBaseType::String => 1,
            FitBaseType::Float32 => 4,
            FitBaseType::Float64 => 8,
            FitBaseType::Uint8z => 1,
            FitBaseType::Uint16z => 2,
            FitBaseType::Uint32z => 4,
            FitBaseType::Byte => 1,
            FitBaseType::Sint64 => 8,
            FitBaseType::Uint64 => 8,
            FitBaseType::Uint64z => 8,
            _ => 1, //Unknown FitBaseType, default to Byte
        }
    }
}

/// Parse the FIT file header
pub fn fit_file_header(input: &[u8]) -> IResult<&[u8], FitFileHeader> {
    match fit_file_header_impl(input) {
        Ok(r) => Ok(r),
        Err(Err::Incomplete(_)) => {
            // output a correct "needed" value, assume 14 bytes as default since that's preferred
            Err(Err::Incomplete(Needed::new(
                input.first().map_or(14, |v| *v as usize - input.len()),
            )))
        }
        Err(r) => Err(r),
    }
}

/// Parse the FIT file header, the public function wraps an incomplete error to fix the needed bytes
fn fit_file_header_impl(input: &[u8]) -> IResult<&[u8], FitFileHeader> {
    let (input, (header_size, proto, prof, data_size)) =
        tuple((le_u8, le_u8, le_u16, le_u32))(input)?;
    let (input, _) = tag(".FIT")(input)?;
    let (input, crc) = cond(header_size > 12, le_u16)(input)?;
    let protocol_ver_enc =
        split_decimal_to_float((proto >> 4) as u16, (proto & ((1 << 4) - 1)) as u16);
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
fn split_decimal_to_float(left: u16, right: u16) -> f32 {
    let scale = ((right as f32).log10() + 1f32).floor() as i32;
    if right > 0 {
        (left as f32) + (right as f32) / 10f32.powi(scale)
    } else {
        left as f32
    }
}

/// Parse a FIT data or definition message
pub fn fit_message<'a>(
    input: &'a [u8],
    definitions: &HashMap<u8, Arc<FitDefinitionMessage>>,
    developer_field_descriptions: &HashMap<(u8, u8), DeveloperFieldDescription>,
) -> IResult<&'a [u8], FitMessage> {
    // parse a single message of either variety
    let (input, header) = message_header(input)?;
    match header.message_type {
        FitMessageType::Data => {
            if let Some(def_mesg) = definitions.get(&header.local_message_number) {
                let (input, (fields, developer_fields)) =
                    data_message_fields(input, def_mesg, developer_field_descriptions)?;
                Ok((
                    input,
                    FitMessage::Data(FitDataMessage {
                        fields,
                        developer_fields,
                        global_message_number: def_mesg.global_message_number,
                        time_offset: header.time_offset,
                    }),
                ))
            } else {
                // this is technically is an Error but nom can't represent it well
                Ok((
                    input,
                    FitMessage::MissingDefinitionMessage(header.local_message_number),
                ))
            }
        }
        FitMessageType::Definition => {
            let (input, message) = definition_message(input, &header)?;
            Ok((input, FitMessage::Definition(message)))
        }
    }
}

/// Parse the header of a single FIT message
fn message_header(input: &[u8]) -> IResult<&[u8], FitMessageHeader> {
    let (input, msg_header_byte) = le_u8(input)?;
    let contains_developer_data: bool;
    let local_message_number: u8;
    let message_type: FitMessageType;
    let time_offset: Option<u8>;

    if msg_header_byte & 0x80 == 0x80 {
        // compressed timestamp header
        contains_developer_data = false;
        local_message_number = (msg_header_byte >> 5) & 0x3; // bits 5-6
        message_type = FitMessageType::Data;
        time_offset = Some(msg_header_byte & 0x1F);
    } else if (msg_header_byte & 0x40) == 0x40 {
        contains_developer_data = msg_header_byte & 0x20 == 0x20;
        local_message_number = msg_header_byte & 0xF;
        message_type = FitMessageType::Definition;
        time_offset = None;
    } else {
        // developer data bit is reserved for Data messages and should be 0, so we don't check it
        contains_developer_data = false;
        local_message_number = msg_header_byte & 0xF;
        message_type = FitMessageType::Data;
        time_offset = None;
    }

    Ok((
        input,
        FitMessageHeader {
            contains_developer_data,
            local_message_number,
            message_type,
            time_offset,
        },
    ))
}

/// parse a definition message
fn definition_message<'a>(
    input: &'a [u8],
    header: &FitMessageHeader,
) -> IResult<&'a [u8], FitDefinitionMessage> {
    let (input, _) = take(1usize)(input)?; // reserved byte, consume it and ignore the value
    let (input, arch_byte) = le_u8(input)?;
    let byte_order = if arch_byte == 1 {
        Endianness::Big
    } else {
        Endianness::Little
    };
    let (input, global_message_number) = u16(byte_order)(input)?;
    let (input, number_of_fields) = le_u8(input)?;
    let (input, field_definitions) = count(field_definition, number_of_fields as usize)(input)?;
    let (input, developer_field_definitions) = if header.contains_developer_data {
        let (input, nflds) = le_u8(input)?;
        let (input, dev_fld_defs) = count(developer_field_definition, nflds as usize)(input)?;
        (input, dev_fld_defs)
    } else {
        (input, Vec::new())
    };

    Ok((
        input,
        FitDefinitionMessage {
            byte_order,
            local_message_number: header.local_message_number,
            global_message_number,
            field_definitions,
            developer_field_definitions,
        },
    ))
}

fn field_definition(input: &[u8]) -> IResult<&[u8], FieldDefinition> {
    let (input, field_definition_number) = le_u8(input)?;
    let (input, size) = le_u8(input)?;
    let (input, base_type_field) = le_u8(input)?;
    // Mask out bit 5 and 6, since base type should ignore these
    let mut base_type = FitBaseType::from(base_type_field & 0x9f);
    // check that the field size is a valid multiple of the base size, if not drop data into a
    // byte array since the field value is undefined. This prevents a potential add-overflow
    // panic in the `data_field_value` function.
    if size % base_type.size() != 0 {
        eprintln!(
            "ERROR: field size: {} is not a multiple of the base type {:?} (size {}) parsing as a byte array",
            size,
            base_type,
            base_type.size()
        );
        base_type = FitBaseType::Byte;
    }

    Ok((
        input,
        FieldDefinition {
            field_definition_number,
            size,
            base_type,
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
#[allow(clippy::type_complexity)]
fn data_message_fields<'a>(
    input: &'a [u8],
    def_mesg: &FitDefinitionMessage,
    developer_field_descriptions: &HashMap<(u8, u8), DeveloperFieldDescription>,
) -> IResult<&'a [u8], (HashMap<u8, Value>, HashMap<(u8, u8), Value>)> {
    match data_message_fields_impl(input, def_mesg, developer_field_descriptions) {
        Ok(r) => Ok(r),
        Err(Err::Incomplete(_)) => {
            // output a correct "needed" value, subtract one because we've already parsed the header
            Err(Err::Incomplete(Needed::new(
                def_mesg.data_message_size() - input.len() - 1,
            )))
        }
        Err(r) => Err(r),
    }
}

/// Function to actually parse the data fields, the public function wraps incomplete errors to provide
/// an accurate number for the bytes "needed" if we hit an incomplete error
#[allow(clippy::type_complexity)]
fn data_message_fields_impl<'a>(
    input: &'a [u8],
    def_mesg: &FitDefinitionMessage,
    developer_field_descriptions: &HashMap<(u8, u8), DeveloperFieldDescription>,
) -> IResult<&'a [u8], (HashMap<u8, Value>, HashMap<(u8, u8), Value>)> {
    let mut fields = HashMap::new();
    let mut developer_fields = HashMap::new();
    let mut input = input;
    for field_def in &def_mesg.field_definitions {
        let (i, value) = data_field_value(
            input,
            field_def.base_type,
            def_mesg.byte_order,
            field_def.size,
        )?;
        if let Some(value) = value {
            fields.insert(field_def.field_definition_number, value);
        }
        input = i;
    }
    // handle developer fields analogously to fixed fields
    for field_def in &def_mesg.developer_field_definitions {
        let dev_field_description = developer_field_descriptions
            .get(&(field_def.developer_data_index, field_def.field_number))
            .ok_or({
                nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Fail,
                })
            })?;
        let (i, value) = data_field_value(
            input,
            dev_field_description.fit_base_type_id,
            def_mesg.byte_order,
            field_def.size,
        )?;
        if let Some(value) = value {
            developer_fields.insert(
                (field_def.developer_data_index, field_def.field_number),
                value,
            );
        }
        input = i;
    }

    Ok((input, (fields, developer_fields)))
}

/// Parse a single raw data value.
///
/// This can panic if the size is greater than `255 - base_type.size()` but that should only
/// occur when the field size is not a multiple of the base_type size. Size agreement is checked
/// when parsing field definitions preventing overflow during regular normal execution.
fn data_field_value(
    input: &[u8],
    base_type: FitBaseType,
    byte_order: Endianness,
    size: u8,
) -> IResult<&[u8], Option<Value>> {
    let mut input = input;
    let mut bytes_consumed = 0;
    let mut values: Vec<Value> = Vec::with_capacity((size / base_type.size()) as _);

    while bytes_consumed < size {
        let (i, value) = match base_type {
            FitBaseType::Enum => le_u8(input).map(|(i, v)| (i, Value::Enum(v)))?,
            FitBaseType::Sint8 => le_i8(input).map(|(i, v)| (i, Value::SInt8(v)))?,
            FitBaseType::Uint8 => le_u8(input).map(|(i, v)| (i, Value::UInt8(v)))?,
            FitBaseType::Sint16 => i16(byte_order)(input).map(|(i, v)| (i, Value::SInt16(v)))?,
            FitBaseType::Uint16 => u16(byte_order)(input).map(|(i, v)| (i, Value::UInt16(v)))?,
            FitBaseType::Sint32 => i32(byte_order)(input).map(|(i, v)| (i, Value::SInt32(v)))?,
            FitBaseType::Uint32 => u32(byte_order)(input).map(|(i, v)| (i, Value::UInt32(v)))?,
            FitBaseType::String => {
                // consume the field as defined by its size and then locate the first NUL byte
                // and ignore everything after it when converting to a string
                let (input, field_value) = take(size as usize)(input)?;
                let field_value = &field_value[0..field_value
                    .iter()
                    .position(|v| *v == 0u8)
                    .unwrap_or(field_value.len())];
                if let Ok(value) = String::from_utf8(field_value.to_owned()) {
                    return Ok((input, Some(Value::String(value))));
                } else {
                    return Ok((input, None));
                }
            }
            FitBaseType::Float32 => f32(byte_order)(input).map(|(i, v)| (i, Value::Float32(v)))?,
            FitBaseType::Float64 => f64(byte_order)(input).map(|(i, v)| (i, Value::Float64(v)))?,
            FitBaseType::Uint8z => le_u8(input).map(|(i, v)| (i, Value::UInt8z(v)))?,
            FitBaseType::Uint16z => u16(byte_order)(input).map(|(i, v)| (i, Value::UInt16z(v)))?,
            FitBaseType::Uint32z => u32(byte_order)(input).map(|(i, v)| (i, Value::UInt32z(v)))?,
            FitBaseType::Byte => le_u8(input).map(|(i, v)| (i, Value::Byte(v)))?,
            FitBaseType::Sint64 => i64(byte_order)(input).map(|(i, v)| (i, Value::SInt64(v)))?,
            FitBaseType::Uint64 => u64(byte_order)(input).map(|(i, v)| (i, Value::UInt64(v)))?,
            FitBaseType::Uint64z => u64(byte_order)(input).map(|(i, v)| (i, Value::UInt64z(v)))?,
            _ => le_u8(input).map(|(i, v)| (i, Value::UInt8(v)))?, // Treat unexpected like Byte
        };
        bytes_consumed += base_type.size();
        if matches!(base_type, FitBaseType::Byte) || value.is_valid() {
            values.push(value);
        } else {
            values.push(Value::Invalid)
        }
        input = i;
    }

    // Return either a regular Value or an Array of them
    let value = if values.len() == 1 {
        values.swap_remove(0)
    } else {
        values.shrink_to_fit();
        Value::Array(values)
    };

    // Strip invalid values from the output
    if value.is_valid() {
        Ok((input, Some(value)))
    } else {
        Ok((input, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // I could define static byte slices for these basic functions to operate on
    // without loading an actual fixture file. I'll still want full file tests
    // down the road

    #[test]
    fn header_test() {
        let data = include_bytes!("../../tests/fixtures/Activity.fit");
        let (_, hdr) = fit_file_header(data).unwrap();
        assert_eq!(hdr.header_size, 12);
        assert_eq!(hdr.protocol_ver_enc, 1.0);
        assert_eq!(hdr.profile_ver_enc, 1.0);
        assert_eq!(hdr.data_size, 757);
        assert_eq!(hdr.crc, None);
    }

    #[test]
    fn definition_message_header_test() {
        let data = include_bytes!("../../tests/fixtures/Activity.fit");
        let sl = &data[12..];
        let (_, hdr) = message_header(sl).unwrap();

        assert!(!hdr.contains_developer_data);
        assert_eq!(hdr.local_message_number, 0);
        assert_eq!(hdr.message_type, FitMessageType::Definition);
        assert_eq!(hdr.time_offset, None);
    }

    #[test]
    fn data_field_value_test_single_value() {
        let data = [0x01, 0xFF];

        // parse off a valid byte
        let (rem, val) =
            data_field_value(&data, FitBaseType::Uint8, Endianness::Native, 1).unwrap();
        match val {
            Some(v) => assert_eq!(v, Value::UInt8(0x01)),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[0xFF]);

        // parse off an invalid byte
        let (rem, val) = data_field_value(rem, FitBaseType::Uint8, Endianness::Native, 1).unwrap();
        if val.is_some() {
            panic!("None should be returned for invalid bytes.")
        }
        assert_eq!(rem, &[]);

        // parse two byte values with defined endianess
        let (rem, val) = data_field_value(&data, FitBaseType::Uint16, Endianness::Big, 2).unwrap();
        match val {
            Some(v) => assert_eq!(v, Value::UInt16(0x01FF)),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[]);

        let (rem, val) =
            data_field_value(&data, FitBaseType::Uint16, Endianness::Little, 2).unwrap();
        match val {
            Some(v) => assert_eq!(v, Value::UInt16(0xFF01)),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[]);
    }

    #[test]
    fn data_field_value_test_array_value() {
        let data = [0x00, 0x01, 0x02, 0x03, 0xFF, 0x05];

        // parse off a valid byte
        let (rem, val) =
            data_field_value(&data, FitBaseType::Uint8, Endianness::Native, 4).unwrap();
        match val {
            Some(v) => assert_eq!(
                v,
                Value::Array(vec![
                    Value::UInt8(0x00),
                    Value::UInt8(0x01),
                    Value::UInt8(0x02),
                    Value::UInt8(0x03)
                ])
            ),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[0xFF, 0x05]);

        // parse off an invalid byte
        let (rem, val) =
            data_field_value(&data, FitBaseType::Uint8, Endianness::Native, 6).unwrap();
        match val {
            Some(v) => assert_eq!(
                v,
                Value::Array(vec![
                    Value::UInt8(0x00),
                    Value::UInt8(0x01),
                    Value::UInt8(0x02),
                    Value::UInt8(0x03),
                    Value::Invalid,
                    Value::UInt8(0x05),
                ])
            ),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[]);
    }

    #[test]
    fn data_field_value_test_byte_array_value() {
        let data = [0xFF, 0x01, 0xFF, 0x03, 0xFF, 0x05];

        // parse off a valid byte array containing 0xFF bytes
        let (rem, val) = data_field_value(&data, FitBaseType::Byte, Endianness::Native, 4).unwrap();
        match val {
            Some(v) => assert_eq!(
                v,
                Value::Array(vec![
                    Value::Byte(0xFF),
                    Value::Byte(0x01),
                    Value::Byte(0xFF),
                    Value::Byte(0x03),
                ])
            ),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[0xFF, 0x05]);
    }

    #[test]
    fn data_field_value_test_string_value() {
        let data = [71, 65, 82, 77, 73, 78, 0, 63, 255];

        // parse off a valid byte
        let (rem, val) =
            data_field_value(&data, FitBaseType::String, Endianness::Native, 8).unwrap();
        match val {
            Some(v) => assert_eq!(v, Value::String(String::from("GARMIN"))),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[0xFF]);

        // parse invalid UTF8 string
        let data = [71, 195, 40, 77, 73, 78, 0, 63, 255];
        let (rem, val) =
            data_field_value(&data, FitBaseType::String, Endianness::Native, 8).unwrap();
        if val.is_some() {
            panic!("None should be returned for invalid string.")
        }
        assert_eq!(rem, &[0xFF]);

        // parse string with NUL byte before invalid UTF8 sequence
        let data = [71, 65, 82, 77, 0, 195, 40, 63, 255];
        let (rem, val) =
            data_field_value(&data, FitBaseType::String, Endianness::Native, 8).unwrap();
        match val {
            Some(v) => assert_eq!(v, Value::String(String::from("GARM"))),
            None => panic!("No value returned."),
        }
        assert_eq!(rem, &[0xFF]);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn data_field_value_test_size_mismatch_array_value() {
        // try and parse an array with a size that isn't a multiple of the base type
        let data: Vec<u8> = (0..=255).collect();
        match data_field_value(&data, FitBaseType::Uint16, Endianness::Native, 255) {
            Ok(..) | Err(..) => {}
        };
    }

    #[test]
    fn value_array_is_valid() {
        let val = Value::Array(vec![Value::UInt8(0xFF), Value::UInt8(42u8)]);
        assert!(
            val.is_valid(),
            "This Value array should be valid since it contains a valid value"
        );
        let val = Value::Array(vec![Value::Invalid, Value::UInt8(42u8)]);
        assert!(
            val.is_valid(),
            "This Value array should be valid since it contains a valid value"
        );
        let val = Value::Array(vec![Value::UInt8(0x00), Value::UInt8(42u8)]);
        assert!(
            val.is_valid(),
            "This Value array should be valid since it contains only valid values"
        );
        let val = Value::Array(vec![Value::Byte(0xFF), Value::Byte(0xFF)]);
        assert!(
            !val.is_valid(),
            "This Value array should be invalid since it contains no valid values"
        );
    }
}
