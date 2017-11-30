use ::token::Tokn;
use ::util::*;
use lexer::{Lexer, Error, State};
use lexer::new::*;

#[derive(Debug)]
struct ZeroScanner;

impl Scanner for ZeroScanner {
    fn scan(&self, state: State, c: u8) -> ScanResult {
        let mut res = ScanResult::Error;

        if state == State::Normal && c == b'0' {
            let opt = vec![
               ContinueOption::PushCharToToken,
               ContinueOption::ChangeState(State::ZeroNumber),
            ];
            res = ScanResult::Continue(opt);
        } else if state == State::ZeroNumber {
            res = match c {
                0 => ScanResult::Finish,
                _ if is_whitespace(c) => ScanResult::Finish ,
                _ => ScanResult::Error,
            };
        }
        res
    }

    fn return_token(&self, token_string: String) -> Tokn {
        Tokn::Nmbr(token_string)
    }

    fn s(&self) -> String {
        "ZeroScanner".to_string()
    }
}

#[test]
fn test() {
    lex_from_str("0", vec!["Nmbr<0>"]);
}

fn lex_from_str(text: &str, right: Vec<&str>) {
    let mut iter = text.bytes();
    let mut lexer = Lexer::new2(&mut iter, vec![&ZeroScanner]);
    let mut result = vec![];
    if let TokenizeResult::Ok(t) = lexer.tokenize() {
        let s = format!("{:?}", t);
        result.push(s);
    }
    assert_eq!(result, right);
}




impl<'a, 'b> Lexer<'a, 'b> {

    // 区切り文字ならここで数値を終わらせる必要がある
    // ただし、全ての区切り文字がここで判断されるわけではない
    // '[' | ']' | '(' | ')' | ':' | '|' => Some("finish"),
    pub fn scan_number(&mut self, c: u8) {
        println!("scan_number: {:?}", c);
        match c {
            // ゼロを判別する
            b'0' => self.scan_number_zero(c),
            // ゼロ以外の数値を判別する
            _ if is_digit(c) => self.scan_number_normal(c),
            // ここは通らないはず
            _ => { unimplemented!() },
        }
    }

    pub fn scan_number_normal(&mut self, c: u8) {
        self.token_bytes.push(c);
        self.consume_char();

        loop {
            println!("is_digit current_char: {:?} token_bytes: {:?}", self.current_char, self.token_bytes);
            if is_whitespace(self.current_char) || self.eof() {
                println!("{:?}", "eof finish");
                self.finish_number();
                break;
            } else if is_digit(self.current_char) {
                self.token_bytes.push(self.current_char);
                self.consume_char();
            /*} else if is_whitespace(self.current_char) || self.eof() {
                println!("{:?}", "is_whitespace");
                self.finish_number();
                break;*/
            } else {
                self.finish_error_number();
                break;
            }
        }
        println!("{:?}", "finish");
    }

    pub fn scan_number_zero(&mut self, c: u8) {
        println!("scan_number_zero: {:?}", c);
        self.token_bytes.push(c);
        self.consume_char();

        // if is_whitespace(self.current_char) {
        if is_whitespace(self.current_char) || self.eof() {
            println!("{:?}", "is_whitespace_zero");
            self.finish_number();
        } else {
            println!("{:?}", "is_whitespace_error");
            self.finish_error_number();
        }
    }

    fn finish_number(&mut self) {
        let s = self.get_token_string();
        println!("Valid number: {}", s);
        // self.finish_error(Error::Invalid(format!("Invalid number zero: {}", s)));
        self.finish(Ok(Tokn::Nmbr(s)), State::Normal);
    }

    fn finish_error_number(&mut self) {
        self.finish_error(Error::Invalid("Invalid number".to_string()));
    }
}
