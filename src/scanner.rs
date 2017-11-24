use super::lexer_basic::Lexer;

pub trait Token {
    fn condition(c: u8) -> bool;
    fn scan<T: Token>(lexer: &Lexer, c: u8) -> T;
}
