// use std::cell::{Cell, RefCell};

use super::{Lexer, LexerState};
use super::Tokn;
use super::error::Error;
use ::util::*;

/*
 Lexer用の各種基本関数
*/
impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter,
            current_char: c, state: LexerState::Normal,
            token_bytes: vec![], token: Err(Error::First), eof: false, }
    }

    pub fn finish_error(&mut self, e: Error) {
        self.finish(Err(e), LexerState::Normal);
    }

    pub fn advance(&mut self, c: u8, next: LexerState) {
        self.token_bytes.push(c);
        self.consume_char();
        self.state = next;
    }

    pub fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerState::Normal);
    }

    pub fn reset_token(&mut self) {
        self.token_bytes.clear();
        self.token = Err(
            Error::First);
    }

    pub fn finish(&mut
        self, tokn: Result<Tokn, Error>, next: LexerState) {
        self.token = tokn;
        self.state = next;
    }

    pub fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            self.eof = true;
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
