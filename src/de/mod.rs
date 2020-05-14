//! Deserialize a stream of FIT file data into the serde data model by parsing the file and
//! applying the packaged FIT profile to the data.
use crate::error::{Error, ErrorKind, Result};
use serde::de;
use std::collections::HashMap;
use std::io;

mod parser;
use parser;

// I'll probably track the definition messsages in Deserializer as well as
// field accumlations since it will probably make sense to apply the
// profile as we go since I have a struct to track the current context.

// it would be nice to check the CRC on the fly, if I use some kind of buffered reader
// for certain sections that might be possible since I could build the CRC as chunks
// of the file are read

/// Stores data and manages the deserialization of a FIT data stream into Rust constructs
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
    definitions: HashMap<u8, FitDefinitionMessage>
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
            definitions: HashMap::new()
        }
    }

    /// Deserialize a FIT file stored as an array of bytes
    fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer::new(input)
    }

    ///  Deserialize a FIT file stored in a readable source
    fn from_reader<T: io::Read>(source: T) -> Self {
        // I'll also need to export a pub interface like I do for `from_bytes`
        todo!();
    }

    /// Parse the FIT header
    pub fn parse_header(&mut self) -> Result<()> {
        let (input, header) = parser::fit_file_header(self.input)?;
        self.input = input;
        self.header_size = header.header_size();
        self.data_size = header.data_size();
        self.protocol_ver_enc = header.protocol_ver_enc();
        self.profile_ver_enc = header.profile_ver_enc();

        Ok(())
    }
}

/// Deserialize a FIT file stored as an array of bytes
pub fn from_bytes<'a, T>(input: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    // create deserializer and parse header data that comes before the first messages.
    let mut deserializer = Deserializer::from_bytes(input);
    deserializer.parse_header()?;

    // In here I need to replace the call to `T::deserialize` with my own logic since
    // FIT files cannot really encode arbitrary data structures. They always encode a sequence
    // of messages and each message is essentially a map with `name: {value, units}`.
    // It probably makes sense to call deserialize_seq first since we have a sequence
    // of messages. Then we will want to call deserialize_map on the data messages.
    // Lastly we will use deserialize_struct for each data field, we will also apply
    // the FIT profile here to get field info and do proper conversions.
    // Due to the message `kind` field, data messages need to be a struct as well I suppose.

    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(ErrorKind::TrailingBytes(deserializer.input.len()).into())
    }
}

macro_rules! deserialize_not_supported {
    ($method:ident) => {
        #[inline]
        fn $method<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            Err(Box::new(ErrorKind::NotSupported(stringify!($method).into())))
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    deserialize_not_supported!(deserialize_any);
    deserialize_not_supported!(deserialize_bool);
    deserialize_not_supported!(deserialize_i8);
    deserialize_not_supported!(deserialize_i16);
    deserialize_not_supported!(deserialize_i32);
    deserialize_not_supported!(deserialize_i64);
    deserialize_not_supported!(deserialize_u8);
    deserialize_not_supported!(deserialize_u16);
    deserialize_not_supported!(deserialize_u32);
    deserialize_not_supported!(deserialize_u64);
    deserialize_not_supported!(deserialize_f32);
    deserialize_not_supported!(deserialize_f64);
    deserialize_not_supported!(deserialize_char);
    deserialize_not_supported!(deserialize_str);
    deserialize_not_supported!(deserialize_string);
    deserialize_not_supported!(deserialize_bytes);
    deserialize_not_supported!(deserialize_byte_buf);
    deserialize_not_supported!(deserialize_option);
    deserialize_not_supported!(deserialize_unit);
    deserialize_not_supported!(deserialize_seq);
    deserialize_not_supported!(deserialize_map);
    deserialize_not_supported!(deserialize_identifier);
    deserialize_not_supported!(deserialize_ignored_any);

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported(
            "deserialize_unit_struct".into(),
        )))
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported(
            "deserialize_newtype_struct".into(),
        )))
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported(
            "deserialize_tuple".into(),
        )))
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported(
            "deserialize_tuple_struct".into(),
        )))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported(
            "deserialize_struct".into(),
        )))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::NotSupported("deserialize_enum".into())))
    }
}
