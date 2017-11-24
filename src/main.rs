// #![feature(slice_patterns, advanced_slice_patterns)]
// #![feature(box_syntax, box_patterns)]

mod util;
mod core;
mod token;
mod lexer_error;
mod lexer_state;
mod lexer_basic;
mod scanner;
mod lexer;
mod parser_error;
mod parser;
mod printer;

mod nmbr;

// use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
// use std::path::Path;
// use std::io;
use std::io::Read;
// use std::cell::RefCell;

use lexer_basic::Lexer;
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
