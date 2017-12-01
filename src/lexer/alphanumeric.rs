use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct AlphanumericScanner;

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
                    0 => ScanResult::EOF,
                    _ if is_whitespace(c) => finish!(),
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
fn test() {
    let scanners: &Vec<&Scanner> = &vec![&AlphanumericScanner];
    lex_from_str("Abc", vec!["Otag<Abc>"], scanners);
    lex_from_str("abc", vec!["Chvc<abc>"], scanners);
}
