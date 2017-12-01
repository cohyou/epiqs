use core::*;
use lexer::*;

#[derive(Debug)]
pub struct DelimiterScanner;

impl Scanner for DelimiterScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'|' => push_into_mode!(Delimiter),
                    _    => go_ahead!(),
                }
            },
            State::Delimiter => {
                // 何が来ても終了
                match c {
                    0 => ScanResult::EOF,
                    _ => delimite!(),
                }
            },
            _ => go_ahead!(),
        }
    }

    fn return_token(&self, _state: State, token_string: String) -> Option<Tokn> {
        if token_string == "|" {
            Some(Tokn::Pipe)
        } else {
            None
        }
    }
}

#[test]
#[ignore]
fn test() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&DelimiterScanner];
    lex_from_str("|", vec!["Pipe"], scanners);
}
