use super::Lexer;
use util::*;

impl<'a> Lexer<'a> {
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

/*    fn finish_underscore_number(&mut self) {
        let s = self.get_token_string().replace("_", "");
        self.finish(Ok(Tokn::Usnm(s)), State::Normal);
    }
    */
}
