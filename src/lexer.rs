use super::token::Tokn;
use super::lexer_error::LexerError;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerState {
    Normal,
    InnerName,

    // ZeroNumber,
    // InnerNumber,
    // InnerText,
    // FinishText,
    // AfterUnderscore,
    // AfterDot,
    // InnerComment,
}

pub struct Lexer<'a> {
    iter: &'a mut Iterator<Item=u8>,
    current_char: u8,
    state: LexerState,
    token_bytes: Vec<u8>,
    token: Result<Tokn, LexerError>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new<I>(iter: &'a mut I) -> Lexer
    where I: Iterator<Item=u8> {
        let c = iter.next().unwrap();
        Lexer { iter: iter, current_char: c, state: LexerState::Normal,
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

        // println!("{:?} {:?}", c as char, self.state);

        match self.state {
            LexerState::Normal => {
                match c {
                    _ if self.eof => self.finish_error(LexerError::EOF), // 普通にEOF

                    b'|' => self.delimit(c, Tokn::Pipe),

                    b':' => self.delimit(c, Tokn::Coln),


                    // b';' => self.delimit(c, Tokn::Smcl),

                    // b'(' => self.delimit(c, Tokn::Lprn),
                    // b')' => self.delimit(c, Tokn::Rprn),

                    /*
                    b'.' => {
                        self.delimit(c, Tokn::Stop);
                        self.state = LexerState::AfterDot;
                    },
                    */

                    // b'[' => self.delimit(c, Tokn::Lbkt),
                    // b']' => self.delimit(c, Tokn::Rbkt),
                    // b'{' => self.delimit(c, Tokn::Lcrl),
                    // b'}' => self.delimit(c, Tokn::Rcrl),

                    // b'_' => self.advance(c, LexerState::AfterUnderscore),

                    /*
                    b'"' => {
                        self.delimit(c, Tokn::Dbqt);
                        self.state = LexerState::InnerText;
                    },
                    */

                    // b'^' => self.delimit(c, Tokn::Crrt),
                    // b'$' => self.delimit(c, Tokn::Dllr),
                    // b'!' => self.delimit(c, Tokn::Bang),

                    // b'@' => self.delimit(c, Tokn::Atsm),

                    // b',' => self.delimit(c, Tokn::Comm),

                    // // 以下、本来は2文字目に来るものばかり
                    // b'#' => self.delimit(c, Tokn::Hash),
                    // b'\\' => self.delimit(c, Tokn::Bksl),
                    // b'+' => self.delimit(c, Tokn::Plus),
                    // b'%' => self.delimit(c, Tokn::Pcnt),
                    // b'?' => self.delimit(c, Tokn::Qstn),
                    // b'&' => self.delimit(c, Tokn::Amps),

                    // 現在テストの中には、乗算記号としてのみ出現している
                    // b'*' => self.delimit(c, Tokn::Star),

                    // b'0' => self.advance(c, LexerState::ZeroNumber),

                    // _ if self.is_digit(c) => self.advance(c, LexerState::InnerNumber),

                    _ if self.is_alphabetic(c) => self.advance(c, LexerState::InnerName), // 独自のalphabet判定メソッドを使っているので注意

                    _ if self.is_whitespace(c) => self.consume_char(),

                    _ => self.consume_char(),
                }
            },

            /*
            LexerState::ZeroNumber => {
                match c {
                    _ if self.eof => self.finish_number(), // 0でファイルが終わってもOK

                    b'[' | b']' => self.finish_number(),

                    _ if self.is_whitespace(c) => self.finish_number(),

                    _ => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    },
                }
            },
            */

            /*
            LexerState::InnerNumber => {
                // println!("LexerState::InnerNumber");
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerState::InnerNumber),
                    Some("finish") => self.finish_number(),
                    Some(&_) | None => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    }
                }
            },
            */

            /*
            LexerState::InnerText => {
                match c {
                    b'"' => self.finish_text(),

                    _ if self.eof => {
                        // 文字列の途中でファイルが終わってしまった
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidText(s));
                    }

                    _ => self.advance(c, LexerState::InnerText),
                }
            },
            */

            /*
            LexerState::FinishText => {
                // println!("LexerState::FinishText");
                self.finish(Ok(Tokn::Dbqt), LexerState::Normal);
            },
            */

            LexerState::InnerName => {
                match c {
                    // 途中で終わってもそこまでのNameとみなす
                    _ if self.eof => {
                        self.finish_charactor_vector();
                    },

                    // 区切り文字ならここでNameを終わらせる必要がある
                    // ただし、全ての区切り文字がここで判断されるわけではない
                    b'[' | b']' | b'(' | b')' | b'{' | b'}' | b':' | b',' => self.finish_charactor_vector(),

                    _ if self.is_whitespace(c) => self.finish_charactor_vector(),

                    // 英数字なら、引き続き次の文字
                    _ if self.is_alphabetic(c) => self.advance(c, LexerState::InnerName),
                    _ if self.is_digit(c) => self.advance(c, LexerState::InnerName),

                    // それ以外はエラー
                    _ => {
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidName(s));
                    }
                }
            },

            /*
            LexerState::AfterUnderscore => {
                match self.lex_numeric(c) {
                    Some("next") => self.advance(c, LexerState::AfterUnderscore),
                    Some("finish") => self.finish_underscore_number(),
                    Some(&_) | None => {
                        self.token_bytes.push(c);
                        // 最初の_は取り除く
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidNumber(s));
                    },
                }
            },
            */

            LexerState::AfterDot => {
                match c {
                    // ..はコメント開始 改行まで
                    /*
                    b'.' => {
                        self.delimit(c, Tokn::Stop);
                        self.state = LexerState::InnerComment;
                    },
                    */

                    // `:` cons
                    // b'%' => self.delimit(c, Tokn::Pcnt),
                    // b'?' => self.delimit(c, Tokn::Qstn),
                    // b'&' => self.delimit(c, Tokn::Amps),
                    // b'@' => self.delimit(c, Tokn::Atsm),
                    // b'#' => self.delimit(c, Tokn::Hash),
                    // b'$' => self.delimit(c, Tokn::Dllr),
                    // b'!' => self.delimit(c, Tokn::Bang),
                    // b'+' => self.delimit(c, Tokn::Plus),
                    // b'\\' => self.delimit(c, Tokn::Bksl),

                    // `%+` define
                    // '^' => self.delimit(c, Tokn::Crrt), metadataは未実装

                    _ => {
                        // 上記のもの以外がきたらひとまずはエラー
                        self.token_bytes.push(c);
                        let s = self.get_token_string();
                        self.finish_error(LexerError::InvalidTag(s));
                    }
                }
            },

            LexerState::InnerComment => {
                match c {
                    b'\n' => {
                        self.finish_charactor_vector();
                    }
                    _ if self.eof => self.finish_charactor_vector(),
                    _ => self.advance(c, LexerState::InnerComment),
                }
            }
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

    fn finish_charactor_vector(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Chvc(s)), LexerState::Normal);
    }

    /*
    fn finish_text(&mut self) {
        self.consume_char();
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Chvc(s)), LexerState::FinishText);
    }
    */

    /*
    fn finish_number(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Nmbr(s)), LexerState::Normal);
    }
    fn finish_underscore_number(&mut self) {
        let s = self.get_token_string().replace("_", "");
        self.finish(Ok(Tokn::Usnm(s)), LexerState::Normal);
    }
    */
    fn finish_error(&mut self, e: LexerError) {
        // let s = self.token_bytes.clone();
        self.finish(Err(e), LexerState::Normal);
    }

    fn advance(&mut self, c: u8, next: LexerState) {
        self.token_bytes.push(c);
        // println!("c: {:?}, self.token_bytes.as_bytes(): {:?}", c, self.token_bytes.as_bytes());
        self.consume_char();
        self.state = next;
    }

    fn delimit(&mut self, c: u8, t: Tokn) {
        self.token_bytes.push(c);
        self.consume_char();
        self.finish(Ok(t), LexerState::Normal);
    }

    fn reset_token(&mut self) {
        self.token_bytes.clear();
        self.token = Err(LexerError::First);
    }

    fn finish(&mut self, tokn: Result<Tokn, LexerError>, next: LexerState) {
        self.token = tokn;
        self.state = next;
    }

    fn consume_char(&mut self) {
        if let Some(c) = self.iter.next() {
            // self.current_char = c as char;
            self.current_char = c;
            // println!("c: {:?}, self.current_char: {:?}", c, self.current_char);
        } else {
            self.eof = true;
        }
    }

    fn get_token_string(&self) -> String {
        String::from_utf8(self.token_bytes.clone()).expect("Found invalid UTF-8")
    }

    fn is_whitespace(&self, c: u8) -> bool {
        c == b' ' || c == b'\t' || c == b'\n' || c == b'\t'
    }

    fn is_digit(&self, c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn is_alphabetic(&self, c: u8) -> bool {
        is_alphabetic_uppercase() || is_alphabetic_lowercase()
    }

    fn is_alphabetic_uppercase() -> bool {
        (c >= b'A' && c <= b'Z')
    }

    fn is_alphabetic_lowercase() -> bool {
        (c >= b'a' && c <= b'z')
    }

}
