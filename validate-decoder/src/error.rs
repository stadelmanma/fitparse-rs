//! Error handling for validation crate
use std::error::Error as StdError;
use std::io;
use std::{error, fmt};

/// The result of a deserialization operation.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error that can be produced during ref file parsing or validation.
pub type Error = Box<ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    /// Errors from the fit parser crate
    FitParserError(fitparser::Error),
    /// Errors tied to IO issues and not the actual parsing steps.
    Io(io::Error),
    /// RefFile Parsing error, (line number, error message)
    ParseError((usize, String)),
    /// RefFile validation error
    ValidationError(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ErrorKind::FitParserError(ref err) => write!(fmt, "fit parsing error: {}", err),
            ErrorKind::Io(ref err) => write!(fmt, "io error: {}", err),
            ErrorKind::ParseError((line_no, msg)) => {
                write!(fmt, "Parsing error: '{}' on line {}", msg, line_no + 1)
            }
            ErrorKind::ValidationError(msg) => write!(fmt, "Validation Error: {}", msg),
        }
    }
}

impl StdError for ErrorKind {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ErrorKind::FitParserError(ref err) => Some(err),
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::ParseError(..) => None,
            ErrorKind::ValidationError(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl From<fitparser::Error> for Error {
    fn from(err: fitparser::Error) -> Error {
        ErrorKind::FitParserError(err).into()
    }
}

impl From<(usize, std::num::ParseIntError)> for Error {
    fn from(err: (usize, std::num::ParseIntError)) -> Error {
        ErrorKind::ParseError((err.0, err.1.to_string())).into()
    }
}
