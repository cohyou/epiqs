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
                0 => finish!(),
                _ if is_whitespace(c) => finish!(),
                _ if is_token_end_delimiter(c) => delimite!(),
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
                    0 => finish!(),
                    _ if is_whitespace(c) => finish!(),
                    _ if is_token_end_delimiter(c) => delimite!(),
                    _ if c >= b'0' && c <= b'9' => push!(),
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
fn test_zero() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&ZeroScanner, &IntegerScanner];
    lex_from_str("0", "Nmbr<0>", scanners);
}

#[test]
#[ignore]
fn test_one_digit_integer() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&ZeroScanner, &IntegerScanner];
    lex_from_str("1", "Nmbr<1>", scanners);
}

#[test]
#[ignore]
fn test_multiple_digits_integer() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&ZeroScanner, &IntegerScanner];
    lex_from_str("12", "Nmbr<12>", scanners);
}
