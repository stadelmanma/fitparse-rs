use nom;
use std::error::Error as StdError;
use std::io;
use std::{error, fmt};

/// The result of a deserialization operation.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error that can be produced during deserializing.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during deserialization.
/// TODO: Handle errors produced by nom cleanly
#[derive(Debug)]
pub enum ErrorKind {
    /// Errors tied to IO issues and not the actual parsing steps.
    Io(io::Error),
    /// If a definition mesage can't be found, postion of message and local message number
    MissingDefinitionMessage(u8, usize),
    /// Trailing bytes remain after parsing
    TrailingBytes(usize),
    /// Errors generated by trying to parse invalid data with a nom combinator
    ParseError(usize, nom::error::ErrorKind),
    /// Errors tied to insufficent data in the buffer, similar to an IO error but coming from nom
    UnexpectedEof(nom::Needed),
    /// Errors related to interactions with a Value enum
    ValueError(String),
}

impl StdError for ErrorKind {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::MissingDefinitionMessage(..) => None,
            ErrorKind::TrailingBytes(_) => None,
            ErrorKind::ParseError(..) => None, // TODO, I should chain nom's error in here somehow
            ErrorKind::UnexpectedEof(..) => None,
            ErrorKind::ValueError(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io(ref ioerr) => write!(fmt, "io error: {}", ioerr),
            ErrorKind::TrailingBytes(rem) => {
                write!(fmt, "{} bytes remain past expected EOF location", rem)
            }
            ErrorKind::MissingDefinitionMessage(local_number, position) => write!(
                fmt,
                "No definition found for local message number {} at position {:x}",
                local_number, position
            ),
            ErrorKind::ParseError(pos, ref err) => write!(
                fmt,
                "parser error: '{}' at position: {:#x}",
                err.description(),
                pos
            ),
            ErrorKind::UnexpectedEof(nom::Needed::Size(n)) => {
                write!(fmt, "parser error: requires {} more bytes", n)
            }
            ErrorKind::UnexpectedEof(nom::Needed::Unknown) => {
                write!(fmt, "parser error: requires more data")
            }
            ErrorKind::ValueError(ref message) => write!(fmt, "value error: {}", message),
        }
    }
}
