// #![feature(slice_patterns, advanced_slice_patterns)]
// #![feature(box_syntax, box_patterns)]

extern crate epiqs;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::fs::File;
// use std::error::Error;
// use std::path::Path;
// use std::io;
// use std::io::Read;
// use std::cell::RefCell;

use epiqs::lexer::*;
// use epiqs::parser::*;
use epiqs::printer::*;

/*
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
}*/

fn main() {
    let scanners: &mut Vec<&Scanner> = &mut vec![
        &DelimiterScanner,
        &AlphanumericScanner,
        &ZeroScanner,
        &IntegerScanner,
        &EOFScanner,
    ];

    print_str("abc", "abc", scanners);

    /*
    match exec() {
        _ => println!("{:?}", "finished"),
    }
    */
}