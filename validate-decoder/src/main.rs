//! Parse the FIT SDK profile excel workbook to generate the decoding module.
use fitparser;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

/// Parse a FIT file and a refence file that as generated using the FIT SDK to
/// verify our decoder matches what the SDK outputs
#[derive(Debug, StructOpt)]
#[structopt(name = "validate-decoder")]
struct Cli {
    /// Path to FIT file used to generate reference file
    #[structopt(name = "FIT_FILE", parse(from_os_str))]
    fit_file: PathBuf,

    /// Path to reference file generated from FIT SDK decoder
    #[structopt(name = "REF_FILE", parse(from_os_str))]
    ref_file: PathBuf,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Cli::from_args();

    let mut fp = File::open(&opt.fit_file)?;
    let mut data = fitparser::from_reader(&mut fp)?;
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
