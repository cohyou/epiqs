use std::cell::{RefCell, Ref};
use core::*;
use lexer::*;

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    ast: RefCell<AbstractSyntaxTree>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>) -> Self {
        Parser { lexer: lexer, ast: RefCell::new(AbstractSyntaxTree::new()) }
    }

    pub fn parse(&mut self) -> Ref<AbstractSyntaxTree> {
        self.parse_literal();
        self.ast.borrow()
    }

    fn parse_literal(&mut self) {
        match self.lexer.tokenize() {
            TokenizeResult::Ok(t) => {
                self.push_literal(t)
            },
            TokenizeResult::Err(e) => {
                println!("{}", e);
                // break;
            },
            TokenizeResult::EOF(t) => {
                self.push_literal(t)
                // break;
            }
            TokenizeResult::EmptyEOF => { /* break; */ },
        }
    }

    fn push_literal(&mut self, t: Tokn) {
        // println!("{:?}", t);
        match t {
            Tokn::Chvc(ref s) => {
                self.ast.borrow_mut().push(Epiq::Name(s.clone()));
            },
            Tokn::Nmbr(ref s) => {
                self.ast.borrow_mut().push(Epiq::Uit8(s.parse::<u64>().unwrap()));
            },
            _ => {
                // ast.push(Epiq::Tpiq{ o: "".to_string(), p: 0, q: 0 });
            },
        }
    }
}

#[test]
#[ignore]
fn test_parser() {
    let mut iter = "".bytes();
    let scanners: &Vec<&Scanner> = &vec![&EOFScanner];
    let lexer = Lexer::new(&mut iter, scanners);
    let mut parser = Parser::new(lexer);
    parser.parse();
}

/*
mod error;

use super::core::{Epiq, Heliqs};
use lexer::Lexer;
use super::token::Tokn;
use self::error::Error;
use super::printer::*;

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    tokens: Vec<Tokn>,
    p: usize, // index of current token
    // markers: Vec<usize>, // use as stack of returning points when backtracking
    vm: Heliqs,
}

impl<'a> Parser<'a> {

    pub fn new(mut l: Lexer<'a, 'a>) -> Parser {
        let ts = match l.next_token() {
            Ok(t) => vec![t],
            _ => vec![]
        };
        Parser { lexer: l, tokens: ts, vm:Heliqs { vctr: vec![] }, p: 0, /* markers: vec![],*/ }
    }

    pub fn parse(&mut self) -> Result<String, Error> {
        match self.parse_aexp() {
            Ok(i) => {
                let printer = Printer{vm: &self.vm};
                // 最初の空白を取り除いてから返す
                // TODO: めっちゃダサいけど2つの空白を1つにしてるのはあとで直す
                Ok(printer.print_aexp(i).to_string().trim().to_string().replace("  ", " "))
            },

            Err(e) => {
                Err(e)
            },
        }
    }


    /* PARSING */
    fn parse_aexp(&mut self) -> Result<usize, Error> {
        match self.get_target_token() {
            Some(Tokn::Pipe) => {
                self.consume_token();
                self.parse_otag()
            },
            _ => self.parse_name(),
        }
    }

    // Pipe QTag Pval QVal
    fn parse_otag(&mut self) -> Result<usize, Error> {
        match self.get_target_token() {
            Some(Tokn::Otag(otag)) => {
                self.consume_token();
                match self.parse_aexp() {
                    Ok(pidx) => {
                        match self.parse_aexp() {
                            Ok(qidx) => {
                                self.vm.vctr.push(Epiq::Tpiq{o: otag, p: pidx, q: qidx});
                                Ok(self.vm.vctr.len() - 1)
                            },
                            _ => Err(Error::UnknownError(3)),
                        }
                    },
                    _ => Err(Error::UnknownError(2)),
                }
            },
            Some(t) => Err(Error::TokenError(t)),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn parse_name(&mut self) -> Result<usize, Error> {
        match self.get_target_token() {
            Some(Tokn::Chvc(s)) => {
                self.consume_token();
                self.vm.vctr.push(Epiq::Name(s));
                Ok(self.vm.vctr.len() - 1)
            },
            _ => Err(Error::UnknownError(10)),
        }
    }

    // VM内でのconsの最大値を返す
    // 実際には「最後に作られたconsの番号」を返す
    // 将来変わるかもしれないが、今は0から順番にindexをつけているので、最後に作られたconsのindexが一番大きい
    /*
    pub fn max_index(&self) -> usize {
        self.vm.vctr.len() - 1
    }
    */

    fn get_target_token(&self) -> Option<Tokn> {
        match self.tokens.get(self.p) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    /// garantee existence of i tokens in self.tokens
    fn sync_tokens(&mut self, i: usize) {
        // println!("sync_tokens self.tokens: {:?} self.p: {:?}", self.tokens, self.p);
        if self.p + i > self.tokens.len() {
            let n = (self.p + i) - self.tokens.len();
            self.consume_tokens(n);
        }
    }

    /// consume multiple (count of n) tokens
    fn consume_tokens(&mut self, n: usize) {
        for _ in 0..n {
            match self.lexer.next_token() {
                Ok(t) => { self.tokens.push(t); },
                Err(_) => {},
            }

        }
    }

    fn consume_token(&mut self) {
        self.p += 1;
        if self.p == self.tokens.len() /* && !self.is_speculating()*/ {
            self.p = 0;
            self.tokens.clear();
        }
        self.sync_tokens(1);
    }
}
*/
