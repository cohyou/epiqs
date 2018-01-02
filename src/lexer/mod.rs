const LEXER_DEBUGGING: bool = false;

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

macro_rules! finish_into_mode {
    ($e:ident) => {{
        let opts = vec![
            ScanOption::ClearBytes,
            ScanOption::ChangeState(State::$e),
            ScanOption::ConsumeChar,
        ];
        ScanResult::Finish(opts)
    }}
}

macro_rules! finish {
    () => { finish_into_mode!(Normal) }
}

macro_rules! go_ahead {
    () => { ScanResult::Continue(vec![]) }
}

macro_rules! delimite_into_mode {
    ($e:ident) => {{
        let opts = vec![
            ScanOption::ClearBytes,
            ScanOption::ChangeState(State::$e)
        ];
        ScanResult::Finish(opts)
    }}
}

macro_rules! delimite {
    () => { delimite_into_mode!(Normal) }
}

macro_rules! next_char {
    () => { ScanResult::Continue(vec![ScanOption::ConsumeChar]) }
}

macro_rules! print_lexer_info {
    ($slf:ident, $e:ident) => {
        if LEXER_DEBUGGING {
            let s = $slf.state.get();
            let c = $slf.current_char;
            let debug_t = $slf.get_token_string();
            let debub_c = $slf.get_char_string(c);
            println!("state: {:?} bytes: {:?} char: {:?} scanner: {:?}", s, debug_t, debub_c, $e);
        }
    }
}

macro_rules! print_continue {
    () => {
        if LEXER_DEBUGGING { println!("Continue"); }
    }
}

macro_rules! print_finished_token {
    ($r:ident) => {
        if LEXER_DEBUGGING { println!("Ok: {:?}", $r); }
    }
}

mod error;

mod eof;
mod number;
mod delimiter;
mod alphabet;
mod text;

use std::fmt::Debug;
use std::cell::{Cell, RefCell};

use core::*;
use self::error::Error;

pub use self::eof::EOFScanner;
pub use self::alphabet::AlphabetScanner;
pub use self::number::ZeroScanner;
pub use self::number::IntegerScanner;
pub use self::delimiter::DelimiterScanner;
pub use self::text::TextScanner;

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

    InnerText,
    FinishText,
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
    EOF,
}

const FIRST_CHAR: u8 = 255;

impl<'a, 'b> Lexer<'a, 'b> {
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
                        print_continue!();
                        self.exec_option(s, c, opts);
                    },

                    ScanResult::Finish(ref opts) => {
                        let t = self.get_token_string();
                        if let Some(r) = scanner.return_token(s, t) {
                            self.exec_option(s, c, opts);
                            print_finished_token!(r);
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

                    ScanResult::EOF => return TokenizeResult::EOF,
                }
            }
        }
    }
}

#[test]
#[ignore]
fn one_piq() {
    let scanners: &mut Vec<&Scanner> = &mut vec![
        &DelimiterScanner,
        &AlphabetScanner,
        &ZeroScanner,
        &IntegerScanner,
    ];

    lex_from_str("|: abc 123", "Pipe Otag<:> Chvc<abc> Nmbr<123>", scanners);
}

#[test]
#[ignore]
fn bind_piq() {
    lex_from_str_with_all_scanners("|# abc 123", "Pipe Otag<#> Chvc<abc> Nmbr<123>");
}

#[test]
fn bracket_list() {
    lex_from_str_with_all_scanners("[dog 256 cat 512]", "Lbkt Chvc<dog> Nmbr<256> Chvc<cat> Nmbr<512> Rbkt");
}

#[test]
fn mpiq() {
    // lex_from_str_with_all_scanners("^> 246", "Crrt Otag<>> Nmbr<246>");
    lex_from_str_with_all_scanners("|% ; -1", "Pipe Otag<%> Smcl Nmbr<-1>");
}

#[test]
fn tval() {
    lex_from_str_with_all_scanners("^T", "Crrt Otag<T>");
}

#[test]
fn primitive_function() {
    lex_from_str_with_all_scanners("decr", "Chvc<decr>");
}

#[test]
// #[ignore]
fn texts() {
    lex_from_str_with_all_scanners("[0 \"b\"]", "Lbkt Nmbr<0> Dbqt Chvc<b> Dbqt Rbkt")
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

            TokenizeResult::EOF => {
                break;
            }
        }
    }
    let r = result.join(" ");
    assert_eq!(r, right);
}

fn lex_from_str_with_all_scanners(text: &str, right: &str) {
    let scanners: &mut Vec<&Scanner> = &mut vec![
        &DelimiterScanner,
        &TextScanner,
        &AlphabetScanner,
        &ZeroScanner,
        &IntegerScanner,
    ];

    lex_from_str(text, right, scanners);
}
