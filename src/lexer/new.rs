use ::token::Tokn;
use lexer::{Lexer, State, Error};
use util::is_whitespace;

enum ScanResult {
    Continue,
    Finish,
    Error,
    EOF,
}

trait Scanner {
    fn should_scan(&self, state: State, c: u8) -> bool;
    fn scan(&self, c: u8) -> ScanResult;
    fn return_token(&self, token_string: String) -> Tokn;
}

struct ZeroScanner;

impl Scanner for ZeroScanner {
    fn should_scan(&self, state: State, c: u8) -> bool {
        state == State::Normal && c == b'0'
    }

    fn scan(&self, c: u8) -> ScanResult {
        match c {
            0 => ScanResult::Finish,
            _ if is_whitespace(c) => ScanResult::Finish,
            _ => ScanResult::Error,
        }
    }

    fn return_token(&self, token_string: String) -> Tokn {
        Tokn::Nmbr(token_string)
    }
}

enum TokenizeResult {
    Ok(Tokn),
    Err(Error),
    EOF,
}

impl<'a> Lexer<'a> {
    fn consume_char_new(&mut self) {
        if let Some(c) = self.iter.next() {
            self.current_char = c;
        } else {
            self.current_char = 0; // EOF
        }
    }

    fn get_scanner<'b, T: Scanner>(&self) -> &'b T {
        let scanner = ZeroScanner;
        &scanner
    }

    fn tokenize(&mut self) -> TokenizeResult {
        self.token_bytes.clear();

        loop {
            let s = self.state;

            self.consume_char_new();
            let c = self.current_char;

            let scanner = self.get_scanner();

            if scanner.should_scan(s, c) {
                let t = self.get_token_string();
                match scanner.scan(c) {
                    ScanResult::Continue => {},
                    ScanResult::Finish => {
                        return TokenizeResult::Ok(scanner.return_token(t));
                    },
                    ScanResult::Error => {
                        let error_info = format!("state: {:?} token_bytes: {:?} char: {:?}", s, t, c);
                        return TokenizeResult::Err(Error::Invalid(error_info));
                    },
                    ScanResult::EOF => {
                        return TokenizeResult::EOF;
                    },
                }
            }
        }
    }
}

#[test]
fn test() {
    let scanner = ZeroScanner;
}
