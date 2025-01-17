//! # FitParser
//!
//! `fitparser` is a utility to parse an ANT FIT file based on a given profile into a more
//! useful form for consuming applications. To that end the [serde](https://github.com/serde-rs/serde)
//! framework is used to allow the data to be serialized into any format supported by serde. This
//! library currently does not support writing FIT files.
//!
//! ## Example
//! Open a file or pass in any other object that implements the Read
//! trait. A vector of data records is returned if deserialization is
//! successful. See the `fit_to_json` example for a command line utility
//! that parses FIT files and exports them as JSON.
//! ```
//! use fitparser;
//! use fitparser::de::{DecodeOption, from_reader_with_options};
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! println!("Parsing FIT files using Profile version: {:?}", fitparser::profile::VERSION);
//! let mut fp = File::open("tests/fixtures/Activity.fit")?;
//! for data in fitparser::from_reader(&mut fp)? {
//!     // print the data in FIT file
//!     println!("{:#?}", data);
//! }
//!
//! // Optionally ignore CRC validation
//! let opts = [DecodeOption::SkipHeaderCrcValidation,
//!             DecodeOption::SkipDataCrcValidation].iter().map(|o| *o).collect();
//! let mut fp = File::open("tests/fixtures/Activity.fit")?;
//! for data in from_reader_with_options(&mut fp, &opts)? {
//!     // print the data in FIT file
//!     println!("{:#?}", data);
//! }
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
#![warn(missing_docs)]
use chrono::{DateTime, Local};
use profile::field_types::FitBaseType;
use serde::Serialize;
use std::collections::HashMap;
use std::convert;
use std::fmt;

pub mod de;
mod error;
pub mod profile;

pub use de::{from_bytes, from_reader};
pub use error::{Error, ErrorKind, Result};

/// Defines a set of data derived from a FIT Data message.
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecord {
    /// The kind of message the data came from, the FIT profile defines several messages and
    /// custom messages can be defined by altering the profile
    kind: profile::MesgNum,
    /// All the fields present in this message, a record may not have every possible field defined
    fields: Vec<FitDataField>,
}

impl FitDataRecord {
    /// Create an empty data record with a given kind
    pub fn new(kind: profile::MesgNum) -> Self {
        FitDataRecord {
            kind,
            fields: Vec::new(),
        }
    }

    /// Return the kind of FitDataRecord, this value is defined by the FIT profile.
    pub fn kind(&self) -> profile::MesgNum {
        self.kind
    }

    /// Get all fields as a slice
    pub fn fields(&self) -> &[FitDataField] {
        &self.fields
    }

    /// Add a field to the record
    pub fn push(&mut self, field: FitDataField) {
        self.fields.push(field)
    }

    /// Add multiple fields to the record
    pub fn extend(&mut self, fields: Vec<FitDataField>) {
        self.fields.extend(fields)
    }

    /// Consume the record and return the field vector for further processing
    pub fn into_vec(self) -> Vec<FitDataField> {
        self.fields
    }
}

/// Stores a value and it's defined units which are set by the FIT profile during decoding
#[derive(Clone, Debug, Serialize)]
pub struct FitDataField {
    name: String,
    number: u8,
    developer_data_index: Option<u8>, // None for built-in fields, for developer fields identifies which developer
    value: Value,
    units: String,
}

impl FitDataField {
    /// Create a new FitDataField
    pub fn new(
        name: String,
        number: u8,
        developer_data_index: Option<u8>,
        value: Value,
        units: String,
    ) -> Self {
        FitDataField {
            name,
            number,
            developer_data_index,
            value,
            units,
        }
    }

    /// Return the field name as defined in the FIT profile
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the field definition number
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Return a reference to the stored value
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Return units associated with the value
    pub fn units(&self) -> &str {
        &self.units
    }

    /// Consume the field and return the value
    pub fn into_value(self) -> Value {
        self.value
    }
}

impl fmt::Display for FitDataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.units.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.units)
        }
    }
}

/// Contains arbitrary data in the defined format.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// Timestamp field converted to the local timezone
    Timestamp(DateTime<Local>),
    /// Unsigned 8bit integer data
    Byte(u8), // TODO: I think this should actually be a Vec<u8> type
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
    /// Unsigned 8bit integer data where the invalid value is `0x0` instead of `0xFF`
    UInt8z(u8),
    /// Unsigned 16bit integer data where the invalid value is `0x0` instead of `0xFFFF`
    UInt16z(u16),
    /// Unsigned 16bit integer data where the invalid value is `0x0` instead of `0xFFFFFFFF`
    UInt32z(u32),
    /// Signed 64bit integer data
    SInt64(i64),
    /// Unsigned 64bit integer data
    UInt64(u64),
    /// Unsigned 64bit integer data where the invalid value is `0x0` instead of `0xFFFFFFFFFFFFFFFF`
    UInt64z(u64),
    /// Array of Values, while this allows nested arrays and mixed types this is not possible
    /// in a properly formatted FIT file
    Array(Vec<Self>),
    /// Placeholder for invalid values in output
    Invalid,
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
            Value::Array(vals) => write!(f, "{:?}", vals), // printing arrays is hard,
            Value::Invalid => write!(f, ""),
        }
    }
}

impl convert::TryInto<f64> for Value {
    type Error = error::Error;

    fn try_into(self) -> Result<f64> {
        match self {
            Value::Timestamp(val) => Ok(val.timestamp() as f64),
            Value::Byte(val) => Ok(val as f64),
            Value::Enum(val) => Ok(val as f64),
            Value::SInt8(val) => Ok(val as f64),
            Value::UInt8(val) => Ok(val as f64),
            Value::UInt8z(val) => Ok(val as f64),
            Value::SInt16(val) => Ok(val as f64),
            Value::UInt16(val) => Ok(val as f64),
            Value::UInt16z(val) => Ok(val as f64),
            Value::SInt32(val) => Ok(val as f64),
            Value::UInt32(val) => Ok(val as f64),
            Value::UInt32z(val) => Ok(val as f64),
            Value::SInt64(val) => Ok(val as f64),
            Value::UInt64(val) => Ok(val as f64),
            Value::UInt64z(val) => Ok(val as f64),
            Value::Float32(val) => Ok(val as f64),
            Value::Float64(val) => Ok(val),
            Value::String(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an f64", self)).into())
            }
            Value::Array(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an f64", self)).into())
            }
            Value::Invalid => Err(ErrorKind::ValueError(
                "cannot convert an invalid value into an f64".to_string(),
            )
            .into()),
        }
    }
}

impl convert::TryInto<i64> for Value {
    type Error = error::Error;

    fn try_into(self) -> Result<i64> {
        match self {
            Value::Timestamp(val) => Ok(val.timestamp()),
            Value::Byte(val) => Ok(val as i64),
            Value::Enum(val) => Ok(val as i64),
            Value::SInt8(val) => Ok(val as i64),
            Value::UInt8(val) => Ok(val as i64),
            Value::UInt8z(val) => Ok(val as i64),
            Value::SInt16(val) => Ok(val as i64),
            Value::UInt16(val) => Ok(val as i64),
            Value::UInt16z(val) => Ok(val as i64),
            Value::SInt32(val) => Ok(val as i64),
            Value::UInt32(val) => Ok(val as i64),
            Value::UInt32z(val) => Ok(val as i64),
            Value::SInt64(val) => Ok(val),
            Value::UInt64(val) => Ok(val as i64),
            Value::UInt64z(val) => Ok(val as i64),
            Value::Float32(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Float64(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::String(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Array(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Invalid => Err(ErrorKind::ValueError(
                "cannot convert an invalid value into an i64".to_string(),
            )
            .into()),
        }
    }
}

impl convert::TryInto<i64> for &Value {
    type Error = error::Error;

    fn try_into(self) -> Result<i64> {
        match self {
            Value::Timestamp(val) => Ok(val.timestamp()),
            Value::Byte(val) => Ok(*val as i64),
            Value::Enum(val) => Ok(*val as i64),
            Value::SInt8(val) => Ok(*val as i64),
            Value::UInt8(val) => Ok(*val as i64),
            Value::UInt8z(val) => Ok(*val as i64),
            Value::SInt16(val) => Ok(*val as i64),
            Value::UInt16(val) => Ok(*val as i64),
            Value::UInt16z(val) => Ok(*val as i64),
            Value::SInt32(val) => Ok(*val as i64),
            Value::UInt32(val) => Ok(*val as i64),
            Value::UInt32z(val) => Ok(*val as i64),
            Value::SInt64(val) => Ok(*val),
            Value::UInt64(val) => Ok(*val as i64),
            Value::UInt64z(val) => Ok(*val as i64),
            Value::Float32(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Float64(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::String(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Array(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Invalid => Err(ErrorKind::ValueError(
                "cannot convert an invalid value into an i64".to_string(),
            )
            .into()),
        }
    }
}

/// Describes a field value along with its defined units (if any), this struct is useful for
/// serializing data in a key-value store where the key is either the name or definition number
/// since it can be created from a `FitDataField` with minimal data cloning.
#[derive(Clone, Debug, Serialize)]
pub struct ValueWithUnits {
    value: Value,
    units: String,
}

impl ValueWithUnits {
    /// Create a new value with the given information
    pub fn new(value: Value, units: String) -> Self {
        ValueWithUnits { value, units }
    }
}

impl convert::From<FitDataField> for ValueWithUnits {
    fn from(field: FitDataField) -> Self {
        ValueWithUnits::new(field.value, field.units)
    }
}

impl fmt::Display for ValueWithUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.units.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.units)
        }
    }
}

/// Developer fields are fields with properties that are not already defined
/// by the SDK, but instead the definition is part of the fit file, for more
/// flexiblity. Since their properties are dynamically defined and cannot be
/// looked up statically we need to store these properties. This is what the
/// DeveloperFieldDescription struct is for.
///
#[derive(Debug)]
pub struct DeveloperFieldDescription {
    developer_data_index: u8,
    field_definition_number: u8,
    fit_base_type_id: FitBaseType,
    field_name: String,
    scale: f64,
    offset: f64,
    units: String,
}

impl TryFrom<&Vec<FitDataField>> for DeveloperFieldDescription {
    type Error = ErrorKind;

    /// Create DeveloperFieldDescription from FitDataFields
    fn try_from(
        fields: &Vec<FitDataField>,
    ) -> std::result::Result<DeveloperFieldDescription, error::ErrorKind> {
        let mut name_to_value = HashMap::new();
        for field in fields {
            name_to_value.insert(field.name.clone(), field.value.clone());
        }
        let developer_data_index = if let Value::UInt8(developer_data_index) = name_to_value
            .get("developer_data_index")
            .ok_or(ErrorKind::ValueError(
                "developer_data_index is mandatory".to_string(),
            ))? {
            *developer_data_index
        } else {
            return Err(ErrorKind::ValueError(
                "developer_data_index must be u8".to_string(),
            ));
        };
        let field_definition_number = if let Value::UInt8(field_definition_number) = name_to_value
            .get("field_definition_number")
            .ok_or(ErrorKind::ValueError(
                "field_definition_number is mandatory".to_string(),
            ))? {
            *field_definition_number
        } else {
            return Err(ErrorKind::ValueError(
                "field_definition_number must be u8".to_string(),
            ));
        };
        let fit_base_type_id = if let Value::String(fit_base_type_id) = name_to_value
            .get("fit_base_type_id")
            .ok_or(ErrorKind::ValueError(
                "fit_base_type_id is mandatory".to_string(),
            ))? {
            // Since the decoder turns enums to string, we need to undo it to get FitBaseType back
            FitBaseType::from(fit_base_type_id as &str)
        } else {
            return Err(ErrorKind::ValueError(
                "fit_base_type_id must be string".to_string(),
            ));
        };
        let field_name = if let Value::String(field_name) = name_to_value
            .get("field_name")
            .unwrap_or(&Value::String(format!(
                "unknown_developer_field_{}",
                field_definition_number
            ))) {
            field_name.clone()
        } else {
            return Err(ErrorKind::ValueError(
                "field_name must be string".to_string(),
            ));
        };
        let scale =
            if let Value::UInt8(scale) = name_to_value.get("scale").unwrap_or(&Value::UInt8(1u8)) {
                *scale as f64
            } else {
                return Err(ErrorKind::ValueError("scale must be u8".to_string()));
            };
        let offset = if let Value::SInt8(offset) =
            name_to_value.get("offset").unwrap_or(&Value::SInt8(0i8))
        {
            *offset as f64
        } else {
            return Err(ErrorKind::ValueError("offset must be i8".to_string()));
        };
        let units = if let Value::String(units) = name_to_value
            .get("units")
            .unwrap_or(&Value::String(String::new()))
        {
            units.clone()
        } else {
            return Err(ErrorKind::ValueError("units must be string".to_string()));
        };

        Ok(DeveloperFieldDescription {
            developer_data_index,
            field_definition_number,
            fit_base_type_id,
            field_name,
            scale,
            offset,
            units,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
        // make sure we correctly parsed the dev data
        assert_eq!(fit_data[3].fields().len(), 5);
        assert_eq!(fit_data[3].fields()[4].name(), "doughnuts_earned");
        assert_eq!(fit_data[3].fields()[4].value(), &Value::SInt8(1));
        assert_eq!(fit_data[3].fields()[4].units(), "doughnuts");
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

    #[test]
    fn parse_activity_with_hrv() {
        // this test case includes a chained FIT file
        let data = include_bytes!("../tests/fixtures/hrv-activity.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 2260);
        // find the first HRV message and check the array
        // we should see a partial set of bytes
        let field = fit_data
            .into_iter()
            .find(|rec| rec.kind == profile::MesgNum::Hrv)
            .map(|rec| rec.fields)
            .expect("We should have at least one HRV message")
            .into_iter()
            .find(|fld| fld.name == "time")
            .expect("We should have at least one time field");
        assert_eq!(
            field.value,
            Value::Array(vec![
                Value::Float64(0.467),
                Value::Float64(0.464),
                Value::Invalid,
                Value::Invalid,
                Value::Invalid
            ])
        );
    }

    #[test]
    fn parse_with_header_crc_set_to_zero() {
        // Set header CRC to zero so that the CRC at the EOF includes all bytes
        let mut data = include_bytes!("../tests/fixtures/garmin-fenix-5-bike.fit").to_vec();
        let leng = data.len();
        data[12] = 0x00;
        data[13] = 0x00;
        match de::from_bytes(&data) {
            Ok(_) => panic!(
                "This test should fail without the data CRC value being recomputed to include the header."
            ),
            Err(e) => match *e {
                ErrorKind::InvalidCrc(..) => {}
                _ => panic!("Incorrect error returned {:?}", e),
            },
        }

        // update data CRC value so that it includes the header
        data[leng - 2] = 0x58;
        data[leng - 1] = 0x65;
        let fit_data = de::from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 143);
    }

    #[test]
    fn parse_with_invalid_header_crc_with_options() {
        // force header CRC to be an invalid, non-zero value
        let mut data = include_bytes!("../tests/fixtures/MonitoringFile.fit").to_vec();
        data[12] = 0xFF;
        data[13] = 0xFF;
        let mut options = HashSet::new();
        match de::from_bytes_with_options(&data, &options) {
            Ok(_) => panic!("This test should fail without the SkipHeaderCrcValidation option."),
            Err(e) => match *e {
                ErrorKind::InvalidCrc(..) => {}
                _ => panic!("Incorrect error returned {:?}", e),
            },
        }

        // add proper option to decode file
        options.insert(de::DecodeOption::SkipHeaderCrcValidation);
        let fit_data = de::from_bytes_with_options(&data, &options).unwrap();
        assert_eq!(fit_data.len(), 355);
    }

    #[test]
    fn parse_with_invalid_data_crc_with_options() {
        // force data CRC to be an invalid value
        let mut data = include_bytes!("../tests/fixtures/MonitoringFile.fit").to_vec();
        let leng = data.len();
        data[leng - 2] = 0xFF;
        data[leng - 1] = 0xFF;
        let mut options = HashSet::new();
        match de::from_bytes_with_options(&data, &options) {
            Ok(_) => panic!("This test should fail without the SkipDataCrcValidation option."),
            Err(e) => match *e {
                ErrorKind::InvalidCrc(..) => {}
                _ => panic!("Incorrect error returned {:?}", e),
            },
        }

        // add proper option to decode file
        options.insert(de::DecodeOption::SkipDataCrcValidation);
        let fit_data = de::from_bytes_with_options(&data, &options).unwrap();
        assert_eq!(fit_data.len(), 355);
    }
}
