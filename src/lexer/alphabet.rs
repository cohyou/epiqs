use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct AlphabetScanner;

impl AlphabetScanner {
    fn is_first_name_letter(&self, c: u8) -> bool {
        (c >= b'a' && c <= b'z')
    }
}

impl Scanner for AlphabetScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    _ if self.is_first_name_letter(c) => push_into_mode!(InnerName),
                    _ => go_ahead!(),
                }
            },
            State::InnerName => {
                match c {
                    0 => finish!(),
                    _ if is_whitespace(c) => finish!(),
                    _ if is_token_end_delimiter(c) => delimite!(),
                    _ if is_alphanumeric(c) => push!(),
                    b'-' => push!(),
                    _ => ScanResult::Error,
                }
            },
            _ => go_ahead!(),
        }
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::InnerName => Some(Tokn::Chvc(token_string)),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn character_vector() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&AlphabetScanner];
    lex_from_str("abc", "Chvc<abc>", scanners);
}
