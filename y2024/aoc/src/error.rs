use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, AocError>;

#[derive(Debug)]
pub enum AocError {
    ParseError(String),
    RegexError(regex::Error),
    ComputeError(String),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::ParseError(ref message) => write!(f, "Parse error: {}", message),
            AocError::RegexError(ref err) => write!(f, "Regex error: {}", err),
            AocError::ComputeError(ref err) => write!(f, "Compute error: {}", err),
        }
    }
}

impl Error for AocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AocError::ParseError(ref _message) => None,
            AocError::RegexError(ref err) => Some(err),
            AocError::ComputeError(ref _err) => None,
        }
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> Self {
        AocError::ParseError(err.to_string())
    }
}

impl From<regex::Error> for AocError {
    fn from(err: regex::Error) -> Self {
        AocError::RegexError(err)
    }
}
