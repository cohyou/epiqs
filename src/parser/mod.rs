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
use self::TokenState::*;


const UNIT_INDX: usize = 5;
const K: usize = 3;

#[derive(Eq, PartialEq, Clone, Debug)]
enum TokenState {
    SOT, // Start Of Tokens
    Has(Tokn),
    EOT, // End Of Tokens
}

impl Default for TokenState {
    fn default() -> Self { SOT }
}

pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    vm: Rc<RefCell<Heliqs>>,
    // state: State,
    current_token: RefCell<TokenState>,
    // aexp_tokens: Vec<Vec<Tokn>>,

    lookahead: [TokenState; K],
    p: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, 'a>, vm: Rc<RefCell<Heliqs>>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            vm: vm,
            // state: State::Aexp,
            current_token: RefCell::new(SOT),
            // aexp_tokens: vec![vec![]],
            lookahead: Default::default(),
            p: 0,
        };

        for _ in 0..K {
            parser.consume_token();
        }

        parser
    }

    pub fn parse(&mut self) {

        self.add_prim("decr");
        self.add_prim("ltoreq");
        self.add_prim("eq");
        self.add_prim("plus");
        self.add_prim("minus");

        self.add_unit();

        match self.parse_aexp() {
            Ok(_) => {},
            Err(e) => {
                println!("parse error: {:?}", e);
            },
        }
    }

    fn parse_aexp(&mut self) -> Result<usize, Error> {
        self.log("parse_aexp");
        match self.current_token() {
            Has(Tokn::Sgqt) => self.parse_tpiq_single(),
            Has(Tokn::Pipe) => self.parse_tpiq(),
            Has(Tokn::Crrt) => self.parse_mpiq(),
            _ => self.parse_expression(),
        }
    }

    fn parse_tpiq_single(&mut self) -> Result<usize, Error> {
        self.log("parse_tpiq_single");
        self.consume_token(); // dispatcher Sgqtのはず
        match self.current_token() {
            Has(Tokn::Otag(ref otag)) => {
                self.consume_token();
                // 引数は一つ、それをqとみなす
                let qidx = (self.parse_aexp())?;
                self.match_otag(UNIT_INDX, qidx, otag)
            },
            t @ _ => Err(Error::TpiqSingle(t)),
        }
    }

    fn parse_tpiq(&mut self) -> Result<usize, Error> {
        self.log("parse_tpiq");
        self.consume_token(); // dispatcher Pipeのはず
        if let Has(Tokn::Otag(ref otag)) = self.current_token() {
            self.consume_token();
            let pidx = (self.parse_aexp())?;
            let qidx = (self.parse_aexp())?;
            self.match_otag(pidx, qidx, otag)
        } else {
            Err(Error::Unimplemented)
        }
    }

    fn parse_mpiq(&mut self) -> Result<usize, Error> {
        self.consume_token(); // dispatcher Crrtのはず
        if let Has(Tokn::Otag(ref otag)) = self.current_token() {
            self.consume_token();
            match otag.as_ref() {
                // ^Tと^Fは特別扱い
                "T" => push!(self, Epiq::Tval),
                "F" => push!(self, Epiq::Fval),
                _ => {
                    // 現在は^Tか^Fしか認めていないが、将来のため
                    let pidx = (self.parse_aexp())?;
                    let qidx = (self.parse_aexp())?;
                    push!(self, Epiq::Mpiq{o: otag.clone(), p: pidx, q: qidx})
                },
            }
        } else {
            Err(Error::Unimplemented)
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

    // expressionとは、ここでは中置記法の中の値になれるものを指している
    fn parse_expression(&mut self) -> Result<usize, Error> {
        self.log("parse_expression");
        // ここはcomsume_tokenしない
        match self.current_token() {
            Has(Tokn::Lbkt) => self.parse_list(),
            _ => {
                let l = (self.parse_term())?;
                match self.current_token() {
                    Has(Tokn::Bang) => self.parse_apply(l),
                    t @ _ => Ok(l),
                }
            }
        }
    }

    fn parse_list(&mut self) -> Result<usize, Error> {
        self.log("parse_list");
        self.consume_token();
        self.parse_list_internal()
    }

    fn parse_list_internal(&mut self) -> Result<usize, Error> {
        self.log("parse_list_internal");
        // 閉じbracketが出るまで再帰呼出
        match self.current_token() {
            Has(Tokn::Rbkt) => {
                self.consume_token();
                push!(self, Epiq::Unit)
            },
            _ => {
                let pidx = (self.parse_aexp())?;
                let qidx = (self.parse_list_internal())?;
                push!(self, Epiq::Lpiq(pidx, qidx))
            }
        }
    }

    /// "term" means resolve or literal in this context
    fn parse_term(&mut self) -> Result<usize, Error> {
        self.log("parse_term");
        match self.current_token() {
            Has(Tokn::Atsm) => self.parse_resolve(),
            _ => self.parse_literal(),
        }
    }

    fn parse_apply(&mut self, left: usize) -> Result<usize, Error> {
        self.consume_token();
        let qidx = (self.parse_expression())?;
        let id = self.vm.borrow_mut().alloc(Epiq::Appl(left, qidx));
        push!(self, Epiq::Eval(UNIT_INDX, id))
    }

    fn parse_resolve(&mut self) -> Result<usize, Error> {
        self.consume_token(); // Atsm
        let qidx = (self.parse_literal())?;
        let id = self.vm.borrow_mut().alloc(Epiq::Rslv(UNIT_INDX, qidx));
        push!(self, Epiq::Eval(UNIT_INDX, id))
    }

    fn parse_literal(&mut self) -> Result<usize, Error> {
        self.log("parse_literal");
        match self.current_token() {
            Has(Tokn::Smcl) => self.parse_unit(),
            Has(Tokn::Dbqt) => self.parse_text(),
            Has(Tokn::Chvc(ref s)) => self.parse_name(s),
            Has(Tokn::Nmbr(ref s)) => self.parse_number(s),
            _ => Err(Error::Unimplemented),
        }
    }

    fn parse_unit(&mut self) -> Result<usize, Error> {
        self.consume_token(); // Smclのはず
        Ok(UNIT_INDX)
    }

    fn parse_text(&mut self) -> Result<usize, Error> {
        self.consume_token(); // Dbqt
        if let Has(Tokn::Chvc(ref s)) = self.current_token() {
            self.consume_token();
            if let Has(Tokn::Dbqt) = self.current_token() {
                self.consume_token();
                push!(self, Epiq::Text(s.clone()))
            } else {
                Err(Error::Unimplemented)
            }
        } else {
            Err(Error::Unimplemented)
        }
    }

    fn parse_name(&mut self, s: &str) -> Result<usize, Error> {
        self.log("parse_name");
        self.consume_token();
        push!(self, Epiq::Name(s.to_string()))
    }

    fn parse_number(&mut self, s: &str) -> Result<usize, Error> {
        self.consume_token();
        push!(self, Epiq::Uit8(s.parse::<i64>().unwrap()))
    }

    // Unitは常に1つにする(index固定)
    fn add_unit(&mut self) {
        let _unit = self.vm.borrow_mut().alloc(Epiq::Unit);
    }

    fn add_prim(&mut self, name: &str) {
        let prim = self.vm.borrow_mut().alloc(Epiq::Prim(name.to_string()));
        self.vm.borrow_mut().define(name, prim);
    }

    fn consume_token(&mut self) {
        let res = self.lexer.tokenize();
        match res {
            TokenizeResult::Ok(t) => self.set_current_token(Has(t)),
            TokenizeResult::Err(_e) => {},
            TokenizeResult::EOF => self.set_current_token(EOT),
        }
    }

    fn set_current_token(&mut self, t: TokenState) {
        self.lookahead[self.p] = t;
        self.p = (self.p + 1) % K;
    }

    fn token(&self, i: usize) -> TokenState {
        self.lookahead[(self.p + i) % K].clone()
    }

    fn current_token(&self) -> TokenState {
        self.token(0)
    }


    fn log(&self, func_name: &str) {
        if false {
            println!("{}: {:?}", func_name, self.current_token());
        }
    }
}
