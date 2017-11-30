use ::token::Tokn;
use lexer::{Lexer, State, Error};
use util::is_whitespace;

// やり方が分からないのでとりあえず
#[derive(Debug)]
pub enum ContinueOption {
    PushCharToToken,
    ChangeState(State),
}

#[derive(Debug)]
pub enum ScanResult {
    Continue(Vec<ContinueOption>),
    Finish,
    Error,
    EOF,
}

pub trait Scanner {
    fn scan(&self, state: State, c: u8) -> ScanResult;
    fn return_token(&self, token_string: String) -> Tokn;

    fn s(&self) -> String;
}

pub enum TokenizeResult {
    Ok(Tokn),
    Err(Error),
    EOF,
}

impl<'a, 'b> Lexer<'a, 'b> {
    // newメソッドとほぼ同じだが、
    // current_charの初期値を0にしている部分だけが異なる
    pub fn new2<I>(iter: &'a mut I, scanners: Vec<&'b Scanner>) -> Lexer<'a, 'b>
    where I: Iterator<Item=u8> {
        Lexer { iter: iter,
            current_char: 0, state: State::Normal,
            token_bytes: vec![], token: Err(Error::First), eof: false, scanners: scanners, }
    }

    fn consume_char_new(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            self.current_char = 0; // EOF
        }
    }

    pub fn tokenize(&mut self) -> TokenizeResult {
        self.token_bytes.clear();

        loop {
            let s = self.state;

            self.consume_char_new();
            let c = self.current_char;

            for scanner in self.scanners.iter() {
                println!("state: {:?} char: {:?} scanner: {:?}", s, c, scanner.s());
                match scanner.scan(s, c) {
                    ScanResult::Continue(ref opt) => {
                        println!("Continue");
                        for o in opt {
                            match *o {
                                ContinueOption::PushCharToToken => { self.token_bytes.push(c); },
                                ContinueOption::ChangeState(s) => { self.state = s; },
                            }
                        }
                    },
                    ScanResult::Finish => {
                        let t = self.get_token_string();
                        let r = scanner.return_token(t);
                        println!("Ok: {:?}", r);
                        return TokenizeResult::Ok(r);
                    },
                    ScanResult::Error => {
                        let t = self.get_token_string();
                        let error_info = format!("state: {:?} token_bytes: {:?} char: {:?}", s, t, c);
                        println!("Error: {:?}", error_info);
                        return TokenizeResult::Err(Error::Invalid(error_info));
                    },
                    ScanResult::EOF => {
                        println!("EOF");
                        return TokenizeResult::EOF;
                    },
                }
            }
        }
    }
}
