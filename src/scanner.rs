use super::lexer_basic::Lexer;

trait Token {
    fn condition(c: u8) -> bool;
    fn scan(lexer: &Lexer, c: u8) -> Self;
}

pub struct Nmbr(String);

impl Token for Nmbr {
    fn condition(c: u8) -> bool {
        c == b'0'
    }

    fn scan(_lexer: &Lexer, _c: u8) -> Self {
        Nmbr("0".to_string())
    }
}
