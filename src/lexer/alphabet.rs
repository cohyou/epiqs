use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct AlphabetScanner;

impl AlphabetScanner {
    fn is_first_otag_letter(&self, c: u8) -> bool {
        // println!("is_first_otag_letter: {:?}", c);
        (c >= b'A' && c <= b'Z') || self.is_otag_sign(c)
    }

    fn is_otag_sign(&self, c: u8) -> bool {
        c == b':' || c == b'#'
    }

    fn is_first_name_letter(&self, c: u8) -> bool {
        (c >= b'a' && c <= b'z')
    }
}

impl Scanner for AlphabetScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    _ if self.is_first_otag_letter(c) => push_into_mode!(InnerOtag),
                    _ if self.is_first_name_letter(c) => push_into_mode!(InnerName),
                    _ => go_ahead!(),
                }
            },
            State::InnerOtag | State::InnerName => {
                match c {
                    0 => finish!(),
                    _ if is_whitespace(c) => finish!(),
                    _ if is_token_end_delimiter(c) => delimite!(),
                    _ if is_alphanumeric(c) => push!(),
                    _ => ScanResult::Error,
                }
            },
            _ => go_ahead!(),
        }
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
#[ignore]
fn test_otag() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&AlphabetScanner];
    lex_from_str("Abc", "Otag<Abc>", scanners);
}

#[test]
#[ignore]
fn test_character_vector() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&AlphabetScanner];
    lex_from_str("abc", "Chvc<abc>", scanners);
}
