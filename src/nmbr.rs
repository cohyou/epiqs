use super::scanner::Token;
use super::lexer_basic::Lexer;

pub struct Nmbr(String);

impl Token for Nmbr {
    fn condition(c: u8) -> bool {
        c == b'0'
    }

    fn scan<T: Token>(_lexer: &Lexer, _c: u8) -> Nmbr {
        Nmbr("0".to_string())
    }
}
