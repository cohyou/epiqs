#![feature(slice_patterns, advanced_slice_patterns)]
#![feature(box_syntax, box_patterns)]

mod core;
mod lexer;
mod parser;

// use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
// use std::path::Path;
// use std::io;
use std::io::Read;
// use std::cell::RefCell;

use lexer::Lexer;
use parser::Parser;

fn exec() -> Result<Vec<String>, Box<Error>> {
    let f = File::open("_.iq")?;
    let reader = BufReader::new(f);
    let mut iter = reader.bytes().map(|b| b.unwrap());
    let lexer = Lexer::new(&mut iter);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("{:?}", e),
    }

    Ok(vec![])
}

fn main() {
    match exec() {
        _ => println!("{:?}", "finished"),
    }
}

fn parser_from_str(text: &str) -> (Result<(), parser::ParseError>, String) {
    let mut iter = text.bytes();
    let lexer = Lexer::new(&mut iter);
    let mut parser = Parser::new(lexer);
    let ret = parser.parse();
    let s = parser.print_aexp(0);
    (ret, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_zero() {
        let s = "0";
        let r = parser_from_str(s);
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    fn parse_number() {
        let s = "12345";
        let r = parser_from_str(s);
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_big_number() {
        // 今は文字列から数値に変換する際に桁あふれを起こしているが、その対応はいずれ
        let s = "123459578462357356342";
        let r = parser_from_str(s);
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    fn parse_empty_text() {
        let s = "\"\"";
        let r = parser_from_str(s);
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    fn parse_text() {
        let s = "\"string\"";
        let r = parser_from_str(s);
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }
}
