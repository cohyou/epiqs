use ::token::Tokn;
use lexer::*;

#[derive(Debug)]
struct DelimiterScanner;

impl Scanner for DelimiterScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Continue(vec![]);
        match state {
            State::Normal => {
                match c {
                    b'|' => {
                        let opt = vec![
                           ScanOption::PushCharToToken,
                           ScanOption::ChangeState(State::Delimiter),
                        ];
                        res = ScanResult::Continue(opt);
                    },
                    _ => {},
                }
            },

            State::Delimiter => {
                // 何が来ても終了
                let opts = vec![ScanOption::ChangeState(State::Normal)];
                match c {
                    0 => { res = ScanResult::EOF; },
                    _ => { res = ScanResult::Finish(opts); },
                }
            },

            _ => {},
        }
        res
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
fn test() {
    let scanners: &Vec<&Scanner> = &vec![&DelimiterScanner];
    lex_from_str("|", vec!["Pipe"], scanners);
}
