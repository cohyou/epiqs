use core::*;
use lexer::*;

#[derive(Debug)]
pub struct ColonScanner;


impl Scanner for ColonScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b':' => push_into_mode!(AfterColon),
                    _ => go_ahead!(),
                }
            },

            State::InnerName | State::ZeroNumber | State::InnerNumber => {
                match c {
                    b':' => delimite_into_mode!(InnerColon), // 一度区切る
                    _ => go_ahead!(),
                }
            },

            State::InnerColon => {
                match c {
                    b':' => push_into_mode!(AfterColon),
                    _ => go_ahead!(), // 普通はかっこが来るが、構文チェックはparserに任せる
                }
            },

            State::AfterColon => {
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
            State::ZeroNumber | State::InnerNumber => Some(Tokn::Nmbr(token_string)),
            State::AfterColon => Some(Tokn::Coln),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn colon() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&ColonScanner];
    lex_from_str(":", "Coln", scanners);
}
