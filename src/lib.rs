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
//! use fitparser::{parse, FitFile};
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! let mut fp = File::open("test/fixtures/Activity.fit")?;
//! for data in parse(&mut fp)? {
//!     // print the data in FIT file
//!     println!("{:#?}", data);
//!     // alternatively reserialize the data into a new format with serde
//!     // println!("{:#?}",  serde_json::to_string(data)?);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
#![warn(missing_docs)]

extern crate nom;
pub mod objects;
pub use objects::{DataField, DataFieldValue, FitDataRecord, FitFile};
pub mod parser;
pub mod profile;
use std::io::Read;

/// Parse FIT data from a readable source and return the result, after applying the defined profile
/// to the parser AST. FIT files can be chained so we return a Vec containing all datasets read from
/// the source.
pub fn parse<T: Read>(source: &mut T) -> Result<Vec<FitFile>, Box<dyn std::error::Error>> {
    let mut fit_data: Vec<FitFile> = Vec::new();
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;

    // process data until no bytes remain or we hit an error
    let mut remaining: &[u8] = &buffer;
    while remaining.len() > 0 {
        match parser::parse(&remaining) {
            Ok((r, ast)) => {
                remaining = r;
                fit_data.push(FitFile::from_ast(ast));
            }
            Err(e) => return Err(Box::new(e.to_owned())),
        };
    }

    Result::Ok(fit_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_activity() {
        let data = include_bytes!("../test/fixtures/Activity.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 22);
    }

    #[test]
    fn parse_developer_data() {
        let data = include_bytes!("../test/fixtures/DeveloperData.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_monitoring_file() {
        let data = include_bytes!("../test/fixtures/MonitoringFile.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 355);
    }

    #[test]
    fn parse_settings() {
        let data = include_bytes!("../test/fixtures/Settings.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 3);
    }

    #[test]
    fn parse_weight_scale_multi_user() {
        let data = include_bytes!("../test/fixtures/WeightScaleMultiUser.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }

    #[test]
    fn parse_weight_scale_single_user() {
        let data = include_bytes!("../test/fixtures/WeightScaleSingleUser.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_custom_target_values() {
        let data = include_bytes!("../test/fixtures/WorkoutCustomTargetValues.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_individual_steps() {
        let data = include_bytes!("../test/fixtures/WorkoutIndividualSteps.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_repeat_greater_than_step() {
        let data = include_bytes!("../test/fixtures/WorkoutRepeatGreaterThanStep.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }

    #[test]
    fn parse_workout_repeat_steps() {
        let data = include_bytes!("../test/fixtures/WorkoutRepeatSteps.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }
}
