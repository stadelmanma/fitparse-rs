//! Parse the FIT SDK profile excel workbook to generate the decoding module.
use fitparser;
use fitparser::FitDataRecord;
use fitparser::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use structopt::StructOpt;

/// Parse a FIT file and a refence file that as generated using the FIT SDK to
/// verify our decoder matches what the SDK outputs
#[derive(Debug, StructOpt)]
#[structopt(name = "validate-decoder")]
struct Cli {
    /// Path to reference file generated from FIT SDK decoder
    #[structopt(name = "REF_FILE", parse(from_os_str))]
    ref_file: PathBuf,
}

struct Field {
    number: u16,
    name: String,
    value: Value,
    units: Option<String>,
}

struct Message {
    number: u16,
    name: String,
    fields: Vec<Field>,
}

struct RefFile {
    fit_file: PathBuf,
    messages: Vec<Message>,
}

fn parse_ref_file(ref_file: &PathBuf) -> Result<RefFile, Box<dyn std::error::Error>> {
    let fp = File::open(ref_file)?;
    let fp = BufReader::new(fp);
    let mut lines_iter = fp.lines();

    // grab file line first, surely there's a better way to do this?
    let fit_file = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split(":")
        .nth(1)
        .map(|v| PathBuf::from(v))
        .unwrap();

    // use next line to initialize the first message
    let mut messages = Vec::new();
    let mut msg = parse_message(lines_iter.next().unwrap().unwrap())?;

    // parse remaining lines, a new message starts the first characer
    // of a line, while message fields are indented with a tab
    for line in lines_iter {
        let line = line?;
        if line.starts_with('\t') {
            // parse field
            let data = line.split(',').take(4).collect::<Vec<&str>>();
            let units = data.get(3).map(|v| String::from(*v));
            let (number, name, value) = (data[0], data[1], data[2]);
            let field = Field {
                number: number.parse::<u16>()?,
                name: name.to_owned(),
                value: parse_value(value),
                units,
            };
            msg.fields.push(field);
        } else {
            messages.push(msg);
            msg = parse_message(line)?;
        }
    }
    messages.push(msg);
    Ok(RefFile { fit_file, messages })
}

fn parse_message(line: String) -> Result<Message, ParseIntError> {
    let data = line.split(',').take(2).collect::<Vec<&str>>();
    let (number, name) = (data[0], data[1]);
    Ok(Message {
        number: number.parse::<u16>()?,
        name: name.to_owned(),
        fields: Vec::new(),
    })
}

fn parse_value(val_str: &str) -> Value {
    // The CPP code emits everything that isn't a string
    // as a float value
    match val_str.parse::<f64>() {
        Ok(val) => Value::Float64(val),
        Err(_) => Value::String(val_str.to_owned()),
    }
}

fn validate_data(ref_data: &[Message], parsed_data: &[FitDataRecord]) -> Result<(), String> {
    if ref_data.len() != parsed_data.len() {
        return Err(format!(
            "reference file and parsed file have different number of messsages: {} != {}",
            ref_data.len(),
            parsed_data.len()
        ));
    }

    for (ref_msg, data_rec) in ref_data.iter().zip(parsed_data) {
        panic!("not implemented");
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Cli::from_args();

    let ref_data = parse_ref_file(&opt.ref_file)?;

    let mut fp = File::open(&ref_data.fit_file)?;
    let data = fitparser::from_reader(&mut fp)?;

    validate_data(&ref_data.messages, &data)?;
    Ok(())
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
