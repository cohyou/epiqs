use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    // Unknown,
    First,
    Invalid(String),
    // InvalidText(String),
    // InvalidNumber(String),
    InvalidName(String),
    InvalidTag(String),
    EOF,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No matching cities with a population were found.")
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        "not found"
    }
}
