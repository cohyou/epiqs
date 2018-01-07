use core::*;
use lexer::*;

#[derive(Debug)]
pub struct StopScanner;


impl Scanner for StopScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'.' => push_into_mode!(AfterStop),
                    _ => go_ahead!(),
                }
            },

            State::InnerName | State::InnerNumber => {
                match c {
                    b'.' => delimite_into_mode!(InnerStop), // 一度区切る
                    _ => go_ahead!(),
                }
            },

            State::InnerStop => {
                match c {
                    b'.' => push_into_mode!(AfterStop),
                    _ => go_ahead!(), // 普通はかっこが来るが、構文チェックはparserに任せる
                }
            },

            State::AfterStop => {
                // 何が来ても終了
                match c {
                    0 => finish!(),
                    _ => delimite!(),
                }
            },

            _ => go_ahead!(),
        }
    }

    fn return_token(&self, state: State, token_string: String) -> Option<Tokn> {
        match state {
            State::InnerName => Some(Tokn::Chvc(token_string)),
            State::InnerNumber => Some(Tokn::Nmbr(token_string)),
            State::AfterStop => Some(Tokn::Stop),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn stop() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&StopScanner];
    lex_from_str(".", "Stop", scanners);
}
