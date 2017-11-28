use super::Tokn;
use super::error::Error;
use super::{Lexer, State};
use ::util::*;

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Result<Tokn, Error> {
        self.reset_token();

        loop {
            match self.token {
                Err(Error::First) => self.scan(),
                _ => { break; },
            }
        }

        (&self.token).clone()
    }

    fn scan(&mut self) {
        let c = self.current_char;
        let s = self.state;
        match s {
            State::Normal => self.scan_normal(c),

            State::InnerTag | State::InnerName => {
                self.scan_bytes_like_string(c, s)
            },
        }
    }

    // State::Normalの時に通る
    fn scan_normal(&mut self, c: u8) {
        match c {
            // 普通にEOF
            _ if self.eof => self.finish_error(Error::EOF),

            // ゼロを判別する
            b'0' => self.scan_number_zero(c),

            // dispatcher
            _ if self.is_dispatcher_sign(c) => self.scan_dispatcher(c),

            // otag
            _ if self.is_first_otag_letter(c) => self.advance(c, State::InnerTag),

            // normal name
            _ if is_alphabetic_lowercase(c) || is_digit(c) => {
                self.advance(c, State::InnerName)
            },

            // whitespace
            _ if is_whitespace(c) => self.consume_char(),

            // other byte is error
            _ => self.finish_error(Error::Invalid("Invalid Symbol".to_string())),
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

    fn scan_bytes_like_string(&mut self, c: u8, state: State) {
        match c {
            // 途中で終わってもそこまでのOtagとみなす
            _ if self.eof => self.finish_with_state(state),

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

    fn finish_with_state(&mut self, state: State) {
        match state {
            State::InnerTag => self.finish_otag(),
            State::InnerName => self.finish_charactor_vector(),
            _ => self.finish_error(Error::Invalid("Invalid State".to_string())),
        }
    }

    fn error_with_state(&mut self, s: String, state: State) -> Error {
        match state {
            State::InnerTag => Error::Invalid(format!("Invalid tag {}", s)),
            State::InnerName => Error::Invalid(format!("Invalid name {}", s)),
            _ => Error::Invalid("Invalid State".to_string()),
        }
    }

    fn finish_otag(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Otag(s)), State::Normal);
    }

    fn finish_charactor_vector(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Chvc(s)), State::Normal);
    }
}
