mod error;

use std::cell::{RefCell, Ref};
use core::*;
use lexer::*;

use self::error::Error;


pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    ast: RefCell<AbstractSyntaxTree>,
    state: State,
    current_token: RefCell<Option<Tokn>>,
    aexp_tokens: Vec<Vec<Tokn>>,
}

enum State {
    Aexp,
    WaitOtag,
    Error(String),
    EOF,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>) -> Self {
        Parser {
            lexer: lexer,
            ast: RefCell::new(AbstractSyntaxTree::new()),
            state: State::Aexp,
            current_token: RefCell::new(None),
            aexp_tokens: vec![vec![]],
        }
    }

    pub fn parse(&mut self) -> Ref<AbstractSyntaxTree> {
        /*
        loop {
            let s = self.state;
            let t = self.current_token;

            match s {
                State::Aexp => {
                    match t {
                        Ok(Tokn::Pipe) => {
                            self.state = State::WaitOtag;
                            self.consume_token();
                            let len = self.aexp_tokens.len();
                            self.aexp_tokens[len - 1].push(t);
                        },
                        Ok(Tokn::Chvc(ref s)) => {

                        },
                        Ok(Tokn::Nmbr(ref s)) => {

                        },
                    }
                },

                State::WaitOtag => {
                    match t {
                        Ok(Tokn::Otag(":")) => {
                            self.state = State::Aexp;
                            self.consume_token();
                            let len = self.aexp_tokens.len();
                            self.aexp_tokens[len - 1].push(t);
                            self.aexp_tokens.push(vec![]);
                        },
                    }
                },

                State::Error(e) => {

                }
            }
        }
        */
        self.consume_token();
        self.parse_aexp();
        self.ast.borrow()
    }

    fn consume_token(&mut self) {
        let res = self.lexer.tokenize();
        match res {
            TokenizeResult::Ok(t) => {
                let mut token = self.current_token.borrow_mut();
                *token = Some(t);
            },
            TokenizeResult::Err(e) => {
                self.state = State::Error(format!("{}", e));
            }
            TokenizeResult::EOF(t) => {
                let mut token = self.current_token.borrow_mut();
                *token = Some(t);
                self.state = State::EOF;
            },
            TokenizeResult::EmptyEOF => {
                self.state = State::EOF;
            },
        }
    }

    fn parse_aexp(&mut self) -> Result<u32, Error> {
        let res = self.current_token.borrow();
        match *res {
            Some(Tokn::Pipe) => {
                self.consume_token();
                self.parse_otag()
            },
            _ => self.parse_literal(),
        }
    }

    // Pipe QTag Pval QVal
    fn parse_otag(&mut self) -> Result<u32, Error> {
        match *self.current_token.borrow() {
            Some(Tokn::Otag(ref otag)) => {
                self.consume_token();
                match self.parse_aexp() {
                    Ok(pidx) => {
                        match self.parse_aexp() {
                            Ok(qidx) => {
                                self.ast.borrow_mut().push(Epiq::Tpiq{o: otag.clone(), p: pidx, q: qidx});
                                Ok(self.ast.borrow().max_index.get())
                            },
                            _ => Err(Error::UnknownError(3)),
                        }
                    },
                    _ => Err(Error::UnknownError(2)),
                }
            },
            Some(ref t) => Err(Error::TokenError(t.clone())),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn parse_literal(&mut self) -> Result<u32, Error> {
        match *self.current_token.borrow() {
            Some(Tokn::Chvc(ref s)) => {
                self.consume_token();
                self.ast.borrow_mut().push(Epiq::Name(s.clone()));
                Ok(self.ast.borrow().max_index.get())
            },
            Some(Tokn::Nmbr(ref s)) => {
                self.consume_token();
                self.ast.borrow_mut().push(Epiq::Uit8(s.parse::<u64>().unwrap()));
                Ok(self.ast.borrow().max_index.get())
            },
            _ => Err(Error::UnknownError(10)),
        }
    }
}
