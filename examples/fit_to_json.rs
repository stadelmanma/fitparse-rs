//! Read one or more FIT files and dump their contents as JSON
use fitparser;
use fitparser::ser::{FitDataRecordSerializer, ValueWithUnits};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Parse FIT formatted files and output their data in the JSON format
#[derive(Debug, StructOpt)]
#[structopt(name = "fit_to_json")]
struct Cli {
    /// FIT files to convert to JSON
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    /// Output location, if not provided the JSON file will be output alongside the input file. If a
    /// directory is provided all FIT files will be written there using the same filename but with
    /// a '.json' extension. If multiple FIT files are provided and the output path isn't a
    /// directory the JSON array will store all records present in the order they were read. Using
    /// a "-" as the output file name will result in all content being printed to STDOUT.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(Clone, Debug)]
enum OutputLocation {
    Inplace,
    LocalDirectory(PathBuf),
    LocalFile(PathBuf),
    Stdout,
}

impl OutputLocation {
    fn new(location: PathBuf) -> Self {
        if location.is_dir() {
            OutputLocation::LocalDirectory(location)
        } else if location.as_os_str() == "-" {
            OutputLocation::Stdout
        } else {
            OutputLocation::LocalFile(location)
        }
    }

    fn write_json_file(
        &self,
        filename: &PathBuf,
        data: &[FitDataRecordSerializer<String, String, ValueWithUnits>],
    ) -> std::io::Result<()> {
        let j = serde_json::to_string(data).unwrap();

        let outname = match self {
            Self::Inplace => filename.with_extension("json"),
            Self::LocalDirectory(dest) => dest
                .clone()
                .join(filename.file_name().unwrap())
                .with_extension("json"),
            Self::LocalFile(dest) => dest.clone(),
            Self::Stdout => {
                println!("{}", j);
                return Ok(());
            }
        };

        let mut f = File::create(outname).unwrap();
        f.write_all(j.as_bytes())
    }
}

fn main() {
    let opt = Cli::from_args();
    let output_loc = opt
        .output
        .map_or(OutputLocation::Inplace, OutputLocation::new);
    let collect_all = match output_loc {
        OutputLocation::LocalFile(_) => true,
        _ => false,
    };

    // Read each FIT file and output it
    let mut fit_data: Vec<FitDataRecordSerializer<String, String, ValueWithUnits>> = Vec::new();
    for file in opt.files {
        // open file and parse data
        let mut f = File::open(&file).unwrap();
        for record in fitparser::from_reader(&mut f).unwrap() {
            fit_data.push(record.into_name_key_value_with_units_mapping());
        }

        // output a single fit file's data into a single output file
        if !collect_all {
            output_loc.write_json_file(&file, &fit_data).unwrap();
            fit_data.clear();
        }
    }
    // output fit data from all files into a single file
    if collect_all {
        output_loc
            .write_json_file(&PathBuf::new(), &fit_data)
            .unwrap();
    }
}
