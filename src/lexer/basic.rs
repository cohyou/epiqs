// use std::cell::{Cell, RefCell};

/*
use super::{Lexer, State};
use super::Tokn;
use super::error::Error;
use ::util::*;
*/

/*
 Lexer用の各種基本関数
*/
// impl<'a, 'b> Lexer<'a, 'b> {
    /*
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        let v = vec![];
        Lexer { iter: iter,
            current_char: c, state: Cell::new(State::Normal),
            token_bytes: RefCell::new(vec![]), token: Err(Error::First), eof: false, scanners: &v, }
    }

    pub fn finish_error(&mut self, e: Error) {
        self.finish(Err(e), State::Normal);
    }

    pub fn advance(&mut self, c: u8, next: State) {
        self.token_bytes.borrow_mut().push(c);
        self.consume_char();
        self.state.set(next);
    }

    pub fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.borrow_mut().push(c);
        self.consume_char();
        self.finish(Ok(t), State::Normal);
    }

    pub fn reset_token(&mut self) {
        self.token_bytes.borrow_mut().clear();
        self.token = Err(Error::First);
    }

    pub fn finish(&mut self, tokn: Result<Tokn, Error>, next: State) {
        self.token = tokn;
        self.state.set(next);
    }

    pub fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            // TODO: 本来であれば、eofかどうかをself.current_char == 0かどうかで判断したいが、
            // なぜかそうするとうまくループが終了しなくなるので、特別なフラグを立てたままにしている
            // この場合、EOF時にcurrent_charが更新されないままになってしまうので、current_charを判断の基準には使えず、
            // 分岐などでもEOFの場合を優先して判定しないといけなくなり、わかりづらく不便なのでできれば修正したい...

            // self.current_char = b'0';
            self.eof = true;
        }
    }*/

/*
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

    pub fn eof(&self) -> bool {
        // TODO: 本来であれば、eofかどうかをself.current_char == 0かどうかで判断したいが、
        // なぜかそうするとうまくループが終了しなくなるので、特別なフラグを立てたままにしている
        // この場合、EOF時にcurrent_charが更新されないままになってしまうので、current_charを判断の基準には使えず、
        // 分岐などでもEOFの場合を優先して判定しないといけなくなり、わかりづらく不便なのでできれば修正したい...

        // self.current_char == b'0'
        self.eof
    }*/
// }
