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
//! let mut fp = File::open("tests/fixtures/Activity.fit")?;
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

mod de;
mod error;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, ErrorKind, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_activity() {
        let data = include_bytes!("../tests/fixtures/Activity.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 22);
    }

    #[test]
    fn parse_developer_data() {
        let data = include_bytes!("../tests/fixtures/DeveloperData.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_monitoring_file() {
        let data = include_bytes!("../tests/fixtures/MonitoringFile.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 355);
    }

    #[test]
    fn parse_settings() {
        let data = include_bytes!("../tests/fixtures/Settings.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 3);
    }

    #[test]
    fn parse_weight_scale_multi_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleMultiUser.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }

    #[test]
    fn parse_weight_scale_single_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleSingleUser.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_custom_target_values() {
        let data = include_bytes!("../tests/fixtures/WorkoutCustomTargetValues.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_individual_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutIndividualSteps.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 6);
    }

    #[test]
    fn parse_workout_repeat_greater_than_step() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatGreaterThanStep.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }

    #[test]
    fn parse_workout_repeat_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatSteps.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 7);
    }

    #[test]
    fn parse_garmin_fenix_5_bike() {
        // this test case includes a FIT file with a string field, which was broken in v0.1.0
        // and fixed in v0.1.1
        let data = include_bytes!("../tests/fixtures/garmin-fenix-5-bike.fit").to_vec();
        let mut buf = Cursor::new(&data);
        let fitfile = &parse(&mut buf).unwrap()[0];
        assert_eq!(fitfile.records.len(), 143);
    }
}
