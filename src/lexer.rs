// use std::cell::Cell;

use super::token::Tokn;
use super::lexer_error::LexerError;
use super::lexer_state::LexerState;
use super::lexer_basic::Lexer;
use super::util::*;

// use super::scanner::*;
// use super::nmbr::Nmbr;

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Result<Tokn, LexerError> {
        self.reset_token();

        loop {
            match *self.token.borrow() {
                Err(LexerError::First) => self.scan(),
                _ => { break; },
            }
        }

        (&self.token).borrow().clone()
    }

    fn scan(&mut self) {
        let c = self.current_char.get();
        let s = self.state.get();
        match s {
            LexerState::Normal => self.scan_normal(c),

            LexerState::InnerTag | LexerState::InnerName => {
                self.scan_bytes_like_string(c, s)
            },
        }
    }

    // LexerState::Normalの時に通る
    fn scan_normal(&mut self, c: u8) {
        match c {
            // 普通にEOF
            _ if self.eof.get() => self.finish_error(LexerError::EOF),

            // scan_with_scanner
            // _ if self.check_scanner_condition::<Nmbr>(c) => { Nmbr::scan::<Nmbr>(&self, c); },
            _ if Lexer::wowow() => {},

            // dispatcher
            _ if self.is_dispatcher_sign(c) => self.scan_dispatcher(c),

            // otag
            _ if self.is_first_otag_letter(c) => self.advance(c, LexerState::InnerTag),

            // normal name
            _ if is_alphabetic_lowercase(c) || is_digit(c) => {
                self.advance(c, LexerState::InnerName)
            },

            // whitespace
            _ if is_whitespace(c) => self.consume_char(),

            // other byte is error
            _ => self.finish_error(LexerError::Invalid("Invalid Symbol".to_string())),
        }
    }
    /*
    fn check_scanner_condition<T: Token>(&self, c: u8) -> bool {
        T::condition(c)
    }
    */
    fn scan_dispatcher(&mut self, c: u8) {
        match c {
            b'|' => self.delimit(c, Tokn::Pipe),
            // b'^' => self.delimit(c, Tokn::Crrt),
            _ => { /* do nothing, go to next */ }
        }
    }

    fn scan_bytes_like_string(&mut self, c: u8, state: LexerState) {
        match c {
            // 途中で終わってもそこまでのOtagとみなす
            _ if self.eof.get() => self.finish_with_state(state),

            // 空白が来たら区切る
            _ if is_whitespace(c) => self.finish_with_state(state),

            // 英数字なら、引き続き次の文字
            _ if is_alphanumeric(c) => self.advance(c, state),

            // それ以外はエラー
            _ => {
                self.token_bytes.push(c);
                let s; { s = self.get_token_string(); }
                let error = self.error_with_state(s, state);
                self.finish_error(error);
            },
        }
    }

    fn finish_with_state(&self, state: LexerState) {
        match state {
            LexerState::InnerTag => self.finish_otag(),
            LexerState::InnerName => self.finish_charactor_vector(),
            _ => self.finish_error(LexerError::Invalid("Invalid State".to_string())),
        }
    }

    fn error_with_state(&self, s: String, state: LexerState) -> LexerError {
        match state {
            LexerState::InnerTag => LexerError::InvalidTag(s),
            LexerState::InnerName => LexerError::InvalidName(s),
            _ => LexerError::Invalid("Invalid State".to_string()),
        }
    }

    fn finish_otag(&self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Otag(s)), LexerState::Normal);
    }

    fn finish_charactor_vector(&self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Chvc(s)), LexerState::Normal);
    }
}
