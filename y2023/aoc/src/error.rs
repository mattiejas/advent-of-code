use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, AocError>;

#[derive(Debug)]
pub enum AocError {
    ParseError(String),
    RegexError(regex::Error),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::ParseError(ref message) => write!(f, "Parse error: {}", message),
            AocError::RegexError(ref err) => write!(f, "Regex error: {}", err),
        }
    }
}

impl Error for AocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AocError::ParseError(ref _message) => None,
            AocError::RegexError(ref err) => Some(err),
        }
    }
}