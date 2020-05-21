//! Deserialize a stream of FIT file data into the serde data model by parsing the file and
//! applying the packaged FIT profile to the data.
use crate::{FitDataRecord, Value};
use crate::error::{Error, ErrorKind, Result};
use nom::number::complete::le_u16;
use std::collections::HashMap;
use core::iter::Iterator;
use std::io;

mod parser;

use nom::bytes::complete::take; // temporary

/// Stores a definition message or a DataMessage
#[derive(Clone, Debug)]
pub enum FitMessage {
    /// A raw data message
    Data(parser::FitMessageHeader, parser::FitDataMessage),
    /// A definition message used to define upcoming data messages
    Definition(parser::FitMessageHeader, parser::FitDefinitionMessage),
}

/// Stores data and manages the deserialization of a FIT data stream into Rust constructs
#[derive(Debug)]
pub struct Deserializer<'de> {
    // These fields describe FIT file info that may be of use.
    /// Length of header in bytes, should be either 12 or 14
    header_size: u8,
    /// Length of the Data Records section in bytes
    data_size: u32,
    /// Protocol version number as provided in SDK
    protocol_ver_enc: f32,
    /// Profile version number as provided in SDK
    profile_ver_enc: f32,

    // These fields deal with the current parser state
    /// input data stream, bytes are shifted from the front as data gets processed.
    /// TODO: Eventually, this could just be something "readable"
    input: &'de [u8],
    /// Track the current set of FIT message definitions, these are what allows the format to
    /// be self describing.
    definitions: HashMap<u8, parser::FitDefinitionMessage>,
    /// stores the current position in the byte stream, this is needed for error generation and
    /// checking the state of the parser
    position: usize,
    /// stores the location that the current FIT message ends, for chained FIT messges this will
    /// be updated to reflect the new end position
    end_of_messages: usize
}

impl<'de> Deserializer<'de> {
    /// Create the deserializer from an input byte array
    fn new(input: &'de [u8]) -> Self {
        // initialize the deserializer with the remaining input data and header info
        Deserializer {
            header_size: 0,
            data_size: 0,
            protocol_ver_enc: 0.0,
            profile_ver_enc: 0.0,
            input,
            definitions: HashMap::new(),
            position: 0,
            end_of_messages: 0,
        }
    }

    /// Deserialize a FIT file stored as an array of bytes
    fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer::new(input)
    }

    // ///  Deserialize a FIT file stored in a readable source
    // fn from_reader<T: io::Read>(source: T) -> Self {
    //     // I'll also need to export a pub interface like I do for `from_bytes`
    //     todo!();
    // }

    /// Parse the FIT header
    fn parse_header(&mut self) -> Result<()> {
        let (input, header) = parser::fit_file_header(self.input)?;
        self.input = input;
        self.header_size = header.header_size();
        self.data_size = header.data_size();
        self.protocol_ver_enc = header.protocol_ver_enc();
        self.profile_ver_enc = header.profile_ver_enc();

        self.end_of_messages = self.position + self.header_size as usize + self.data_size as usize;
        self.position += self.header_size as usize;

        Ok(())
    }

    /// Extract a 2 byte CRC
    fn parse_crc(&mut self) -> Result<u16> {
        let (input, crc) = le_u16(self.input)?;
        self.input = input;
        self.position += 2;
        Ok(crc)
    }

    /// Parse and decode the next data record
    fn get_next_message(&mut self) -> Result<FitMessage> {
        // parse a single message of eithe variety
        let init_len = self.input.len();
        let (input, header) = parser::message_header(self.input)?;
        match header.message_type() {
            parser::FitMessageType::Data => {
                if let Some(def_mesg) = self.definitions.get(&header.local_message_type()) {
                    let (input, message) = parser::data_message(input, &def_mesg)?;
                    self.input = input;
                    self.position += init_len - input.len();
                    return Ok(FitMessage::Data(header, message));

                }
                else {
                    return Err(ErrorKind::MissingDefinitionMessage(header.local_message_type(), self.position).into());
                }
            },
            parser::FitMessageType::Definition => {
                let (input, message) = parser::definition_message(input, &header)?;
                self.definitions.insert(header.local_message_type(), message.clone());
                self.input = input;
                self.position += init_len - input.len();
                return Ok(FitMessage::Definition(header, message))
            }
        }
    }
}

impl<'de> Iterator for Deserializer<'de> {
    type Item = Result<FitDataRecord>;

    // NOTE: this currently sucks because I can't elegantly deal with errors
    // this could  be useful, if I move the conditionals checking my parser position into get_next_message()
    // so that I'm always returning a result?
    // https://play.rust-lang.org/?gist=aa4ef1fe3a523aaa5b7cf90fd71b9b28&version=stable&backtrace=0

    fn next(&mut self) -> Option<Result<FitDataRecord>> {
        if self.position > 0 && self.position == self.end_of_messages {
            // extract the CRC, eventually we'd want to validate it
            match self.parse_crc() {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }
        }
        if self.position == 0 || (self.position > self.end_of_messages && self.input.len() > 0) {
            // if there is extra bytes remaining the FIT file must be chained so we parse
            // the header and continue on
            match self.parse_header() {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }
        }
        if self.input.len() == 0 {
            return None;
        }

        // loop over messages until we get a data message, valid FIT files always end with a
        // data message as far as I know
        loop {
            match self.get_next_message() {
                Ok(fit_message) => {
                    if let FitMessage::Data(header, message) = fit_message {
                        // todo decode fields via profile
                        // todo check for compressed timestamp
                        return Some(Ok(FitDataRecord::new("todo".to_string())));
                    }
                },
                Err(e) => return Some(Err(e))
            }
        }
    }
}

/// Deserialize a FIT file stored as an array of bytes
pub fn from_bytes<'a>(input: &'a [u8]) -> Result<Vec<FitDataRecord>> {
    // create deserializer and parse header data that comes before the first messages.
    let mut deserializer = Deserializer::from_bytes(input);
    deserializer.into_iter().collect()
}
