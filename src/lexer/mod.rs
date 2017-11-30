mod basic;
mod error;
mod main;
mod number;

mod new;

use super::token::Tokn;
use self::error::Error;
use self::new::Scanner;

pub struct Lexer<'a, 'b> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: State,
    token_bytes: Vec<u8>,
    token: Result<Tokn, Error>,
    eof: bool,
    scanners: Vec<&'b Scanner>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum State {
    Normal,
    InnerTag,
    InnerName,

    ZeroNumber,
    // InnerNumber,
    // InnerText,
    // FinishText,
    // AfterUnderscore,
    // AfterDot,
    // InnerComment,
}
