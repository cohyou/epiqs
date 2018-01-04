macro_rules! push {
    ($s:ident, $t:expr) => {{
        // println!("parser push: {:?}", $t);
        Ok($s.vm.borrow_mut().alloc($t))
    }}
}

mod error;

use std::rc::Rc;
use std::cell::RefCell;
use std::usize::MAX;
use core::*;
use lexer::*;
use self::error::Error;

const UNIT_INDX: usize = 5;
const K: usize = 3;

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    vm: Rc<RefCell<Heliqs>>,
    // state: State,
    current_token: RefCell<CurrentToken>,
    // aexp_tokens: Vec<Vec<Tokn>>,

    lookahead: [CurrentToken; K],
    p: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum CurrentToken {
    SOT, // Start Of Tokens
    Has(Tokn),
    EOT, // End Of Tokens
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>, vm: Rc<RefCell<Heliqs>>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            vm: vm,
            // state: State::Aexp,
            current_token: RefCell::new(CurrentToken::SOT),
            // aexp_tokens: vec![vec![]],
            lookahead: [CurrentToken::SOT, CurrentToken::SOT, CurrentToken::SOT],
            p: K - 1, // 0にはCurrentToken::SOTを入れることが固定で決まっているので、consumeを開始するのはK - 1から
        };

        parser.consume_token();
        parser.consume_token();

        parser
    }

    // Unitは常に1つにする(index固定)
    fn add_unit(&mut self) {
        let _unit = self.vm.borrow_mut().alloc(Epiq::Unit);
    }

    fn add_prim(&mut self, name: &str) {
        let prim = self.vm.borrow_mut().alloc(Epiq::Prim(name.to_string()));
        self.vm.borrow_mut().define(name, prim);
    }

    pub fn parse(&mut self) {

        self.add_prim("decr");
        self.add_prim("ltoreq");
        self.add_prim("eq");
        self.add_prim("plus");
        self.add_prim("minus");

        self.add_unit();

        self.consume_token();

        let _ = self.parse_aexp();
    }

    fn set_current_token(&mut self, t: CurrentToken) {
        self.lookahead[self.p] = t;
        self.p = (self.p + 1) % K;
    }

    fn get_token(&self, i: usize) -> CurrentToken {
        self.lookahead[(self.p + i) % K].clone()
    }

    fn get_current_token(&self) -> CurrentToken {
        self.get_token(0)
    }

    fn consume_token(&mut self) {
        let res = self.lexer.tokenize();
        match res {
            TokenizeResult::Ok(t) => self.set_current_token(CurrentToken::Has(t)),
            TokenizeResult::Err(_e) => {},
            TokenizeResult::EOF => self.set_current_token(CurrentToken::EOT),
        }
    }

    fn parse_aexp(&mut self) -> Result<usize, Error> {
        let res = self.get_current_token();

        match res {
            // 中置記法の判断をする

            // @と!だと@の方が優先度が高い
            CurrentToken::Has(Tokn::Atsm) => {
                if self.get_token(2) == CurrentToken::Has(Tokn::Bang) {
                    println!("{:?}", "@ symbol ! という典型的な場合");
                    // @ symbol ! という典型的な場合
                    self.consume_token(); // consume Atsm
                    let left = (self.parse_resolve())?;
                    self.consume_token(); // consume Bang
                    let qidx = (self.parse_aexp())?;
                    let id = self.vm.borrow_mut().alloc(Epiq::Appl(left, qidx));
                    push!(self, Epiq::Eval(UNIT_INDX, id))
                } else {
                    println!("{:?}", "@の次がliteral以外の場合、まだ考慮していない");
                    // @の次がliteral以外の場合、まだ考慮していない
                    self.consume_token(); // consume Atsm
                    self.parse_resolve()
                }
            },

            CurrentToken::Has(Tokn::Chvc(_)) |
            CurrentToken::Has(Tokn::Nmbr(_))
            if self.get_token(1) == CurrentToken::Has(Tokn::Bang) => {
                self.parse_apply()
            },

            // LL(1)の範囲内
            CurrentToken::Has(Tokn::Sgqt) => {
                self.consume_token();
                self.parse_otag(Tokn::Sgqt)
            },


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
                Ok(UNIT_INDX)
            }
            CurrentToken::Has(Tokn::Lbkt) => {
                self.consume_token();
                self.parse_list()
            },
            CurrentToken::Has(Tokn::Dbqt) => {
                self.consume_token();
                self.parse_text()
            },

            _ => {
                self.parse_literal()
            },
        }
    }

    fn parse_resolve(&mut self) -> Result<usize, Error> {
        // let pidx = (self.parse_aexp())?;
        let qidx = (self.parse_literal())?;
        let id = self.vm.borrow_mut().alloc(Epiq::Rslv(UNIT_INDX, qidx));
        push!(self, Epiq::Eval(UNIT_INDX, id))
    }

    // Pipe QTag Pval QVal
    fn parse_otag(&mut self, tokn: Tokn) -> Result<usize, Error> {
        let current_token = self.get_current_token();
        match current_token {
            CurrentToken::Has(Tokn::Otag(ref otag)) => {
                self.consume_token();

                match tokn {
                    Tokn::Pipe => {
                        let pidx = (self.parse_aexp())?;
                        let qidx = (self.parse_aexp())?;
                        self.match_otag(pidx, qidx, otag)
                    },
                    Tokn::Crrt => {
                        match otag.as_ref() {
                            // ^Tと^Fは特別扱い
                            "T" => push!(self, Epiq::Tval),
                            "F" => push!(self, Epiq::Fval),
                            _ =>  {
                                // 現在は^Tか^Fしか認めていないが、将来のため
                                let pidx = (self.parse_aexp())?;
                                let qidx = (self.parse_aexp())?;
                                push!(self, Epiq::Mpiq{o: otag.clone(), p: pidx, q: qidx})
                            },
                        }
                    },
                    Tokn::Sgqt => {
                        // 引数は一つ、それをqとみなす
                        let qidx = (self.parse_aexp())?;
                        self.match_otag(UNIT_INDX, qidx, otag)
                    },
                    _ => Err(Error::UnknownError(255)),
                }
            },

            CurrentToken::Has(ref t) => Err(Error::TokenError(t.clone())),
            _ => Err(Error::UnknownError(1)),
        }
    }

    fn match_otag(&mut self, pidx: NodeId, qidx: NodeId, otag: &str) -> Result<usize, Error> {
        match otag {
            ">" => push!(self, Epiq::Eval(pidx, qidx)),
            ":" => push!(self, Epiq::Lpiq(pidx, qidx)),
            "!" => push!(self, Epiq::Appl(pidx, qidx)),
            "@" => push!(self, Epiq::Rslv(pidx, qidx)),
            "?" => push!(self, Epiq::Cond(pidx, qidx)),
            "%" => push!(self, Epiq::Envn(pidx, qidx)),
            "#" => push!(self, Epiq::Bind(pidx, qidx)),
            "." => push!(self, Epiq::Accs(pidx, qidx)),
            r"\" => push!(self, Epiq::Lmbd(pidx, qidx)),

            _ => {
                push!(self, Epiq::Tpiq{o: otag.to_string(), p: pidx, q: qidx})
            },
        }
    }

    fn parse_list(&mut self) -> Result<usize, Error> {
        let current_token = self.get_current_token();
        // 閉じbracketが出るまで再帰呼出
        match current_token {
            CurrentToken::Has(Tokn::Rbkt) => {
                self.consume_token();
                push!(self, Epiq::Unit)
            },
            _ => {
                let pidx = (self.parse_aexp())?;
                let qidx = (self.parse_list())?;
                push!(self, Epiq::Lpiq(pidx, qidx))
            }
        }
    }

    fn parse_text(&mut self) -> Result<usize, Error> {
        let current_token1 = self.get_current_token();
        match current_token1 {
            CurrentToken::Has(Tokn::Chvc(ref s)) => {
                self.consume_token();
                let current_token2 = self.get_current_token();
                match current_token2 {
                    CurrentToken::Has(Tokn::Dbqt) => {
                        self.consume_token();
                        push!(self, Epiq::Text(s.clone()))
                    },
                    _ => Err(Error::UnknownError(13)),
                }
            },
            _ => Err(Error::UnknownError(12)),
        }
    }

    fn parse_literal(&mut self) -> Result<usize, Error> {
        let current_token = self.get_current_token();
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

    fn parse_apply(&mut self) -> Result<usize, Error> {
        let left = (self.parse_literal())?;
        if self.get_current_token() == CurrentToken::Has(Tokn::Bang) {
            self.consume_token();
            println!("its bang in apply1: {:?}", left);
            let qidx = (self.parse_aexp())?;
            println!("its bang in apply2: {:?}", self.vm.borrow().get_epiq(qidx));
            let id = self.vm.borrow_mut().alloc(Epiq::Appl(left, qidx));
            push!(self, Epiq::Eval(UNIT_INDX, id))
        } else {
            Err(Error::UnknownError(20)) // 普通は通らない
        }
    }
}
