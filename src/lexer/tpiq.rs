use ::token::Tokn;
use ::util::*;
use lexer::*;
// use lexer::basic::*;

struct TpiqScanner;

impl TpiqScanner {
    fn is_first_otag_letter(&self, c: u8) -> bool {
        (c >= b'A' && c <= b'Z') || self.is_otag_sign(c)
    }

    fn is_otag_sign(&self, c: u8) -> bool {
        c == b':'
        // 区切り文字ならここでNameを終わらせる必要がある
        // ただし、全ての区切り文字がここで判断されるわけではない
        // b'[' | b']' | b'(' | b')' | b'{' | b'}' | b':' | b',' => self.finish_with_state(state),
    }
}

impl Scanner for TpiqScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Continue(vec![]);

        if state == State::Normal {
            if c == b'|' {
                res = ScanResult::Finish(vec![]);
            } else if self.is_first_otag_letter(c) {
                let opt = vec![
                   ScanOption::ChangeState(State::InnerTag),
                ];
                res = ScanResult::Continue(opt);
            }
        } else if state == State::InnerTag {
            match c {
                // 途中で終わってもそこまでのOtagとみなす
                0 => {
                    res = ScanResult::Finish(vec![]);
                },

                // 空白が来たら区切る
                _ if is_whitespace(c) => {
                    let opts = vec![ScanOption::ChangeState(State::Normal)];
                    res = ScanResult::Finish(opts);
                },

                // 英数字なら、引き続き次の文字
                _ if is_alphanumeric(c) => {
                    res = ScanResult::Continue(vec![ScanOption::PushCharToToken]);
                },

                // それ以外はエラー
                _ => { res = ScanResult::Error },
            }
        }

        res
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::Normal => Some(Tokn::Pipe),
            State::InnerTag => Some(Tokn::Otag(token_string)),
            _ => None,
        }

    }

    fn s(&self) -> String {
        "TpiqScanner".to_string()
    }
}

#[test]
fn test() {
    // let scanners: &Vec<&Scanner> = &vec![&TpiqScanner];
    // lex_from_str("|Abc", vec!["Pipe", "Otag<Abc>"], scanners);
}
