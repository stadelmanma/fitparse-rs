//! Deserialize a stream of FIT file data into the serde data model by parsing the file and
//! applying the packaged FIT profile to the data.
use crate::error::{ErrorKind, Result};
use crate::FitDataRecord;
use core::iter::Iterator;
use nom::number::complete::le_u16;
use std::collections::HashMap;
use std::io::Read;

mod decode;
use decode::Decoder;
mod parser;

/// Stores a FIT file object (header, message or CRC)
#[derive(Clone, Debug)]
pub enum FitObject {
    /// Checksum at end of data section
    Crc(u16),
    /// Header containing FIT file info
    Header(parser::FitFileHeader),
    /// A raw data message
    DataMessage(parser::FitDataMessage),
    /// A definition message used to define upcoming data messages
    DefinitionMessage(parser::FitDefinitionMessage),
}

/// Manages the deserialization of a FIT data stream into Rust constructs.
struct Deserializer {
    /// Track the current set of FIT message definitions, these are what allows the format to
    /// be self describing.
    definitions: HashMap<u8, parser::FitDefinitionMessage>,
    /// Stores the current position in the byte stream, this is needed for error generation and
    /// checking the state of the parser
    position: usize,
    /// Stores the location that the current FIT message ends, for chained FIT messges this will
    /// be updated to reflect the new end position
    end_of_messages: usize,
}

impl Deserializer {
    /// Create the deserializer with an empty state
    pub fn new() -> Self {
        Deserializer {
            definitions: HashMap::new(),
            position: 0,
            end_of_messages: 0,
        }
    }

    /// Clear the definition messages used to decode data messages. This can be called between
    /// distinct FIT files but if they are properly formed it should not be necessary since new
    /// definitions will replace the old in the mapping.
    pub fn clear_definitions(&mut self) {
        self.definitions = HashMap::new();
    }

    pub fn deserialize<'de>(&'de mut self, input: &'de [u8]) -> DeserializerIter<'de> {
        DeserializerIter {
            buffer: input,
            deserializer: self,
        }
    }

    /// Advance the parser state returning one of four possible objects defined within the
    /// FIT file.
    pub fn deserialize_next<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        if self.position > 0 && self.position == self.end_of_messages {
            // extract the CRC, eventually we'd want to validate it
            return self.deserialize_crc(input);
        }
        if self.position == 0 || (self.position > self.end_of_messages && !input.is_empty()) {
            // if there is extra bytes remaining the FIT file must be chained so we parse
            // the header and continue on.
            return self.deserialize_header(input);
        }
        // if we reach this point then we must be at some position: 0 < X < self.end_of_messages
        // and a message should exist (either data or definition).
        self.deserialize_message(input)
    }

    /// Parse the FIT header
    fn deserialize_header<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        let (input, header) = parser::fit_file_header(input).map_err(|e| self.to_parse_err(e))?;
        self.end_of_messages =
            self.position + header.header_size() as usize + header.data_size() as usize;
        self.position += header.header_size() as usize;

        Ok((input, FitObject::Header(header)))
    }

    /// Extract a 2 byte CRC
    fn deserialize_crc<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        let (input, crc) = le_u16(input).map_err(|e| self.to_parse_err(e))?;
        self.position += 2;
        Ok((input, FitObject::Crc(crc)))
    }

    /// Parse a FIT data or definition message
    fn deserialize_message<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        // parse a single message of either variety
        let init_len = input.len();
        let (input, message) = parser::fit_message(input, &self.definitions).map_err(|e| self.to_parse_err(e))?;
        match message {
            parser::FitMessage::Data(message) => {
                self.position += init_len - input.len();
                Ok((input, FitObject::DataMessage(message)))
            }
            parser::FitMessage::Definition(message) => {
                // I could use an Rc here to avoid cloning the definition message directly.
                // I don't think I need an Arc since multithreaded parsing of a single FIT file
                // doesn't make a ton of sense.
                self.definitions.insert(message.local_message_number(), message.clone());
                self.position += init_len - input.len();
                Ok((input, FitObject::DefinitionMessage(message)))
            }
            parser::FitMessage::MissingDefinitionMessage(n) => {
                Err(ErrorKind::MissingDefinitionMessage(n, self.position).into())
            }
        }
    }

    /// Inject the byte stream position into the Error when converting a nom parsing error. This
    /// is not easy to get using the vanilla From trait since we need outside information.
    fn to_parse_err(&self, err: nom::Err<(&[u8], nom::error::ErrorKind)>) -> crate::Error {
        match err {
            nom::Err::Error((_, kind)) => ErrorKind::ParseError(self.position, kind).into(),
            nom::Err::Failure((_, kind)) => ErrorKind::ParseError(self.position, kind).into(),
            nom::Err::Incomplete(needed) => ErrorKind::UnexpectedEof(needed).into(),
        }
    }
}

/// Iterable version that processes a buffer.
struct DeserializerIter<'de> {
    /// Fit data buffer
    buffer: &'de [u8],
    /// Deserializer reference to track state
    deserializer: &'de mut Deserializer,
}

impl<'de> Iterator for DeserializerIter<'de> {
    type Item = Result<FitObject>;

    fn next(&mut self) -> Option<Result<FitObject>> {
        // consume data until buffer is empty or the parsing errors, as far as
        // I know a valid FIT file should have no trailing bytes.
        if self.buffer.is_empty() {
            return None;
        }
        match self.deserializer.deserialize_any(self.buffer) {
            Ok((input, obj)) => {
                self.buffer = input;
                Some(Ok(obj))
            }
            Err(e) => {
                Some(Err(e))
            }
        }
    }
}


/// Decode a stream of FitObjects returning only the data records
fn decode_messages<T: IntoIterator<Item=Result<FitObject>>>(decoder: &mut Decoder, fit_objs: T) -> Result<Vec<FitDataRecord>> {
    fit_objs.into_iter().filter_map(|o| {
        match o {
            Ok(FitObject::Crc(..)) => None,
            Ok(FitObject::Header(..)) => {decoder.reset(); None}
            Ok(FitObject::DataMessage(hdr, msg)) => Some(decoder.decode_message(hdr, msg)),
            Ok(FitObject::DefinitionMessage(..)) => None,
            Err(e) => Some(Err(e))
        }
    }).collect()  // we have to return a vec because of the closure using a decoder ref
}

/// Deserialize a FIT file stored as an array of bytes.
pub fn from_bytes(buffer: &[u8]) -> Result<Vec<FitDataRecord>> {
    let mut deserializer = Deserializer::new();
    let mut decoder = Decoder::new();
    decode_messages(&mut decoder, deserializer.deserialize(buffer))
}

/// Deserialize a FIT file stored in a source that implements io::Read.
pub fn from_reader<T: Read>(source: &mut T) -> Result<Vec<FitDataRecord>> {
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;
    from_bytes(&buffer)
}
