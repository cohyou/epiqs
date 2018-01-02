use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct TextScanner;

impl Scanner for TextScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'"' => finish_into_mode!(InnerText),
                    _ => go_ahead!(),
                }
            },
            State::InnerText => {
                // println!("State::InnerText: {:?}", c);
                match c {
                    0 => ScanResult::Error,
                    b'"' => delimite_into_mode!(FinishText),
                    _ => push!(),
                }
            },
            State::FinishText => {
                match c {
                    0 => ScanResult::Error,
                    b'"' => finish!(),
                    _ => ScanResult::Error,
                }
            }
            _ => go_ahead!(),
        }
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::Normal | State::FinishText => Some(Tokn::Dbqt),
            State::InnerText => Some(Tokn::Chvc(token_string)),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn text() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&TextScanner];
    lex_from_str("\"Abc@ | q5djp4m3 def\"", "Dbqt Chvc<Abc@ | q5djp4m3 def> Dbqt", scanners);
}
