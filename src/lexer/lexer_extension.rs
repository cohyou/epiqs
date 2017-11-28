use super::Lexer;
use ::util::*;

impl<'a> Lexer<'a> {
    fn scan_zero_number() {
        /*
        State::ZeroNumber => {
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
    }

    fn scan_inner_number() {
        /*
        State::InnerNumber => {
            // println!("State::InnerNumber");
            match self.lex_numeric(c) {
                Some("next") => self.advance(c, State::InnerNumber),
                Some("finish") => self.finish_number(),
                Some(&_) | None => {
                    self.token_bytes.push(c);
                    let s = self.get_token_string();
                    self.finish_error(LexerError::InvalidNumber(s));
                }
            }
        },
        */
    }

    fn scan_inner_text() {
        /*
        State::InnerText => {
            match c {
                b'"' => self.finish_text(),

                _ if self.eof => {
                    // 文字列の途中でファイルが終わってしまった
                    self.token_bytes.push(c);
                    let s = self.get_token_string();
                    self.finish_error(LexerError::InvalidText(s));
                }

                _ => self.advance(c, State::InnerText),
            }
        },
        */
    }

    fn scan_finish_text() {
        /*
        State::FinishText => {
            // println!("State::FinishText");
            self.finish(Ok(Tokn::Dbqt), State::Normal);
        },
        */
    }

    fn scan_after_underscore() {
        /*
        State::AfterUnderscore => {
            match self.lex_numeric(c) {
                Some("next") => self.advance(c, State::AfterUnderscore),
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
    }

    fn scan_after_dot() {
        /*
        State::AfterDot => {
            match c {
                // ..はコメント開始 改行まで
                b'.' => {
                    self.delimit(c, Tokn::Stop);
                    self.state = State::InnerComment;
                },

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
        */
    }

    fn scan_inner_comment() {
        /*
        State::InnerComment => {
            match c {
                b'\n' => {
                    self.finish_charactor_vector();
                }
                _ if self.eof => self.finish_charactor_vector(),
                _ => self.advance(c, State::InnerComment),
            }
        }
        */
    }

    // read macro的なもの
    fn scan_special_charactor(&mut self) {
        // b'(' => self.delimit(c, Tokn::Lprn),
        // b')' => self.delimit(c, Tokn::Rprn),

        // b'[' => self.delimit(c, Tokn::Lbkt),
        // b']' => self.delimit(c, Tokn::Rbkt),
        // b'{' => self.delimit(c, Tokn::Lcrl),
        // b'}' => self.delimit(c, Tokn::Rcrl),

        // b'_' => self.advance(c, State::AfterUnderscore),
    }

    fn scan_special_tag(&mut self) {
        // b':' => self.delimit(c, Tokn::Coln),
        /*
        b'.' => {
            self.delimit(c, Tokn::Stop);
            self.state = State::AfterDot;
        },
        */
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
    }

    fn scan_literal(&mut self) {
        // b';' => self.delimit(c, Tokn::Smcl),
        /*
        b'"' => {
            self.delimit(c, Tokn::Dbqt);
            self.state = State::InnerText;
        },
        */
        // b'0' => self.advance(c, State::ZeroNumber),
        // _ if self.is_digit(c) => self.advance(c, State::InnerNumber),
    }

    /// lex token like number
    /// e.g. 63 845
    /// not for 07246(=start with 0) 623452w(=end with no digit char)
    fn scan_numeric(&mut self, c: u8) -> Option<&str> {
        // println!("lex_numeric");
        match c as char {
            _ if self.eof => {
                // println!("eof finish");
                Some("finish") // 数字の並びの途中で終わってもそこまでの数値とみなす
            },

            _ if is_digit(c) => Some("next"),

            // 区切り文字ならここで数値を終わらせる必要がある
            // ただし、全ての区切り文字がここで判断されるわけではない
            '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),

            _ if is_whitespace(c) => Some("finish"),

            _ => None,
        }
    }

    /*
    fn finish_text(&mut self) {
        self.consume_char();
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Chvc(s)), State::FinishText);
    }
    */

    /*
    fn finish_number(&mut self) {
        let s = self.get_token_string();
        self.finish(Ok(Tokn::Nmbr(s)), State::Normal);
    }
    fn finish_underscore_number(&mut self) {
        let s = self.get_token_string().replace("_", "");
        self.finish(Ok(Tokn::Usnm(s)), State::Normal);
    }
    */
}
