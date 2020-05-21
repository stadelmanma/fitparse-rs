//! # FitParser
//!
//! `fitparser` is a utility to parse an ANT FIT file based on a given profile into a more
//! useful form for consuming applications. To that end the [serde](https://github.com/serde-rs/serde)
//! framework is used to allow the data to be serialized into any format supported by serde. This
//! library currently does not support writing FIT files.
//!
//! ## Example
//! Open a file or pass in any other object that implements the Read
//! trait. FIT files can be chained so a single file can contain more than
//! one dataset which is why `parse` returns a Vec.
//! ```
//! use fitparser;
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! let mut fp = File::open("tests/fixtures/Activity.fit")?;
//! let mut buffer = Vec::new();
//! fp.read_to_end(&mut buffer)?;
//! for data in fitparser::from_bytes(&buffer)? {
//!     // print the data in FIT file
//!     println!("{:#?}", data);
//!     // alternatively reserialize the data into a new format with serde
//!     // println!("{:#?}",  serde_json::to_string(data)?);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
#![warn(missing_docs)]
use chrono::{DateTime, Local};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt;

mod de;
mod error;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, ErrorKind, Result};

/// Defines a set of data derived from a FIT Data message.
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecord {
    /// The kind of message the data came from, the FIT profile defines several messages and
    /// custom messages can be defined by altering the profile
    kind: String,
    /// All the fields present in this message, a record may not have every possible field defined
    fields: BTreeMap<String, Value>,
}

impl FitDataRecord {
    /// Create an empty data record with a given kind
    pub fn new(kind: String) -> Self {
        FitDataRecord {
            kind,
            fields: BTreeMap::new(),
        }
    }
}

/// Contains arbitrary data in the defined format. These types are condensed from the full list of
/// possible types defined by the FIT profile
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// Timestamp field converted to the local timezone
    Timestamp(DateTime<Local>),
    /// Unsigned 8bit integer data
    Byte(u8),
    /// Unsigned 8bit integer that gets mapped to a FieldType enum
    Enum(u8),
    /// Signed 8bit integer data
    SInt8(i8),
    /// Unsigned 8bit integer data
    UInt8(u8),
    /// Signed 16bit integer data
    SInt16(i16),
    /// Unsigned 16bit integer data
    UInt16(u16),
    /// Signed 32bit integer data
    SInt32(i32),
    /// Unsigned 32bit integer data
    UInt32(u32),
    /// UTF-8 format string data
    String(String),
    /// 32bit floating point data
    Float32(f32),
    /// 64bit floating point data
    Float64(f64),
    /// Unsigned 8bit integer data where the invalid value is 0x0 instead of 0xFF
    UInt8z(u8),
    /// Unsigned 16bit integer data where the invalid value is 0x0 instead of 0xFFFF
    UInt16z(u16),
    /// Unsigned 16bit integer data where the invalid value is 0x0 instead of 0xFFFFFFFF
    UInt32z(u32),
    /// Signed 64bit integer data
    SInt64(i64),
    /// Unsigned 64bit integer data
    UInt64(u64),
    /// Unsigned 64bit integer data where the invalid value is 0x0 instead of 0xFFFFFFFFFFFFFFFF
    UInt64z(u64),
    /// Array of DataFieldValue, while this allows nested arrays and mixed types this is not possible
    /// in a properly formatted FIT file
    Array(Vec<Self>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Value::Timestamp(val) => write!(f, "{}", val),
            Value::Byte(val) => write!(f, "{}", val),
            Value::Enum(val) => write!(f, "{}", val),
            Value::SInt8(val) => write!(f, "{}", val),
            Value::UInt8(val) => write!(f, "{}", val),
            Value::UInt8z(val) => write!(f, "{}", val),
            Value::SInt16(val) => write!(f, "{}", val),
            Value::UInt16(val) => write!(f, "{}", val),
            Value::UInt16z(val) => write!(f, "{}", val),
            Value::SInt32(val) => write!(f, "{}", val),
            Value::UInt32(val) => write!(f, "{}", val),
            Value::UInt32z(val) => write!(f, "{}", val),
            Value::SInt64(val) => write!(f, "{}", val),
            Value::UInt64(val) => write!(f, "{}", val),
            Value::UInt64z(val) => write!(f, "{}", val),
            Value::Float32(val) => write!(f, "{}", val),
            Value::Float64(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Array(vals) => write!(f, "{:?}", vals), // printing arrays is hard
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_activity() {
        let data = include_bytes!("../tests/fixtures/Activity.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 22);
    }

    #[test]
    fn parse_developer_data() {
        let data = include_bytes!("../tests/fixtures/DeveloperData.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_monitoring_file() {
        let data = include_bytes!("../tests/fixtures/MonitoringFile.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 355);
    }

    #[test]
    fn parse_settings() {
        let data = include_bytes!("../tests/fixtures/Settings.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 3);
    }

    #[test]
    fn parse_weight_scale_multi_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleMultiUser.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_weight_scale_single_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleSingleUser.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_custom_target_values() {
        let data = include_bytes!("../tests/fixtures/WorkoutCustomTargetValues.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_individual_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutIndividualSteps.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_repeat_greater_than_step() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatGreaterThanStep.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_workout_repeat_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatSteps.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_garmin_fenix_5_bike() {
        // this test case includes a FIT file with a string field, which was broken in v0.1.0
        // and fixed in v0.1.1
        let data = include_bytes!("../tests/fixtures/garmin-fenix-5-bike.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 143);
    }

    #[test]
    fn parse_sample_mulitple_header() {
        // this test case includes a chained FIT file
        let data = include_bytes!("../tests/fixtures/sample_mulitple_header.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 3023);
    }
}
