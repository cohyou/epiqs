use std::fmt;
use std::error::Error;
/*
use std::iter::Peekable;
use std::io::Bytes;
use std::io::BufReader;
use std::fs::File;
use std::iter::Map;
use std::io::Read;
use std::fmt::Debug;
use std::cell::RefCell;
*/

#[derive(Clone, PartialEq)]
pub enum Tokn {
    Nmbr(String), // Number
    Text(String), // Text
    Name(String), // Name
    Usnm(String), // under score and number (e.g. _0 _34)

    // asciiのうち、記号は32(スペースを除く、また7Fも対象外)

    Dbqt, // " double quotation

    Lbkt, // [ left bracket
    Rbkt, // ] right bracket
    Lprn, // ( left parentheses
    Rprn, // ) right parentheses
    Lcrl, // { left curly brace
    Rcrl, // } right curly brace

    Coln, // : colon

    Pipe, // | vertical bar
    Crrt, // ^ carret
    Dllr, // $ dollar
    Smcl, // ; semi colon
    Bang, // ! exclamation

    // 残りの記号も列挙
    Plus, // + plus
    Star, // * asterisk
    Bksl, // \ back slash
    Stop, // . full stop (period)

    /*
    Slsh, // / slash
    Hash, // # hash

    Qstn, // ? question mark
    Pcnt, // % percent
    Amps, // & ampersand
    Sgqt, // ' single quotation

    Comm, // , comma
    Hphn, // - hyphen-minus
    Less, // < less than
    Grtr, // > greater than
    Eqls, // = equal sign

    Atsm, // @ at symbol
    Udsc, // _ underscore Usnmとは別に一度定義しておく
    Tild, // ~ tilde
    Bkqt, // ` back quote
    */
}

impl fmt::Debug for Tokn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Tokn::Nmbr(ref s) => write!(f, "Nmbr<{}>", s),
            &Tokn::Text(ref s) => write!(f, "Text<{}>", s),
            &Tokn::Name(ref s) => write!(f, "Name<{}>", s),
            &Tokn::Usnm(ref s) => write!(f, "Usnm<{}>", s),

            &Tokn::Dbqt => write!(f, "Dbqt"),

            &Tokn::Lbkt => write!(f, "Lbkt"),
            &Tokn::Rbkt => write!(f, "Rbkt"),
            &Tokn::Lprn => write!(f, "Lprn"),
            &Tokn::Rprn => write!(f, "Rprn"),
            &Tokn::Lcrl => write!(f, "Lcrl"),
            &Tokn::Rcrl => write!(f, "Rcrl"),

            &Tokn::Coln => write!(f, "Coln"),

            &Tokn::Pipe => write!(f, "Pipe"),
            &Tokn::Crrt => write!(f, "Crrt"),
            &Tokn::Dllr => write!(f, "Dllr"),
            &Tokn::Smcl => write!(f, "Smcl"),
            &Tokn::Bang => write!(f, "Bang"),

            // 扱いが不明瞭だがひとまず足しておく
            &Tokn::Plus => write!(f, "Plus"),
            &Tokn::Star => write!(f, "Star"),
            &Tokn::Stop => write!(f, "Stop"),
            &Tokn::Bksl => write!(f, "Bksl"),

            _ => write!(f, "????"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    // Unknown,
    First,
    InvalidText(String),
    InvalidNumber(String),
    InvalidName(String),
    EOF,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No matching cities with a population were found.")
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        "not found"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerStat {
    Normal,
    ZeroNumber,
    InnerNumber,
    InnerText,
    FinishText,
    InnerName,
    AfterUnderscore,
}

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    stat: LexerStat,
    token_bytes: Vec<u8>,
    token: Result<Tokn, LexerError>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter, current_char: c, stat: LexerStat::Normal,
            token_bytes: vec![], token: Err(LexerError::First), eof: false }
    }

    pub fn next_token(&mut self) -> Result<Tokn, LexerError> {
        self.reset_token();

        loop {
            // println!("self.eof: {:?}", self.eof);
            match self.token {
                // _ if self.eof => return Err(LexerError::EOF),
                Err(LexerError::First) => self.scan(),
                // _ if self.eof => return Err(LexerError::EOF),
                _ => { break; },
            }
        }

        (&self.token).clone()
    }

    fn scan(&mut self) {
        let c = self.current_char;

        match self.stat {
            LexerStat::Normal => {
                match c as char {
                    _ if self.eof => self.finish_error(LexerError::EOF), // 普通にEOF

                    '(' => self.delimit(c, Tokn::Lprn),
                    ')' => self.delimit(c, Tokn::Rprn),
                    '[' => self.delimit(c, Tokn::Lbkt),
                    ']' => self.delimit(c, Tokn::Rbkt),
                    '{' => self.delimit(c, Tokn::Lcrl),
                    '}' => self.delimit(c, Tokn::Rcrl),

                    ':' => self.delimit(c, Tokn::Coln),

                    '|' => self.delimit(c, Tokn::Pipe),
                    '^' => self.delimit(c, Tokn::Crrt),
                    '$' => self.delimit(c, Tokn::Dllr),
                    ';' => self.delimit(c, Tokn::Smcl),
                    '!' => self.delimit(c, Tokn::Bang),

                    '_' => self.advance(c, LexerStat::AfterUnderscore),

                    '"' => {
                        self.delimit(c, Tokn::Dbqt);
                        self.stat = LexerStat::InnerText;
                    },

                    // 扱いが不明瞭な記号は、一度Toknにして、判断をParserに丸投げする
                    '+' => self.delimit(c, Tokn::Plus),
                    '.' => self.delimit(c, Tokn::Stop),
                    '*' => self.delimit(c, Tokn::Star),
                    '\\' => self.delimit(c, Tokn::Bksl),

                    '0' => self.advance(c, LexerStat::ZeroNumber),

                    _ if self.is_digit(c) => self.advance(c, LexerStat::InnerNumber),

                    _ if self.is_alphabetic(c) => self.advance(c, LexerStat::InnerName), // 独自のalphabet判定メソッドを使っているので注意

                    _ if self.is_whitespace(c) => self.consume_char(),

                    _ => self.consume_char(),
                }
            },

            LexerStat::ZeroNumber => {
                match c as char {
                    _ if self.eof => self.finish_number(), // 0でファイルが終わってもOK

                    '[' | ']' => self.finish_number(),

                    _ if self.is_whitespace(c) => self.finish_number(),

                    _ => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    },
                }
            },

            LexerStat::InnerNumber => {
                // println!("LexerStat::InnerNumber");
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerStat::InnerNumber),
                    Some("finish") => self.finish_number(),
                    Some(&_) | None => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    }
                }
            },

            LexerStat::InnerText => {
                match c as char {
                    '"' => self.finish_text(),
                    _ if self.eof => {
                        // 文字列の途中でファイルが終わってしまった
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidText(s));
                    }
                    _ => self.advance(c, LexerStat::InnerText),
                }
            },

            LexerStat::FinishText => {
                // println!("LexerStat::FinishText");
                self.finish(Ok(Tokn::Dbqt), LexerStat::Normal);
            },

            LexerStat::InnerName => {
                match c as char {
                    // 途中で終わってもそこまでのNameとみなす
                    _ if self.eof => {
                        self.finish_name();
                    },

                    // 区切り文字ならここでNameを終わらせる必要がある
                    // ただし、全ての区切り文字がここで判断されるわけではない
                    '[' | ']' | '(' | ')' | '{' | '}' | ':' => self.finish_name(),

                    _ if self.is_whitespace(c) => self.finish_name(),

                    // 英数字なら、引き続き次の文字
                    _ if self.is_alphabetic(c) => self.advance(c, LexerStat::InnerName),
                    _ if self.is_digit(c) => self.advance(c, LexerStat::InnerName),

                    // それ以外はエラー
                    _ => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidName(s));
                    }
                }
            },

            LexerStat::AfterUnderscore => {
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerStat::AfterUnderscore),
                    Some("finish") => self.finish_underscore_number(),
                    Some(&_) | None => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    },
                }
            },
        }
    }

    /// lex token like number
    /// e.g. 63 845
    /// not for 07246(=start with 0) 623452w(=end with no digit char)
    fn lex_numeric(&mut self, c: u8) -> Option<&str> {
        // println!("lex_numeric");
        match c as char {
            _ if self.eof => {
                // println!("eof finish");
                Some("finish") // 数字の並びの途中で終わってもそこまでの数値とみなす
            },

            _ if self.is_digit(c) => Some("next"),

            // 区切り文字ならここで数値を終わらせる必要がある
            // ただし、全ての区切り文字がここで判断されるわけではない
            '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),

            _ if self.is_whitespace(c) => Some("finish"),

            _ => None,
        }
    }

    fn finish_text(&mut self) {
        self.consume_char();
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Text(s)), LexerStat::FinishText);
    }

    fn finish_number(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Nmbr(s)), LexerStat::Normal);
    }

    fn finish_underscore_number(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Usnm(s)), LexerStat::Normal);
    }

    fn finish_name(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Name(s)), LexerStat::Normal);
    }

    fn finish_error(&mut self, e: LexerError) {
        // let s = self.token_bytes.clone();
        self.finish(Err(e), LexerStat::Normal);
    }

    fn advance(&mut self, c: u8, next: LexerStat) {
        self.token_bytes.push(c);
        // println!("c: {:?}, self.token_bytes.as_bytes(): {:?}", c, self.token_bytes.as_bytes());
        self.consume_char();
        self.stat = next;
    }

    fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerStat::Normal);
    }

    fn reset_token(&mut self) {
        self.token_bytes.clear();
        self.token = Err(LexerError::First);
    }

    fn finish(&mut self, tokn: Result<Tokn, LexerError>, next: LexerStat) {
        self.token = tokn;
        self.stat = next;
    }

    fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            // self.current_char = c as char;
            self.current_char = c;
            println!("c: {:?}, self.current_char: {:?}", c, self.current_char);
        } else {
            self.eof = true;
        }
    }

    fn get_token_string(&self) -> String {
        String::from_utf8(self.token_bytes.clone()).expect("Found invalid UTF-8")
    }

    fn is_whitespace(&self, c: u8) -> bool {
        c == ' ' as u8 || c == '\t' as u8 || c == '\n' as u8 || c == '\t' as u8
    }

    fn is_digit(&self, c: u8) -> bool {
        c >= '0' as u8 && c <= '9' as u8
    }

    fn is_alphabetic(&self, c: u8) -> bool {
        (c >= 'A' as u8 && c <= 'Z' as u8) || (c >= 'a' as u8 && c <= 'z' as u8)
    }
}
