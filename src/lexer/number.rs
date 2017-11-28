use ::token::Tokn;
use ::util::*;
use lexer::{Lexer, Error, State};

impl<'a> Lexer<'a> {
    pub fn scan_number_zero(&mut self, c: u8) {
        self.token_bytes.push(c);
        self.consume_char();
        // println!("self.current_char: {}", self.current_char);
        if is_whitespace(self.current_char) || self.eof {
            self.finish_number();
        } else {
            self.finish_error(Error::Invalid("Invalid number".to_string()));
        }
    }

    fn finish_number(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Nmbr(s)), State::Normal);
    }
}
