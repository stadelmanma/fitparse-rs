extern crate nom;
pub mod objects;
pub use objects::FitFile;
pub mod parser;
pub mod profile;
use std::io::Read;

/// Parse FIT data from a readable source and return the result, after applying the defined profile
/// to the parser AST.
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
            },
            Err(e) => return Err(Box::new(e.to_owned()))
        };
    }

    Result::Ok(fit_data)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_activity() {
        let data = include_bytes!("../test/fixtures/Activity.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 22);
        assert_eq!(r.len(), 0);
    }

    #[test]
    #[ignore]
    fn parse_developer_data() {
        let data = include_bytes!("../test/fixtures/DeveloperData.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 6);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_monitoring_file() {
        let data = include_bytes!("../test/fixtures/MonitoringFile.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 355);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_settings() {
        let data = include_bytes!("../test/fixtures/Settings.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 3);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_weight_scale_multi_user() {
        let data = include_bytes!("../test/fixtures/WeightScaleMultiUser.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 7);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_weight_scale_single_user() {
        let data = include_bytes!("../test/fixtures/WeightScaleSingleUser.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 6);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_workout_custom_target_values() {
        let data = include_bytes!("../test/fixtures/WorkoutCustomTargetValues.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 6);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_workout_individual_steps() {
        let data = include_bytes!("../test/fixtures/WorkoutIndividualSteps.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 6);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_workout_repeat_greater_than_step() {
        let data = include_bytes!("../test/fixtures/WorkoutRepeatGreaterThanStep.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 7);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn parse_workout_repeat_steps() {
        let data = include_bytes!("../test/fixtures/WorkoutRepeatSteps.fit");
        let (r, fitfile) = parse_file(data).unwrap();
        assert_eq!(fitfile.records.len(), 7);
        assert_eq!(r.len(), 0);
    }
}
