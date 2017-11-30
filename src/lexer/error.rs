use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    // Unknown,
    First,
    Invalid(String),
    EOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::First => { write!(f, "First") },
            &Error::Invalid(ref s) => { write!(f, "{}", s) },
            &Error::EOF => { write!(f, "EOF") },
        }

    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "not found"
    }
}
