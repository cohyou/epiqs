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

#[cfg(test)]
mod tests {
    use super::*;

    fn is_alphabetic(c: char) -> bool {
        (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
    }

    fn lexer_from_str(text: &str, right: &str) {
        let mut iter = text.bytes();
        let mut lexer = Lexer::new(&mut iter);
        let mut result = String::from("");
        while let Ok(t) = lexer.next_token() {
            let s = &format!("{:?} ", t);
            result.push_str(s);
            // println!("{:?}", s);
        }
        let len = result.len() - 1;
        result.remove(len);
        // result.trim_right();
        assert_eq!(result, right);
    }

    #[test]
    fn test_alphabeticality() {
        assert!(is_alphabetic('a'));
        assert!(is_alphabetic('j'));
        assert!(is_alphabetic('z'));
        assert!(is_alphabetic('A'));
        assert!(is_alphabetic('L'));
        assert!(is_alphabetic('Z'));
        assert!(!is_alphabetic('Ａ'));
        assert!(!is_alphabetic('0'));
        assert!(!is_alphabetic('%'));
    }

    #[test]
    fn lex_true() { lexer_from_str("T", "Name<T>"); }
    #[test]
    fn lex_false() { lexer_from_str("F", "Name<F>"); }
    #[test]
    fn lex_none() { lexer_from_str("N", "Name<N>"); }

    #[test]
    fn lex_symbol() {
        lexer_from_str("symbol", "Name<symbol>");
        lexer_from_str("SYMBOL", "Name<SYMBOL>");
        lexer_from_str("SYMBOL12345", "Name<SYMBOL12345>");
    }

    #[test]
    fn lex_double_quote() {
        // lexer_from_str("\"double quote\"", "Dbqt Text<double quote> Dbqt");
        lexer_from_str("\"我輩は猫である。名前はまだない。\"", "Dbqt Text<我輩は猫である。名前はまだない。> Dbqt");
    }

    #[test]
    fn lex_multiple_tokens() {
        lexer_from_str("T N F", "Name<T> Name<N> Name<F>");
    }

    #[test]
    fn lex_parens() {
        lexer_from_str("()", "Lprn Rprn");
        lexer_from_str("    (  ) ", "Lprn Rprn");
        lexer_from_str("(a)", "Lprn Name<a> Rprn");
        lexer_from_str("(a 0 \"c\")", "Lprn Name<a> Nmbr<0> Dbqt Text<c> Dbqt Rprn");
    }

    #[test]
    fn lex_brackets() {
        lexer_from_str("[]", "Lbkt Rbkt");
        lexer_from_str(" [   ]     ", "Lbkt Rbkt");
        lexer_from_str("[53420417]", "Lbkt Nmbr<53420417> Rbkt");
        lexer_from_str("[\"i t ' s\" bang \"m i r a c l e\" done 9999]",
                       "Lbkt Dbqt Text<i t ' s> Dbqt Name<bang> Dbqt Text<m i r a c l e> Dbqt Name<done> Nmbr<9999> Rbkt");
    }

    #[test]
    fn lex_curly_braces() {
        lexer_from_str("{}", "Lcrl Rcrl");
        lexer_from_str("  { }   ", "Lcrl Rcrl");
        lexer_from_str("{\"波かっこの中です\"}", "Lcrl Dbqt Text<波かっこの中です> Dbqt Rcrl");
        lexer_from_str("{name \"baad\" age 288 favorite machine}",
                       "Lcrl Name<name> Dbqt Text<baad> Dbqt Name<age> Nmbr<288> Name<favorite> Name<machine> Rcrl");
    }

    #[test]
    fn lex_nested_sequences() {
        lexer_from_str("(())", "Lprn Lprn Rprn Rprn");
        lexer_from_str("[[]]", "Lbkt Lbkt Rbkt Rbkt");
        lexer_from_str("{{}}", "Lcrl Lcrl Rcrl Rcrl");

        lexer_from_str("((a))", "Lprn Lprn Name<a> Rprn Rprn");
        lexer_from_str("(a ())", "Lprn Name<a> Lprn Rprn Rprn");
        lexer_from_str("(() a)", "Lprn Lprn Rprn Name<a> Rprn");

        lexer_from_str("[ [   a   b ]]", "Lbkt Lbkt Name<a> Name<b> Rbkt Rbkt");
        lexer_from_str("[a   b[ ]]", "Lbkt Name<a> Name<b> Lbkt Rbkt Rbkt");
        lexer_from_str(" [ [ ]a b]   ", "Lbkt Lbkt Rbkt Name<a> Name<b> Rbkt");

        lexer_from_str("{a{b}}", "Lcrl Name<a> Lcrl Name<b> Rcrl Rcrl");
        lexer_from_str("{{a}b}", "Lcrl Lcrl Name<a> Rcrl Name<b> Rcrl");
        lexer_from_str("{a{}b}", "Lcrl Name<a> Lcrl Rcrl Name<b> Rcrl");

        // Plus(+)とStar(*)が不明瞭なままテストの中に書いてしまったが、ひとまず問題が洗い出せただけでも素晴らしい
        lexer_from_str("{SUM (+ 1 6 (* s 7)) MEMBER [\"Tom\" \"John\" {first: \"Kazuo\" family: \"Ishiguro\"}]}",
                       "Lcrl Name<SUM> Lprn Plus Nmbr<1> Nmbr<6> Lprn Star Name<s> Nmbr<7> Rprn Rprn Name<MEMBER> Lbkt Dbqt Text<Tom> Dbqt Dbqt Text<John> Dbqt Lcrl Name<first> Coln Dbqt Text<Kazuo> Dbqt Name<family> Coln Dbqt Text<Ishiguro> Dbqt Rcrl Rbkt Rcrl");
    }

    fn parser_from_str(text: &str) -> (Result<(), parser::ParseError>, String) {
        let mut iter = text.bytes();
        let lexer = Lexer::new(&mut iter);
        let mut parser = Parser::new(lexer);
        let ret = parser.parse();
        let s = parser.print_aexp(parser.max_index());
        (ret, s)
    }

    #[test]
    fn parse_zero() {
        let s = "0";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Int8(0)");
    }

    #[test]
    fn parse_number() {
        let s = "12345";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Int8(12345)");
        println!("{:?}", r.1);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_big_number() {
        // 今は文字列から数値に変換する際に桁あふれを起こしているが、その対応はいずれ
        let s = "123459578462357356342";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    fn parse_empty_text() {
        let s = "\"\"";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Text(\"\")");
    }

    #[test]
    fn parse_text() {
        let s = "\"string\"";
        let r = parser_from_str(s);
        assert_eq!(r.1, "Text(\"string\")");
    }

    #[test]
    #[should_panic]
    fn parse_true() {
        // まだ真偽値リテラルは実装されていない
        let s = "T";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_false() {
        // まだ真偽値リテラルは実装されていない
        let s = "F";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }

    #[test]
    #[should_panic]
    fn parse_none() {
        // まだNoneリテラルは実装されていない(代わりにUnitがある)
        let s = "N";
        let r = parser_from_str(s);
        assert!(r.0.is_ok());
    }
}
