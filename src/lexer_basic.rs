use super::token::Tokn;
// use super::lexer::Lexer;
use super::lexer_state::LexerState;
use super::lexer_error::LexerError;
use super::util::*;

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    pub current_char: u8,
    pub state: LexerState,
    pub token_bytes: Vec<u8>,
    token: Result<Tokn, LexerError>,
    pub eof: bool,
}

/*
 Lexer用の各種基本関数
*/
impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter, current_char: c, state: LexerState::Normal,
            token_bytes: vec![], token: Err(LexerError::First), eof: false, }
    }

    pub fn finish_error(&mut self, e: LexerError) {
        // let s = self.token_bytes.clone();
        self.finish(Err(e), LexerState::Normal);
    }

    pub fn next_token(&mut self) -> Result<Tokn, LexerError> {
        self.reset_token();

        loop {
            match self.token {
                Err(LexerError::First) => self.scan(),
                _ => { break; },
            }
        }

        (&self.token).clone()
    }

    pub fn advance(&mut self, c: u8, next: &LexerState) {
        self.token_bytes.push(c);
        self.consume_char();
        self.state = next.clone();
    }

    pub fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerState::Normal);
    }

    fn reset_token(&mut self) {
        self.token_bytes.clear();
        self.token = Err(LexerError::First);
    }

    pub fn finish(&mut self, tokn: Result<Tokn, LexerError>, next: LexerState) {
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
