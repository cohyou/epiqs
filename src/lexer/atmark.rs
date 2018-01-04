use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct AtmarkScanner;


impl Scanner for AtmarkScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'@' => finish!(),
                    _ => go_ahead!(),
                }
            },
            _ => go_ahead!(),
        }
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::Normal => Some(Tokn::Atsm),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn atmark() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&AtmarkScanner];
    lex_from_str("@", "Atsm", scanners);
}
