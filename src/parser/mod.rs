mod error;

use std::cell::{RefCell, Ref};
use core::*;
use lexer::*;
use self::error::Error;

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    ast: RefCell<AbstractSyntaxTree>,
    state: State,
    current_token: RefCell</*Option<Tokn>*/CurrentToken>,
    aexp_tokens: Vec<Vec<Tokn>>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum State {
    Aexp,
    WaitOtag,
    Error(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum CurrentToken {
    SOT, // Start Of Token
    Has(Tokn),
    EOT, // End Of Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>) -> Self {
        Parser {
            lexer: lexer,
            ast: RefCell::new(AbstractSyntaxTree::new()),
            state: State::Aexp,
            current_token: RefCell::new(/*None*/CurrentToken::SOT),
            aexp_tokens: vec![vec![]],
        }
    }

    pub fn parse(&mut self) -> Ref<AbstractSyntaxTree> {
        /*
        loop {
            let s = self.state.clone();
            let t = self.current_token.borrow().clone();

            println!("parser state: {:?} token: {:?}", s, t);
            match s {
                State::Aexp => {
                    match t {
                        CurrentToken::SOT => {
                            // 初期値状態 最初のトークンを取得する
                            self.consume_token();
                        },

                        CurrentToken::Has(Tokn::Pipe) => {
                            self.state = State::WaitOtag;

                            self.consume_token();

                            let len = self.aexp_tokens.len();
                            self.aexp_tokens[len - 1].push(Tokn::Pipe);
                        },

                        CurrentToken::Has(Tokn::Chvc(ref s)) => {
                            self.consume_token();

                            let name = Epiq::Name(s.clone());
                            self.ast.borrow_mut().push(name);

                            let len = self.aexp_tokens.len();
                            self.aexp_tokens[len - 1].push(Tokn::Chvc(s.to_string()));
                        },

                        CurrentToken::Has(Tokn::Nmbr(ref s)) => {
                            self.consume_token();
                            self.ast.borrow_mut().push(Epiq::Uit8(s.parse::<u64>().unwrap()));

                            let len = self.aexp_tokens.len();
                            self.aexp_tokens[len - 1].push(Tokn::Nmbr(ref s));
                        },

                        CurrentToken::EOT => {
                            break;
                        },
                        _ => {},
                    }
                },

                State::WaitOtag => {
                    match t {
                        CurrentToken::Has(Tokn::Otag(tag_name)) => {
                            match tag_name.as_str() {
                                ":" => {
                                    self.state = State::Aexp;
                                    self.consume_token();
                                    let len = self.aexp_tokens.len();
                                    self.aexp_tokens[len - 1].push(Tokn::Otag(tag_name));
                                    self.aexp_tokens.push(vec![]);
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                },

                State::Error(_e) => {
                    break;
                },
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
                *token = CurrentToken::Has(t);
            },
            TokenizeResult::Err(e) => {
                self.state = State::Error(format!("{}", e));
            }
            TokenizeResult::EOF => {
                let mut token = self.current_token.borrow_mut();
                *token = CurrentToken::EOT;
            },
        }
    }

    fn parse_aexp(&mut self) -> Result<u32, Error> {
        let res = self.current_token.borrow().clone();

        match res {
            CurrentToken::Has(Tokn::Pipe) => {
                self.consume_token();
                self.parse_otag()
            },
            _ => self.parse_literal(),
        }
    }

    // Pipe QTag Pval QVal
    fn parse_otag(&mut self) -> Result<u32, Error> {
        let current_token = self.current_token.borrow().clone();
        match current_token {
            CurrentToken::Has(Tokn::Otag(ref otag)) => {
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
            CurrentToken::Has(ref t) => Err(Error::TokenError(t.clone())),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn parse_literal(&mut self) -> Result<u32, Error> {
        let current_token = self.current_token.borrow().clone();
        match current_token {
            CurrentToken::Has(Tokn::Chvc(ref s)) => {
                self.consume_token();
                self.ast.borrow_mut().push(Epiq::Name(s.clone()));
                Ok(self.ast.borrow().max_index.get())
            },
            CurrentToken::Has(Tokn::Nmbr(ref s)) => {
                self.consume_token();
                self.ast.borrow_mut().push(Epiq::Uit8(s.parse::<u64>().unwrap()));
                Ok(self.ast.borrow().max_index.get())
            },
            _ => Err(Error::UnknownError(10)),
        }
    }
}
