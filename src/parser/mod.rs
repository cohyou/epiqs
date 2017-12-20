macro_rules! push {
    ($s:ident, $t:expr) => {{
        // println!("parser push: {:?}", $t);
        Ok($s.vm.borrow_mut().alloc($t))
    }}
}

mod error;

use std::rc::Rc;
use std::cell::RefCell;
use core::*;
use lexer::*;
use self::error::Error;

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    // ast: &'a NodeArena<Epiq>,
    vm: Rc<RefCell<Heliqs>>,
    // state: State,
    current_token: RefCell</*Option<Tokn>*/CurrentToken>,
    // aexp_tokens: Vec<Vec<Tokn>>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum CurrentToken {
    SOT, // Start Of Tokens
    Has(Tokn),
    EOT, // End Of Tokens
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>, vm: Rc<RefCell<Heliqs>>) -> Self {
        Parser {
            lexer: lexer,
            // ast: ast,
            vm: vm,
            // state: State::Aexp,
            current_token: RefCell::new(/*None*/CurrentToken::SOT),
            // aexp_tokens: vec![vec![]],
        }
    }

    pub fn parse(&mut self) {
        // Prim追加
        let prim1 = self.vm.borrow_mut().alloc(Epiq::Prim("decr".to_string())); // 0
        self.vm.borrow_mut().define("decr", prim1);
        let prim2 = self.vm.borrow_mut().alloc(Epiq::Prim("ltoreq".to_string())); // 1
        self.vm.borrow_mut().define("ltoreq", prim2);


        self.consume_token();

        self.parse_aexp();
    }

    fn consume_token(&mut self) {
        let res = self.lexer.tokenize();
        match res {
            TokenizeResult::Ok(t) => {
                let mut token = self.current_token.borrow_mut();
                *token = CurrentToken::Has(t);
            },
            TokenizeResult::Err(_e) => {
                // self.state = State::Error(format!("{}", e));
            }
            TokenizeResult::EOF => {
                let mut token = self.current_token.borrow_mut();
                *token = CurrentToken::EOT;
            },
        }
    }

    fn parse_aexp(&mut self) -> Result<usize, Error> {
        let res = self.current_token.borrow().clone();

        match res {
            CurrentToken::Has(Tokn::Pipe) => {
                self.consume_token();
                self.parse_otag(Tokn::Pipe)
            },
            CurrentToken::Has(Tokn::Crrt) => {
                self.consume_token();
                self.parse_otag(Tokn::Crrt)
            },
            CurrentToken::Has(Tokn::Smcl) => {
                self.consume_token();
                push!(self, Epiq::Unit)
            }
            CurrentToken::Has(Tokn::Lbkt) => {
                self.consume_token();
                self.parse_list()
            }
            _ => self.parse_literal(),
        }
    }

    // Pipe QTag Pval QVal
    fn parse_otag(&mut self, tokn: Tokn) -> Result<usize, Error> {
        let current_token = self.current_token.borrow().clone();
        match current_token {
            CurrentToken::Has(Tokn::Otag(ref otag)) => {
                self.consume_token();

                // ^Tと^Fは特別扱い
                match otag.as_ref() {
                    "T" => push!(self, Epiq::Tval),
                    "F" => push!(self, Epiq::Fval),
                    _ => {
                        let pidx = (self.parse_aexp())?;
                        let qidx = (self.parse_aexp())?;
                        match tokn {
                            Tokn::Pipe => push!(self, Epiq::Tpiq{o: otag.clone(), p: pidx, q: qidx}),
                            Tokn::Crrt => push!(self, Epiq::Mpiq{o: otag.clone(), p: pidx, q: qidx}),
                            _ => Err(Error::UnknownError(255)),
                        }
                    }
                }
            },

            CurrentToken::Has(ref t) => Err(Error::TokenError(t.clone())),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn parse_list(&mut self) -> Result<usize, Error> {
        let current_token = self.current_token.borrow().clone();
        // 閉じbracketが出るまで再帰呼出
        match current_token {
            CurrentToken::Has(Tokn::Rbkt) => {
                self.consume_token();
                push!(self, Epiq::Unit)
            },
            _ => {
                let pidx = (self.parse_aexp())?;
                let qidx = (self.parse_list())?;
                push!(self, Epiq::Tpiq{o: ":".to_string(), p: pidx, q: qidx})
            }
        }
    }

    fn parse_literal(&mut self) -> Result<usize, Error> {
        let current_token = self.current_token.borrow().clone();
        match current_token {
            CurrentToken::Has(Tokn::Chvc(ref s)) => {
                self.consume_token();
                push!(self, Epiq::Name(s.clone()))
            },
            CurrentToken::Has(Tokn::Nmbr(ref s)) => {
                self.consume_token();
                push!(self, Epiq::Uit8(s.parse::<i64>().unwrap()))
            },
            _ => Err(Error::UnknownError(10)),
        }
    }
}
