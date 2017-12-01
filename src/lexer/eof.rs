use lexer::*;

#[derive(Debug)]
pub struct EOFScanner;

impl Scanner for EOFScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        if state == State::Normal {
            match c {
                0 => ScanResult::EOF,
                _ => next_char!(),
            }
        } else {
            next_char!()
        }
    }
}

#[test]
#[ignore]
fn test() {
    lex_from_str("", "", &mut vec![&EOFScanner]);
}
