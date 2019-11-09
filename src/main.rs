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

// use epiqs::lexer::*;
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
    print_str("abc", "abc");

    if args.len() >= 2 {
        let file_name = &args[1];
        // let file = File::open(file_name).unwrap();
        // let reader = BufReader::new(file);
        println!("{}", evaled_reader("sample/macro.iq"));
        // let mut s = String::new();
        // file.read_to_string(&mut s).unwrap();
        // println!("{}", evaled_str(&s));
    } else {
        println!("{}", "sorry, REPL is now developing, please wait a bit...");
    }
    */
}
