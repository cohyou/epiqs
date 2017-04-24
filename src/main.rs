#![feature(slice_patterns, advanced_slice_patterns)]
#![feature(box_syntax, box_patterns)]

mod core;
mod lexer;
mod parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("_.iq");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
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


}
