macro_rules! push {
    ($s:ident, $t:expr) => {{
        $s.ast.borrow_mut().push_and_entry($t);
        Ok($s.ast.borrow().max_index.get())
    }}
}

mod error;

use std::cell::RefCell;
use core::*;
use lexer::*;
use self::error::Error;

pub struct Parser<'a, 'b> {
    lexer: Lexer<'a, 'a>,
    ast: &'b RefCell<AbstractSyntaxTree>,
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

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(lexer: Lexer<'a, 'a>, ast: &'b RefCell<AbstractSyntaxTree>) -> Self {
        Parser {
            lexer: lexer,
            ast: ast/*RefCell::new(AbstractSyntaxTree::new())*/,
            // state: State::Aexp,
            current_token: RefCell::new(/*None*/CurrentToken::SOT),
            // aexp_tokens: vec![vec![]],
        }
    }

    pub fn parse(&mut self) -> &'b RefCell<AbstractSyntaxTree> {
        self.consume_token();

        self.parse_aexp();

        self.ast
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

    fn parse_aexp(&mut self) -> Result<u32, Error> {
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
    fn parse_otag(&mut self, tokn: Tokn) -> Result<u32, Error> {
        let current_token = self.current_token.borrow().clone();
        match current_token {
            CurrentToken::Has(Tokn::Otag(ref otag)) => {
                self.consume_token();

                /* ?マクロを使いたい */
                let pidx = (self.parse_aexp())?;
                let qidx = (self.parse_aexp())?;
                match tokn {
                    Tokn::Pipe => push!(self, Epiq::Tpiq{o: otag.clone(), p: pidx, q: qidx}),
                    Tokn::Crrt => push!(self, Epiq::Mpiq{o: otag.clone(), p: pidx, q: qidx}),
                    _ => Err(Error::UnknownError(255)),
                }
            },

            CurrentToken::Has(ref t) => Err(Error::TokenError(t.clone())),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn parse_list(&mut self) -> Result<u32, Error> {
        let mut current_token = self.current_token.borrow().clone();
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

    fn parse_literal(&mut self) -> Result<u32, Error> {
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

    // 書き換えたがLexerと同様の構造にするのが難しそうだったので一旦保留
    /*
    #[derive(Eq, PartialEq, Clone, Debug)]
    enum State {
        Aexp,
        WaitOtag,
        Error(String),
    }
    */

    /*
    fn parse_flat() {
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
    }
    */
}
