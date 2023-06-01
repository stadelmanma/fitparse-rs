//! Parse the FIT SDK profile excel workbook to generate the decoding module.
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

mod decode;
use crate::decode::write_decode_file;
mod field_types;
use crate::field_types::write_types_file;
mod parse;
use crate::parse::parse_profile;

/// Parse the Profile.xlsx included in the FIT SDK and update the related modules
#[derive(Debug, StructOpt)]
#[structopt(name = "update_profile")]
struct Cli {
    /// Path to Profile.xlsx file
    #[structopt(name = "FILE", parse(from_os_str))]
    profile_path: PathBuf,

    /// Manually specify the SDK version, usually we can infer this from the path to the Profile.xlsx
    /// file unless it's been moved
    #[structopt(long)]
    sdk_version: Option<String>,
}

/// call rustfmt on a generated file to cleanup auto-gen code
fn rustfmt(fname: &PathBuf) {
    Command::new("rustfmt")
        .arg(fname)
        .status()
        .unwrap_or_else(|_| panic!("failed to execute rustfmt on {fname:?}"));
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Cli::from_args();
    let profile_fname = opts.profile_path;
    let profile_vers = opts.sdk_version.map_or_else(
        || {
            profile_fname
                .parent()
                .and_then(std::path::Path::file_name)
                .map_or_else(
                    || String::from("unknown"),
                    |dirname| {
                        dirname
                            .to_str()
                            .expect("Unable to convert dirname to str")
                            .replace("FitSDKRelease_", "")
                    },
                )
        },
        |vers| vers,
    );
    assert!(
        profile_vers.chars().all(|c| c.is_ascii_digit() || c == '.'),
        "Could not determine version from Profile.xslx path: '{profile_fname:?}' - %{profile_vers}%"
    );

    // process excel file and output
    // let profile = parse_profile(&profile_fname, profile_vers).unwrap();
    let profile = parse_profile(&profile_fname, profile_vers).expect("Failed to parse profile");

    let dest_dir = Path::new("./fitparser/src/profile");
    let types_fname = dest_dir.join("field_types.rs");
    eprintln!("Generating file: {:?}", &types_fname);
    let mut out_file = File::create(&types_fname)?;
    write_types_file(&profile, &mut out_file)?;
    rustfmt(&types_fname);

    let decode_fname = dest_dir.join("decode.rs");
    eprintln!("Generating file: {:?}", &decode_fname);
    let mut out_file = File::create(&decode_fname)?;
    write_decode_file(&profile, &mut out_file)?;
    rustfmt(&decode_fname);

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    });
}
