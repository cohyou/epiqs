mod basic;
mod error;
mod main;
// mod extension;
mod number;

use super::token::Tokn;
use self::error::Error;

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: State,
    token_bytes: Vec<u8>,
    token: Result<Tokn, Error>,
    eof: bool,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum State {
    Normal,
    InnerTag,
    InnerName,

    // ZeroNumber,
    // InnerNumber,
    // InnerText,
    // FinishText,
    // AfterUnderscore,
    // AfterDot,
    // InnerComment,
}
