use lexer::*;

#[derive(Debug)]
pub struct EOFScanner;

impl Scanner for EOFScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        if state == State::Normal && c == 0 {
            ScanResult::EOF
        } else {
            go_ahead!()
        }
    }
}

#[test]
#[ignore]
fn test() {
    lex_from_str("", vec![], &vec![&EOFScanner]);
}
