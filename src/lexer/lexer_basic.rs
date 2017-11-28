use std::cell::{Cell, RefCell};

use super::{Lexer, LexerState};
use super::token::Tokn;
use super::lexer_error::LexerError;
use ::util::*;

/*
 Lexer用の各種基本関数
*/
impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter,
            current_char: Cell::new(c), state: Cell::new(LexerState::Normal),
            token_bytes: vec![], token: RefCell::new(Err(LexerError::First)), eof: Cell::new(false), }
    }

    pub fn finish_error(&self, e: LexerError) {
        self.finish(Err(e), LexerState::Normal);
    }

    pub fn advance(&mut self, c: u8, next: LexerState) {
        self.token_bytes.push(c);
        self.consume_char();
        self.state.set(next);
    }

    pub fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerState::Normal);
    }

    pub fn reset_token(&mut self) {
        self.token_bytes.clear();
        let mut t = self.token.borrow_mut();
        *t = Err(LexerError::First);
    }

    pub fn finish(&self, tokn: Result<Tokn, LexerError>, next: LexerState) {
        let mut t = self.token.borrow_mut();
        *t = tokn;
        self.state.set(next);
    }

    pub fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char.set(c);
        } else {
            self.eof.set(true);
        }
    }

    pub fn get_token_string(&self) -> String {
        String::from_utf8(self.token_bytes.clone()).expect("Found invalid UTF-8")
    }

    pub fn is_first_otag_letter(&self, c: u8) -> bool {
        is_alphabetic_uppercase(c) || self.is_otag_sign(c)
    }

    pub fn is_dispatcher_sign(&self, c: u8) -> bool {
        c == b'|'
    }

    pub fn is_otag_sign(&self, c: u8) -> bool {
        c == b':'
        // 区切り文字ならここでNameを終わらせる必要がある
        // ただし、全ての区切り文字がここで判断されるわけではない
        // b'[' | b']' | b'(' | b')' | b'{' | b'}' | b':' | b',' => self.finish_with_state(state),
    }
}
