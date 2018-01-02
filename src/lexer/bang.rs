use core::*;
use lexer::*;
use util::*;

#[derive(Debug)]
pub struct BangScanner;


impl Scanner for BangScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        match state {
            State::Normal => {
                match c {
                    b'!' => push_into_mode!(AfterBang),
                    _ => go_ahead!(),
                }
            },

            State::InnerName => {
                match c {
                    b'!' => delimite_into_mode!(InnerBang), // 一度区切る
                    _ => go_ahead!(),
                }
            },

            State::InnerBang => {
                match c {
                    b'!' => push_into_mode!(AfterBang),
                    _ => go_ahead!(), // 普通はかっこが来るが、構文チェックはparserに任せる
                }
            },

            State::AfterBang => {
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
            State::AfterBang => Some(Tokn::Bang),
            _ => None,
        }
    }
}

#[test]
// #[ignore]
fn text() {
    let scanners: &mut Vec<&Scanner> = &mut vec![&BangScanner];
    lex_from_str("!", "Bang", scanners);
}
