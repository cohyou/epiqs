use ::token::Tokn;
use ::util::*;
use lexer::*;

#[derive(Debug)]
struct AlphanumericScanner;

impl AlphanumericScanner {
    fn is_first_otag_letter(&self, c: u8) -> bool {
        (c >= b'A' && c <= b'Z') || self.is_otag_sign(c)
    }

    fn is_otag_sign(&self, c: u8) -> bool {
        c == b':'
        // 区切り文字ならここでNameを終わらせる必要がある
        // ただし、全ての区切り文字がここで判断されるわけではない
        // b'[' | b']' | b'(' | b')' | b'{' | b'}' | b':' | b',' => self.finish_with_state(state),
    }

    fn is_first_name_letter(&self, c: u8) -> bool {
        (c >= b'a' && c <= b'z')
    }
}

impl Scanner for AlphanumericScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Continue(vec![]);
        match state {
            State::Normal => {
                match c {
                    _ if self.is_first_otag_letter(c) => {
                        let opt = vec![
                           ScanOption::PushCharToToken,
                           ScanOption::ChangeState(State::InnerOtag),
                        ];
                        res = ScanResult::Continue(opt);
                    },

                    _ if self.is_first_name_letter(c) => {
                        let opt = vec![
                           ScanOption::PushCharToToken,
                           ScanOption::ChangeState(State::InnerName),
                        ];
                        res = ScanResult::Continue(opt);
                    },

                    _ => {},
                }
            },
            State::InnerOtag | State::InnerName => {
                res = match c {
                    // 途中で終わってもそこまでとみなす
                    0 => ScanResult::EOF,

                    // 空白が来たら区切る
                    _ if is_whitespace(c) => {
                        let opt = vec![
                           ScanOption::ChangeState(State::Normal),
                        ];
                        ScanResult::Finish(opt)
                    },

                    // 英数字なら、引き続き次の文字
                    _ if is_alphanumeric(c) =>
                        ScanResult::Continue(vec![ScanOption::PushCharToToken]),

                    // それ以外はエラー
                    _ => ScanResult::Error,
                }
            },
            _ => {},
        }
        res
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::InnerOtag => Some(Tokn::Otag(token_string)),
            State::InnerName => Some(Tokn::Chvc(token_string)),
            _ => None,
        }
    }
}

#[test]
fn test() {
    let scanners: &Vec<&Scanner> = &vec![&AlphanumericScanner];
    lex_from_str("Abc", vec!["Otag<Abc>"], scanners);
    lex_from_str("abc", vec!["Chvc<abc>"], scanners);
}
