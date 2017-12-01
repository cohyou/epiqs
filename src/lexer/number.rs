use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct ZeroScanner;

impl Scanner for ZeroScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        if state == State::Normal && c == b'0' {
            push_into_mode!(ZeroNumber)
        } else if state == State::ZeroNumber {
            match c {
                0 => ScanResult::EOF,
                _ if is_whitespace(c) => finish!(),
                _ => ScanResult::Error,
            }
        } else {
            go_ahead!()
        }
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        Some(Tokn::Nmbr(token_string))
    }
}

#[derive(Debug)]
pub struct IntegerScanner;

impl Scanner for IntegerScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                if c >= b'1' && c <= b'9' {
                    push_into_mode!(InnerNumber)
                } else {
                    go_ahead!()
                }
            },
            State::InnerNumber => {
                match c {
                    0 => ScanResult::EOF,
                    _ if c >= b'0' && c <= b'9' => push!(),
                    _ if is_whitespace(c) => finish!(),
                    // 区切り文字ならここで数値を終わらせる必要がある
                    // ただし、全ての区切り文字がここで判断されるわけではない
                    // '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),
                    _ => ScanResult::Error,
                }
            }
            _ => go_ahead!()
        }
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        Some(Tokn::Nmbr(token_string))
    }
}

#[test]
#[ignore]
fn test() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&ZeroScanner, &IntegerScanner];
    lex_from_str("0", vec!["Nmbr<0>"], scanners);
    lex_from_str("1", vec!["Nmbr<1>"], scanners);
    lex_from_str("12", vec!["Nmbr<12>"], scanners);
}
