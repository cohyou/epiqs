macro_rules! push_into_mode {
    ($e:ident) => {{
        let opt = vec![
           ScanOption::PushCharToToken,
           ScanOption::ChangeState(State::$e),
        ];
        ScanResult::Continue(opt)
    }}
}
macro_rules! push {
    () => {{
        ScanResult::Continue(vec![ScanOption::PushCharToToken])
    }}
}

macro_rules! finish {
    () => {{
        let opts = vec![
            ScanOption::ClearBytes,
            ScanOption::ChangeState(State::Normal),
            ScanOption::ConsumeChar,
        ];
        ScanResult::Finish(opts)
    }}
}

macro_rules! go_ahead {
    () => { ScanResult::Continue(vec![]) }
}

macro_rules! delimite {
    () => {{
        let opts = vec![
            ScanOption::ClearBytes,
            ScanOption::ChangeState(State::Normal)
        ];
        ScanResult::Finish(opts)
    }}
}

macro_rules! next_char {
    () => { ScanResult::Continue(vec![ScanOption::ConsumeChar]) }
}

macro_rules! print_lexer_info {
    ($slf:ident, $e:ident) => {{
        let s = $slf.state.get();
        let c = $slf.current_char;
        let debug_t = $slf.get_token_string();
        let debub_c = $slf.get_char_string(c);
        println!("state: {:?} bytes: {:?} char: {:?} scanner: {:?}", s, debug_t, debub_c, $e);
    }}
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
pub use self::delimiter::DelimiterScanner;

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
    ClearBytes,
    PushCharToToken,
    ChangeState(State),
    ConsumeChar,
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

const FIRST_CHAR: u8 = 255;

impl<'a, 'b> Lexer<'a, 'b> {
    // newメソッドとほぼ同じだが、
    // current_charの初期値を0にしている部分だけが異なる
    pub fn new<I>(iter: &'a mut I, scanners: &'b Vec<&'b Scanner>) -> Lexer<'a, 'b>
    where I: Iterator<Item=u8> {
        Lexer { iter: iter, current_char: FIRST_CHAR, state: Cell::new(State::Normal),
            token_bytes: RefCell::new(vec![]), scanners: scanners, }
    }

    fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            self.current_char = 0; // EOF
        }
    }

    fn exec_option(&mut self, _s: State, c: u8, opts: &Vec<ScanOption>) {
        for o in opts.iter() {
            match *o {
                ScanOption::ClearBytes => { self.token_bytes.borrow_mut().clear(); }
                ScanOption::PushCharToToken => { self.token_bytes.borrow_mut().push(c); },
                ScanOption::ChangeState(s) => { self.state.set(s); },
                ScanOption::ConsumeChar => { self.consume_char(); },
            }
        }
    }

    fn get_tokenize_error(&self, s: State, t: String, c: u8) -> TokenizeResult {
        let error_info = format!("state: {:?} token_bytes: {:?} char: {:?}", s, t.clone(), c);
        println!("Error: {:?}", error_info);
        return TokenizeResult::Err(Error::Invalid(error_info));
    }

    fn get_char_string(&self, c: u8) -> String {
        match c {
            FIRST_CHAR => "<SOF>".to_string(),
            0 => "<EOF>".to_string(),
            _ => String::from_utf8(vec![c]).expect("Found invalid UTF-8"),
        }

    }

    fn get_token_string(&self) -> String {
        String::from_utf8(self.token_bytes.borrow().clone()).expect("Found invalid UTF-8")
    }

    pub fn tokenize(&mut self) -> TokenizeResult {
        loop {
            let s = self.state.get();
            let c = self.current_char;

            for scanner in self.scanners.iter() {
                print_lexer_info!(self, scanner);

                match scanner.scan(s, c) {
                    ScanResult::Continue(ref opts) => {
                        println!("Continue");
                        self.exec_option(s, c, opts);
                    },

                    ScanResult::Finish(ref opts) => {
                        let t = self.get_token_string();
                        if let Some(r) = scanner.return_token(s, t) {
                            self.exec_option(s, c, opts);
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

#[test]
#[ignore]
fn test() {
    let scanners: &mut Vec<&Scanner> = &mut vec![
        &DelimiterScanner,
        &AlphanumericScanner,
        &ZeroScanner,
        &IntegerScanner,
    ];
    // lex_from_str("|Ab", vec!["Pipe", "Otag<Ab>"], scanners);
    lex_from_str("|: abc 123", "Pipe Otag<:> Chvc<abc> Nmbr<123>", scanners);
}

fn lex_from_str(text: &str, right: &str, scanners: &mut Vec<&Scanner>) {
    let mut iter = text.bytes();
    scanners.push(&EOFScanner);
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
    let r = result.join(" ");
    assert_eq!(r, right);
}
