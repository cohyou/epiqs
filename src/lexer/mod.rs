macro_rules! push_into_mode {
    ($e:ident) => {
        {
            let opt = vec![
               ScanOption::PushCharToToken,
               ScanOption::ChangeState(State::$e),
            ];
            ScanResult::Continue(opt)
        }
    }
}
macro_rules! push {
    () => {{
        ScanResult::Continue(vec![ScanOption::PushCharToToken])
    }}
}

macro_rules! finish {
    () => {{
        let opts = vec![ScanOption::ChangeState(State::Normal)];
        ScanResult::Finish(opts)
    }}
}

macro_rules! go_ahead {
    () => { ScanResult::Continue(vec![]) }
}

mod error;

mod eof;
mod number;
mod delimiter;
mod alphanumeric;

use std::fmt::Debug;
use std::cell::{Cell, RefCell};

use core::*;
use self::error::Error;

pub use self::eof::EOFScanner;
pub use self::alphanumeric::AlphanumericScanner;
pub use self::number::ZeroScanner;
pub use self::number::IntegerScanner;

pub struct Lexer<'a, 'b> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: Cell<State>,
    token_bytes: RefCell<Vec<u8>>,
    scanners: &'b Vec<&'b Scanner>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum State {
    Normal,
    InnerOtag,
    InnerName,

    ZeroNumber,
    InnerNumber,
    Delimiter,

    // InnerText,
    // FinishText,
    // AfterUnderscore,
    // AfterDot,
    // InnerComment,
}

// やり方が分からないのでとりあえず
#[derive(Debug)]
pub enum ScanOption {
    PushCharToToken,
    ChangeState(State),
}

#[derive(Debug)]
pub enum ScanResult {
    Continue(Vec<ScanOption>),
    Finish(Vec<ScanOption>),
    Error,
    EOF,
}

pub trait Scanner : Debug {
    fn scan(&self, state: State, c: u8) -> ScanResult;
    fn return_token(&self, _state: State, _token_string: String) -> Option<Tokn> {
        None
    }
}

pub enum TokenizeResult {
    Ok(Tokn),
    Err(Error),
    EOF(Tokn),
    EmptyEOF,
}

impl<'a, 'b> Lexer<'a, 'b> {
    // newメソッドとほぼ同じだが、
    // current_charの初期値を0にしている部分だけが異なる
    pub fn new<I>(iter: &'a mut I, scanners: &'b Vec<&'b Scanner>) -> Lexer<'a, 'b>
    where I: Iterator<Item=u8> {
        Lexer { iter: iter, current_char: 0, state: Cell::new(State::Normal),
            token_bytes: RefCell::new(vec![]), scanners: scanners, }
    }

    fn consume_char_new(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            self.current_char = 0; // EOF
        }
    }

    fn exec_option(&self, _s: State, c: u8, opts: &Vec<ScanOption>) {
        for o in opts.iter() {
            match *o {
                ScanOption::PushCharToToken => { self.token_bytes.borrow_mut().push(c); },
                ScanOption::ChangeState(s) => { self.state.set(s); },
            }
        }
    }

    fn push_char_if_needed(&self, _s: State, c: u8, opts: &Vec<ScanOption>) {
        for o in opts.iter() {
            match *o {
                ScanOption::PushCharToToken => { self.token_bytes.borrow_mut().push(c); },
                ScanOption::ChangeState(_) => { /* do nothing */ },
            }
        }
    }

    fn change_state_if_needed(&self, _s: State, _c: u8, opts: &Vec<ScanOption>) {
        for o in opts.iter() {
            match *o {
                ScanOption::PushCharToToken => { /* do nothing */ },
                ScanOption::ChangeState(s) => { self.state.set(s); },
            }
        }
    }

    fn get_tokenize_error(&self, s: State, t: String, c: u8) -> TokenizeResult {
        let error_info = format!("state: {:?} token_bytes: {:?} char: {:?}", s, t.clone(), c);
        println!("Error: {:?}", error_info);
        return TokenizeResult::Err(Error::Invalid(error_info));
    }

    pub fn get_token_string(&self) -> String {
        String::from_utf8(self.token_bytes.borrow().clone()).expect("Found invalid UTF-8")
    }

    pub fn tokenize(&mut self) -> TokenizeResult {
        self.token_bytes.borrow_mut().clear();

        loop {
            let s = self.state.get();

            self.consume_char_new();
            let c = self.current_char;

            for scanner in self.scanners.iter() {
                println!("state: {:?} char: {:?} scanner: {:?}", s, c, scanner);
                match scanner.scan(s, c) {
                    ScanResult::Continue(ref opts) => {
                        println!("Continue");
                        self.exec_option(s, c, opts);
                    },

                    ScanResult::Finish(ref opts) => {
                        self.push_char_if_needed(s, c, opts);
                        let t = self.get_token_string();
                        if let Some(r) = scanner.return_token(s, t) {
                            self.change_state_if_needed(s, c, opts);
                            println!("Ok: {:?}", r);
                            return TokenizeResult::Ok(r);
                        } else {
                            let t2 = self.get_token_string();
                            return self.get_tokenize_error(s, t2, c);
                        }
                    },

                    ScanResult::Error => {
                        let t = self.get_token_string();
                        return self.get_tokenize_error(s, t, c);
                    },

                    ScanResult::EOF => {
                        let t = self.get_token_string();
                        if t.len() == 0 {
                            return TokenizeResult::EmptyEOF;
                        }
                        if let Some(r) = scanner.return_token(s, t) {
                            println!("EOF: {:?}", r);
                            return TokenizeResult::EOF(r);
                        } else {
                            let t2 = self.get_token_string();
                            return self.get_tokenize_error(s, t2, c);
                        }
                    },
                }
            }
        }
    }
}

fn lex_from_str(text: &str, right: Vec<&str>, scanners: &Vec<&Scanner>) {
    let mut iter = text.bytes();
    let mut lexer = Lexer::new(&mut iter, scanners);
    let mut result = vec![];
    loop {
        match lexer.tokenize() {
            TokenizeResult::Ok(t) => {
                let s = format!("{:?}", t);
                result.push(s);
            },
            TokenizeResult::Err(e) => {
                let s = format!("{}", e);
                result.push(s);
                break;
            },
            TokenizeResult::EOF(t) => {
                let s = format!("{:?}", t);
                result.push(s);
                break;
            }
            TokenizeResult::EmptyEOF => { break; },
        }
    }
    assert_eq!(result, right);
}
