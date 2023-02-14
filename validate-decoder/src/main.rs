//! Parse the FIT SDK profile excel workbook to generate the decoding module.
use fitparser;
use fitparser::FitDataRecord;
use fitparser::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

mod error;
use error::{Error, ErrorKind, Result};

/// Parse a FIT file and a refence file that as generated using the FIT SDK to
/// verify our decoder matches what the SDK outputs
#[derive(Debug, StructOpt)]
#[structopt(name = "validate-decoder")]
struct Cli {
    /// Path to reference file generated from FIT SDK decoder
    #[structopt(name = "REF_FILE", parse(from_os_str))]
    ref_file: PathBuf,
}

#[derive(Debug)]
struct Field {
    number: u8,
    name: String,
    value: Value,
    units: Option<String>,
}

#[derive(Debug)]
struct Message {
    number: u16,
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
struct RefFile {
    fit_file: PathBuf,
    messages: Vec<Message>,
}

fn parse_ref_file(ref_file: &PathBuf) -> Result<RefFile> {
    let fp = File::open(ref_file)?;
    let fp = BufReader::new(fp);
    let mut lines_iter = fp.lines().enumerate();

    // grab file line first, surely there's a better way to do this?
    let fit_file = lines_iter
        .next()
        .unwrap()
        .1
        .unwrap()
        .split(":")
        .nth(1)
        .map(|v| PathBuf::from(v.trim()))
        .unwrap();

    // use next line to initialize the first message
    let mut messages = Vec::new();
    let mut msg = match lines_iter.next() {
        Some((ln, line)) => line
            .map_err(|e| Error::from(e))
            .and_then(|l| parse_message(ln, l)),
        None => {
            return Err(Box::new(ErrorKind::ParseError((
                1,
                "No line present".to_string(),
            ))))
        }
    }?;

    // parse remaining lines, a new message starts the first characer
    // of a line, while message fields are indented with a tab
    for (line_no, line) in lines_iter {
        let line = line?;
        if line.starts_with('\t') {
            // parse field
            let data = line.split(',').take(4).collect::<Vec<&str>>();
            let units = data.get(3).map(|v| String::from(*v));
            let (number, name, value) = (data[0], data[1], data[2]);
            let field = Field {
                number: number
                    .trim()
                    .parse::<u8>()
                    .map_err(|e| Error::from((line_no, e)))?,
                name: name.to_owned(),
                value: parse_value(value),
                units,
            };
            msg.fields.push(field);
        } else {
            msg.fields.sort_by(|f1, f2| f1.number.cmp(&f2.number));
            messages.push(msg);
            msg = parse_message(line_no, line)?;
        }
    }
    messages.push(msg);
    Ok(RefFile { fit_file, messages })
}

fn parse_message(line_no: usize, line: String) -> Result<Message> {
    let data = line.split(',').take(2).collect::<Vec<&str>>();
    let (number, name) = (data[0], data[1]);
    Ok(Message {
        number: number
            .parse::<u16>()
            .map_err(|e| Error::from((line_no, e)))?,
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

fn validate_data(ref_data: &[Message], parsed_data: &[FitDataRecord]) -> Result<()> {
    if ref_data.len() != parsed_data.len() {
        return Err(ErrorKind::ValidationError(format!(
            "reference file and parsed file have different number of messsages: {} != {}",
            ref_data.len(),
            parsed_data.len()
        ))
        .into());
    }

    for (idx, (ref_msg, data_rec)) in ref_data.iter().zip(parsed_data).enumerate() {
        if ref_msg.number != data_rec.kind().as_u16() {
            return Err(ErrorKind::ValidationError(format!(
                "message number difference in message #{}: {} != {}",
                idx + 1,
                ref_msg.number,
                data_rec.kind().as_u16()
            ))
            .into());
        }
        if ref_msg.name != data_rec.kind().to_string() {
            return Err(ErrorKind::ValidationError(format!(
                "message name difference in message #{}: {} != {}",
                idx + 1,
                ref_msg.name,
                data_rec.kind().to_string()
            ))
            .into());
        }
        for (fidx, (ref_fld, data_fld)) in ref_msg.fields.iter().zip(data_rec.fields()).enumerate()
        {
            if ref_fld.number != data_fld.number() {
                return Err(ErrorKind::ValidationError(format!(
                    "field number difference in message #{}, field #{}: {:?} != {:?}",
                    idx + 1,
                    fidx + 1,
                    ref_msg,
                    data_rec
                ))
                .into());
            }
            if ref_fld.name != data_fld.name() {
                return Err(ErrorKind::ValidationError(format!(
                    "field name difference in message #{}, field #{}: {} != {}",
                    idx + 1,
                    fidx + 1,
                    ref_fld.name,
                    data_fld.name()
                ))
                .into());
            }
            // TODO validate value
            // TODO valdate units
        }
    }

    eprintln!("Valdiation successful!");
    Ok(())
}

fn run() -> Result<()> {
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
