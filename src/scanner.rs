use super::lexer_basic::Lexer;

pub trait Token {
    fn condition(c: u8) -> bool;
    fn scan<T: Token>(lexer: &Lexer, c: u8) -> T;
}

impl<'a> Lexer<'a> {
    pub fn wowow() -> bool {
        println!("{:?}", "a");
        false
    }
}
