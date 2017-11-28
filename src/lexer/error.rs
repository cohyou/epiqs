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
        write!(f, "No matching cities with a population were found.")
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "not found"
    }
}
