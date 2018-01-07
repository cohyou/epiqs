use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct OtagScanner;

impl OtagScanner {
    fn is_first_otag_letter(&self, c: u8) -> bool {
        // println!("is_first_otag_letter: {:?}", c);
        (c >= b'A' && c <= b'Z') || self.is_otag_sign(c)
    }

    fn is_otag_sign(&self, c: u8) -> bool {
        c == b':' || c == b'#' || c == b'%' || c == b'\\' ||
        c == b'@' || c == b'~' || c == b'>' || c == b'!' || c == b'.' ||
        c == b'?' || c == b'='
    }
}

impl Scanner for OtagScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'|' | b'^' | b'\'' => push_into_mode!(Dispatcher),
                    _    => go_ahead!(),
                }
            },
            State::Dispatcher => {
                // 何が来ても終了
                match c {
                    0 => ScanResult::Error,
                    _ => delimite_into_mode!(AfterDispatcher),
                }
            },
            State::AfterDispatcher => {
                match c {
                    0 => ScanResult::Error,
                    _ if self.is_first_otag_letter(c) => push_into_mode!(InnerOtag),
                    b'[' => push_into_mode!(InnerSpecialOtag),
                    _ => go_ahead!(),
                }
            },
            State::InnerSpecialOtag => {
                // 何が来ても終了
                match c {
                    0 => ScanResult::Error,
                    _ => delimite!(),
                }
            },
            State::InnerOtag => {
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
            State::Dispatcher => {
                match token_string.as_ref() {
                    "|" => Some(Tokn::Pipe),
                    "^" => Some(Tokn::Crrt),
                    "'" => Some(Tokn::Sgqt),
                    _ => None,
                }
            },
            State::InnerSpecialOtag => {
                match token_string.as_ref() {
                    "[" => Some(Tokn::Lbkt),
                    _ => None,
                }
            }
            State::InnerOtag => Some(Tokn::Otag(token_string)),
            _ => None,
        }
    }
}

#[test]
fn single_quote() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&OtagScanner];
    lex_from_str("'A", "Sgqt Otag<A>", scanners);
}

#[test]
// #[ignore]
fn otag() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&OtagScanner];
    lex_from_str("|Abc", "Pipe Otag<Abc>", scanners);
}
