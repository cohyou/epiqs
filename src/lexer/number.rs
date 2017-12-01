use ::token::Tokn;
use ::util::*;
use lexer::*;

#[derive(Debug)]
struct ZeroScanner;

impl Scanner for ZeroScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Continue(vec![]);

        if state == State::Normal && c == b'0' {
            let opt = vec![
               ScanOption::PushCharToToken,
               ScanOption::ChangeState(State::ZeroNumber),
            ];
            res = ScanResult::Continue(opt);
        } else if state == State::ZeroNumber {
            res = match c {
                0 => ScanResult::EOF,
                _ if is_whitespace(c) => {
                    let opts = vec![ScanOption::ChangeState(State::Normal)];
                    ScanResult::Finish(opts)
                },
                _ => ScanResult::Error,
            };
        }
        res
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        Some(Tokn::Nmbr(token_string))
    }
}

#[derive(Debug)]
struct IntegerScanner;

impl Scanner for IntegerScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Continue(vec![]);

        if state == State::Normal && (c >= b'1' && c <= b'9') {
            let opt = vec![
               ScanOption::PushCharToToken,
               ScanOption::ChangeState(State::InnerNumber),
            ];
            res = ScanResult::Continue(opt);
        } else if state == State::InnerNumber {
            res = match c {
                0 => ScanResult::EOF,
                _ if c >= b'0' && c <= b'9' =>
                    ScanResult::Continue(vec![ScanOption::PushCharToToken]),

                _ if is_whitespace(c) => {
                    let opts = vec![ScanOption::ChangeState(State::Normal)];
                    ScanResult::Finish(opts)
                },
                // 区切り文字ならここで数値を終わらせる必要がある
                // ただし、全ての区切り文字がここで判断されるわけではない
                // '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),

                _ => ScanResult::Error,
            };
        }
        res
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        Some(Tokn::Nmbr(token_string))
    }
}

#[test]
fn test() {
    let scanners: &Vec<&Scanner> = &vec![&ZeroScanner, &IntegerScanner];
    lex_from_str("0", vec!["Nmbr<0>"], scanners);
    lex_from_str("1", vec!["Nmbr<1>"], scanners);
    lex_from_str("12", vec!["Nmbr<12>"], scanners);
}
