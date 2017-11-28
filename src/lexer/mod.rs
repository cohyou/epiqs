pub mod token;
mod lexer_basic;
mod lexer_error;
mod lexer_main;
mod lexer_extension;

use std::cell::{Cell, RefCell};
use self::token::Tokn;
use self::lexer_error::LexerError;

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: Cell<u8>,
    state: Cell<LexerState>,
    token_bytes: Vec<u8>,
    token: RefCell<Result<Tokn, LexerError>>,
    eof: Cell<bool>,
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
