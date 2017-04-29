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

use lexer::{Lexer, LexerError};
use parser::Parser;

fn exec() -> Result<Vec<String>, Box<Error>> {
    let f = File::open("_.iq")?;
    let reader = BufReader::new(f);
    let mut iter = reader.bytes().map(|b| b.unwrap());
    let mut lexer = Lexer::new(&mut iter);
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

    /*
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {
            let tokens = match lexer::Lexer::lex(&s) {
                Err(why) => panic!("LEX ERROR: {}", why.0),
                Ok(tokens) => tokens,
            };

            println!("{:?}", tokens);

            match parser::Parser::parse(&tokens) {
                Ok(res) => println!("{:?}", res),
                Err(e) => panic!("PARSE ERROR: {}", e.0),
            }
        },
    }
    */
}
