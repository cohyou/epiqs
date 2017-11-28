mod lexer_basic;
mod lexer_error;
mod lexer_main;
// mod lexer_extension;

// use std::cell::{Cell, RefCell};
use super::token::Tokn;
use self::lexer_error::LexerError;

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: LexerState,
    token_bytes: Vec<u8>,
    token: Result<Tokn, LexerError>,
    eof: bool,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LexerState {
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
