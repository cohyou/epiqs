use core::*;
use lexer::*;

#[derive(Debug)]
pub struct DelimiterScanner;

impl Scanner for DelimiterScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'|' | b';' => push_into_mode!(Delimiter),
                    _    => go_ahead!(),
                }
            },
            State::Delimiter => {
                // 何が来ても終了
                match c {
                    0 => finish!()/*ScanResult::EOF*/,
                    _ => delimite!(),
                }
            },
            _ => go_ahead!(),
        }
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        match token_string.as_ref() {
            "|" => Some(Tokn::Pipe),
            ";" => Some(Tokn::Smcl),
            _ => None,
        }
    }
}

#[test]
#[ignore]
fn test_pipe() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&DelimiterScanner];
    lex_from_str("|", "Pipe", scanners);
}

#[test]
fn test_semicolon() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&DelimiterScanner];
    lex_from_str(";", "Smcl", scanners);
}
