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

#[derive(Debug, Clone, PartialEq)]
pub enum Tokn {
    Nmbr(String), // Number
    Text(String), // Text
    // Name(String),
    Usnm(String), // under score and number (e.g. _0 _34)

    Dbqt, // " double quotation

    Lbkt, // [ left bracket
    Rbkt, // ] right bracket
    Lprn, // ( left parentheses
    Rprn, // ) right parentheses

    Coln, // : colon

    Pipe, // | vertical bar
    Crrt, // ^ carret
    Dllr, // $ dollar
    Smcl, // ; semi colon
    Bang, // ! exclamation

    // Bksl, // back slash

    /*
    Hash,
    BigP,
    BigQ,
    Asterisk,
    Lbrace,
    Rbrace,
    Question,
    Dot,
    */
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    // Unknown,
    First,
    // InvalidChar(char),
    InvalidNumber(String),
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
    AfterUnderscore,
}

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: char,
    stat: LexerStat,
    token_string: String,
    token: Result<Tokn, LexerError>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter, current_char: c as char, stat: LexerStat::Normal,
            token_string: "".to_string(), token: Err(LexerError::First), eof: false }
    }

    pub fn next_token(&mut self) -> Result<Tokn, LexerError> {
        self.reset_token();

        loop {
            match self.token {
                _ if self.eof => return Err(LexerError::EOF),
                Err(LexerError::First) => self.scan(),
                _ => { break; },
            }
        }

        (&self.token).clone()
    }

    fn scan(&mut self) {
        let c = self.current_char;

        match self.stat {
            LexerStat::Normal => {
                match c {
                    '[' => self.delimit(c, Tokn::Lbkt),
                    ']' => self.delimit(c, Tokn::Rbkt),
                    '(' => self.delimit(c, Tokn::Lprn),
                    ')' => self.delimit(c, Tokn::Rprn),

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

                    '0' => self.advance(c, LexerStat::ZeroNumber),

                    _ if c.is_digit(10) => self.advance(c, LexerStat::InnerNumber),

                    _ if c.is_whitespace() => self.consume_char(),

                    _ => self.consume_char(),
                }
            },

            LexerStat::ZeroNumber => {
                match c {
                    '[' | ']' => self.finish_number(),

                    _ if c.is_whitespace() => self.finish_number(),

                    _ => {
                        self.token_string.push(c);
                        self.finish_error();
                    },
                }
            },

            LexerStat::InnerNumber => {
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerStat::InnerNumber),
                    Some("finish") => self.finish_number(),
                    Some(&_) | None => {
                        self.token_string.push(c);
                        self.finish_error();
                    }
                }
            },

            LexerStat::InnerText => {
                match c {
                    '"' => self.finish_text(),
                    _ => self.advance(c, LexerStat::InnerText),
                }
            },

            LexerStat::FinishText => {
                self.finish(Ok(Tokn::Dbqt), LexerStat::Normal);
            },

            LexerStat::AfterUnderscore => {
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerStat::AfterUnderscore),
                    Some("finish") => self.finish_underscore_number(),
                    Some(&_) | None => {
                        self.token_string.push(c);
                        self.finish_error();
                    },
                }
            },
        }
    }

    /// lex token like number
    /// e.g. 63 845
    /// not for 07246(=start with 0) 623452w(=end with no digit char)
    fn lex_numeric(&mut self, c: char) -> Option<&str> {
        match c {
            _ if c.is_digit(10) => Some("next"),

            // 区切り文字ならここで数値を終わらせる必要がある
            // ただし、全ての区切り文字がここで判断されるわけではない
            '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),

            _ if c.is_whitespace() => Some("finish"),

            _ => None,
        }
    }

    fn finish_text(&mut self) {
        self.consume_char();
        let s = self.token_string.clone();
        self.finish(Ok(Tokn::Text(s)), LexerStat::FinishText);
    }

    fn finish_number(&mut self) {
        let s = self.token_string.clone();
        self.finish(Ok(Tokn::Nmbr(s)), LexerStat::Normal);
    }

    fn finish_underscore_number(&mut self) {
        let s = self.token_string.clone();
        self.finish(Ok(Tokn::Usnm(s)), LexerStat::Normal);
    }

    fn finish_error(&mut self) {
        let s = self.token_string.clone();
        self.finish(Err(LexerError::InvalidNumber(s)), LexerStat::Normal);
    }

    fn advance(&mut self, c: char, next: LexerStat) {
        self.token_string.push(c);

        self.consume_char();
        self.stat = next;
    }

    fn delimit(&mut self, c: char, t: Tokn) {
        self.token_string.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerStat::Normal);
    }

    fn reset_token(&mut self) {
        self.token_string = "".to_string();
        self.token = Err(LexerError::First);
    }

    fn finish(&mut self, tokn: Result<Tokn, LexerError>, next: LexerStat) {
        self.token = tokn;
        self.stat = next;
    }

    fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c as char;
        } else {
            self.eof = true;
        }
    }
}
