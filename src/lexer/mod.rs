mod basic;
mod error;
mod main;

mod number;
mod tpiq;

use std::cell::{Cell, RefCell};

use super::token::Tokn;
use self::error::Error;

pub struct Lexer<'a, 'b> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: Cell<State>,
    token_bytes: RefCell<Vec<u8>>,
    // token: Result<Tokn, Error>,
    // eof: bool,
    scanners: &'b Vec<&'b Scanner>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum State {
    Normal,
    InnerTag,
    InnerName,

    ZeroNumber,
    InnerNumber,
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
    Stop,
    EOF,
}

pub trait Scanner {
    fn scan(&self, state: State, c: u8) -> ScanResult;
    fn return_token(&self, state: State, token_string: String) -> Option<Tokn>;

    fn s(&self) -> String;
}

pub enum TokenizeResult {
    Ok(Tokn),
    Err(Error),
    Stop(Tokn),
    EOF,
}

impl<'a, 'b> Lexer<'a, 'b> {
    // newメソッドとほぼ同じだが、
    // current_charの初期値を0にしている部分だけが異なる
    pub fn new2<I>(iter: &'a mut I, scanners: &'b Vec<&'b Scanner>) -> Lexer<'a, 'b>
    where I: Iterator<Item=u8> {
        Lexer { iter: iter,
            current_char: 0, state: Cell::new(State::Normal),
            token_bytes: RefCell::new(vec![]), /*token: Err(Error::First), eof: false,*/ scanners: scanners, }
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
                ScanOption::ChangeState(s) => { /* do nothing */ },
            }
        }
    }

    fn change_state_if_needed(&self, _s: State, c: u8, opts: &Vec<ScanOption>) {
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
                println!("state: {:?} char: {:?} scanner: {:?}", s, c, scanner.s());
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

                    ScanResult::Stop => {
                        let t = self.get_token_string();
                        if let Some(r) = scanner.return_token(s, t) {
                            println!("EOF: {:?}", r);
                            return TokenizeResult::Stop
                            (r);
                        } else {
                            let t2 = self.get_token_string();
                            return self.get_tokenize_error(s, t2, c);
                        }
                    },
                    ScanResult::EOF => {
                        return TokenizeResult::EOF;
                    },
                }
            }
        }
    }
}

fn lex_from_str(text: &str, right: Vec<&str>, scanners: &Vec<&Scanner>) {
    let mut iter = text.bytes();
    let mut lexer = Lexer::new2(&mut iter, scanners);
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
            TokenizeResult::Stop(t) => {
                let s = format!("{:?}", t);
                result.push(s);
                break;
            },
            TokenizeResult::EOF => {
                break;
            }
        }
    }
    assert_eq!(result, right);
}
