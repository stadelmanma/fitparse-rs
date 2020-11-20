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
    DataMessage(parser::FitMessageHeader, parser::FitDataMessage),
    /// A definition message used to define upcoming data messages
    DefinitionMessage(parser::FitMessageHeader, parser::FitDefinitionMessage),
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
    end_of_messages: usize
}

impl Deserializer {
    /// Create the deserializer from something "readable"
    pub fn new() -> Self {
        Deserializer {
            definitions: HashMap::new(),
            position: 0,
            end_of_messages: 0,
        }
    }

    /// Create an iterable that will process the provided byte slice into a set of
    /// FitObjects.
    pub fn deserialize<'de>(&'de mut self, input: &'de [u8]) -> DeserializerIter<'de> {
        DeserializerIter { buffer: input, deserializer: self }
    }

    /// Advance the parser state returning one of four possible objects defined within the
    /// FIT file.
    pub fn deserialize_any<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
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
        let (input, header) = parser::fit_file_header(input)?;
        self.end_of_messages = self.position + header.header_size() as usize + header.data_size() as usize;
        self.position += header.header_size() as usize;

        Ok((input, FitObject::Header(header)))
    }

    /// Extract a 2 byte CRC
    fn deserialize_crc<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        let (input, crc) = le_u16(input)?;
        self.position += 2;
        Ok((input, FitObject::Crc(crc)))
    }

    /// Parse a FIT data or definition message
    fn deserialize_message<'de>(&mut self, input: &'de [u8]) -> Result<(&'de [u8], FitObject)> {
        // parse a single message of either variety
        let init_len = input.len();
        let (input, header) = parser::message_header(input)?;
        match header.message_type() {
            parser::FitMessageType::Data => {
                if let Some(def_mesg) = self.definitions.get(&header.local_message_type()) {
                    let (input, message) = parser::data_message(input, &def_mesg)?;
                    self.position += init_len - input.len();
                    Ok((input, FitObject::DataMessage(header, message)))
                } else {
                    Err(ErrorKind::MissingDefinitionMessage(
                        header.local_message_type(),
                        self.position,
                    )
                    .into())
                }
            }
            parser::FitMessageType::Definition => {
                let (input, message) = parser::definition_message(input, &header)?;
                self.definitions
                    .insert(header.local_message_type(), message.clone());
                self.position += init_len - input.len();
                Ok((input, FitObject::DefinitionMessage(header, message)))
            }
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
            return None
        }
        match self.deserializer.deserialize_any(self.buffer) {
            Ok((input, obj)) => {
                self.buffer = input;
                Some(Ok(obj))
            },
            Err(e) => {
                // this works now but I need to return the remaining data
                // or position that the error occured since otherwise I don't know where the problem
                // happened and I can't recover from it. This is an issue that's existed since the
                // initial creation of the parser. I could also copy the remaining bytes and return
                // them with the error. Not as "efficient" but the parsing as already failed so...
                Some(Err(e))
            }
        }
    }
}

/// Deserialize a FIT file stored as an array of bytes.
pub fn from_bytes(buffer: &[u8]) -> Result<Vec<FitDataRecord>> {
    let mut deserializer = Deserializer::new();
    let mut decoder = Decoder::new();
    decoder.decode_messages(deserializer.deserialize(buffer))
}

/// Deserialize a FIT file stored in a source that implements io::Read.
pub fn from_reader<T: Read>(source: &mut T) -> Result<Vec<FitDataRecord>> {
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;
    from_bytes(&buffer)
}
