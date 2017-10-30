// use std::fmt::Write;

use super::lexer::{Lexer, Tokn};
use super::core::{Epiq, Heliqs};

#[derive(Debug)]
pub enum ParseError {
    UnknownError,
    Int8CastError,
    TextError,
    // NotAexpError,
    NotAffxError,
    NotCpiqError,
    CanNotCloseParenError,
    NotTrueListError,
    NotDebruijnIndexError,
    Next(String),
}


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: Vec<Tokn>,
    p: usize, // index of current token
    markers: Vec<usize>, // use as stack of returning points when backtracking
    vm: Heliqs,
    // output: String,
}

impl<'a> Parser<'a> {

    pub fn new(mut l: Lexer<'a>) -> Parser {
        let ts = match l.next_token() {
            Ok(t) => vec![t],
            _ => vec![]
        };
        Parser { lexer: l, tokens: ts, vm:Heliqs { vctr: vec![] }, p: 0, markers: vec![], /*output: "".to_string(),*/ }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        match self.parse_aexp() {
            Ok(i) => {
                // 評価前
                // println!("parse self.vm.vctr: {:#?}", self.vm.vctr);
                // self.print_aexp(i);
                // println!("");

                match self.vm.vctr.get(i) {
                    Some(&Epiq::Aexp{ a, e }) => self.beta_reduct(e),
                    _ => println!("{:?}", "not A-Expression"),
                }

                // 評価後
                // self.print_aexp(i);
                // println!("");

                Ok(())
            },

            Err(e) => {
                // println!("");
                Err(e)
            },
        }
    }


    /* PARSING */

    fn parse_aexp(&mut self) -> Result<usize, ParseError> {
        // println!("{:?}", ("parse_aexp", &self.tokens));

        match self.parse_affx() {
            Ok(a) => {
                match self.parse_epiq() {
                    Ok(e) => {
                        let a = Epiq::Aexp { a: a, e: e };
                        self.vm.vctr.push(a);
                        Ok(self.vm.vctr.len() - 1)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// for parsing of cons list
    fn parse_aexp_excluding_cons(&mut self) -> Result<usize, ParseError> {
        match self.parse_affx() {
            Ok(a) => {
                match self.parse_epiq_excluding_cons() {
                    Ok(e) => {
                        let a = Epiq::Aexp { a: a, e: e };
                        self.vm.vctr.push(a);
                        Ok(self.vm.vctr.len() - 1)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_affx(&mut self) -> Result<usize, ParseError> {
        self.vm.vctr.push(Epiq::Unit);
        Ok(self.vm.vctr.len() - 1)
        /*
        match self.parse_ctgr() {
            Ok(i) => Ok(i),
            Err(ParseError::Next(_)) => {},
            _ => Err(ParseError::NotAffxError),
        }
        */
    }

    fn parse_epiq(&mut self) -> Result<usize, ParseError> {
        match self.parse_ctgr() {
            Ok(i) => Ok(i),
            Err(ParseError::Next(_)) => {
                match self.parse_list() {
                    Ok(i) => {
                        println!("VM: {:?}", self.vm.vctr);
                        Ok(i)
                    },

                    Err(ParseError::Next(_)) => {
                        if self.speculate_cpiq() {
                            self.parse_cpiq()
                        } else {
                            self.parse_pexp()
                        }
                    },

                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// for parsing of cons list
    fn parse_epiq_excluding_cons(&mut self) -> Result<usize, ParseError> {
        // println!("parse_epiq_excluding_cons self.tokens: {:?}", &self.tokens[self.p..]);
        match self.parse_list() {
            Ok(i) => {
                println!("VM: {:#?}", self.vm.vctr);
                Ok(i)
            },

            Err(ParseError::Next(_)) => {
                self.parse_pexp()
            },

            Err(e) => Err(e),
        }
    }


    fn parse_ctgr(&mut self) -> Result<usize, ParseError> {
        // println!("parse_ctgr self.tokens: {:?}", &self.tokens[self.p..]);
        match self.get_target_token() {
            Some(Tokn::Crrt) => {
                self.consume_token();
                // println!("parse_ctgr self.tokens: {:?}", &self.tokens[self.p..]);
                match self.get_target_token() {
                    Some(Tokn::Dllr) => {
                        self.consume_token();
                        // ^$ は cons系より結合力が低いので貪欲にparseするように
                        match self.parse_aexp() {
                            Ok(i) => {
                                self.vm.vctr.push(Epiq::Prmt(i));
                                Ok(self.vm.vctr.len() - 1)
                            },
                            Err(e) => Err(e),
                        }
                    },
                    _ => Err(ParseError::NotAffxError),
                }
            },
            _ => Err(ParseError::Next("DontCTGR".to_string())),
        }
    }

    fn parse_list(&mut self) -> Result<usize, ParseError> {
        // println!("{:?}", ("parse_list", &self.tokens));
        match self.get_target_token() {
            Some(Tokn::Lbkt) => {
                self.consume_token();
                self.parse_list_internal()
            },

            _ => Err(ParseError::Next("DontStartWithLBKT".to_string())),
        }
    }

    fn parse_list_internal(&mut self) -> Result<usize, ParseError> {
        match self.get_target_token() {
            Some(Tokn::Rbkt) => {
                self.consume_token();
                self.vm.vctr.push(Epiq::Unit);
                Ok(self.vm.vctr.len() - 1)
            },

            _ => {
                match self.parse_aexp() {
                    Ok(i1) => {
                        match self.parse_list_internal() {
                            Ok(i2) => {
                                let l = Epiq::Lpiq { p: i1, q: i2 };
                                self.vm.vctr.push(l);
                                Ok(self.vm.vctr.len() - 1)
                            },
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
        }
    }

    fn speculate_cpiq(&mut self) -> bool {
        let mut res = true;
        self.add_marker();
        // println!("speculate_cpiq1 self.tokens: {:?}", &self.tokens[self.p..]);
        match self.parse_aexp_excluding_cons() {
            Ok(_) => {
                // println!("speculate_cpiq2 self.tokens: {:?}", &self.tokens[self.p..]);
                match self.get_target_token() {
                    Some(Tokn::Coln) => { res = true },
                    Some(Tokn::Pipe) => {
                        self.consume_token();
                        match self.get_target_token() {
                            Some(Tokn::Crrt) => { res = true },
                            Some(Tokn::Bang) => { res = true },
                            _ => {res = false },
                        }
                    },
                    _ => { res = false },
                }
            },
            Err(_) => { res = false },
        }
        self.release_marker();
        // println!("speculate_cpiq: {:?}", res);
        res
    }

    fn parse_cpiq(&mut self) -> Result<usize, ParseError> {
        // println!("parse_cpiq self.tokens: {:?} self.p: {:?}", self.tokens, self.p);
        match self.parse_aexp_excluding_cons() {
            Ok(i1) => {
                // println!("parse_cpiq self.tokens: {:?}", &self.tokens[self.p..]);
                match self.get_target_token() {
                    Some(Tokn::Coln) => {
                        self.consume_token();
                        match self.parse_aexp() {
                            Ok(i2) => {
                                let l = Epiq::Lpiq { p: i1, q: i2 };
                                self.vm.vctr.push(l);
                                Ok(self.vm.vctr.len() - 1)
                            },
                            Err(e) => Err(e),
                        }
                    },

                    Some(Tokn::Pipe) => {
                        self.consume_token();
                        match self.get_target_token() {
                            Some(Tokn::Crrt) => {
                                self.consume_token();
                                match self.parse_aexp() {
                                    Ok(i2) => {
                                        let l = Epiq::Fpiq { p: i1, q: i2 };
                                        self.vm.vctr.push(l);
                                        Ok(self.vm.vctr.len() - 1)
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            // TODO: Crrtの場合とコードが重複しているのでまとめたい
                            Some(Tokn::Bang) => {
                                self.consume_token();
                                match self.parse_aexp() {
                                    Ok(i2) => {
                                        let l = Epiq::Apiq { p: i1, q: i2 };
                                        self.vm.vctr.push(l);
                                        Ok(self.vm.vctr.len() - 1)
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            _ => Err(ParseError::NotCpiqError),
                        }
                    },

                    _ => {
                        // self.parse_pexp()
                        Err(ParseError::NotCpiqError)
                    },
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_pexp(&mut self) -> Result<usize, ParseError> {
        // println!("parse_pexp self.tokens: {:?}", &self.tokens[self.p..]);
        match self.parse_unit() {
            Ok(i) => return Ok(i),
            Err(ParseError::Next(_)) => {},
            Err(e) => return Err(e),
        }

        match self.parse_primary_parentheses() {
            Err(ParseError::Next(_)) => {
                match self.parse_text() {
                    Err(ParseError::Next(_)) => {
                        match self.parse_number() {
                            Err(ParseError::Next(_)) => self.parse_de_bruijn_index(),
                            Ok(e) => Ok(e),
                            Err(e) => Err(e),
                        }
                    },
                    Ok(e) => Ok(e),
                    Err(e) => Err(e),
                }
            }
            Ok(e) => Ok(e),
            Err(e) => Err(e),
        }
    }

    fn parse_unit(&mut self) -> Result<usize, ParseError> {
        if self.get_target_token() == Some(Tokn::Smcl) {
            self.consume_token();
            self.vm.vctr.push(Epiq::Unit);
            return Ok(self.vm.vctr.len() - 1);
        } else {
            Err(ParseError::Next("DontUNIT".to_string()))
        }
    }

    // これは優先順位表現用のカッコのparseなので、中に入れられる要素は一つだけ
    fn parse_primary_parentheses(&mut self) -> Result<usize, ParseError> {
        match self.get_target_token() {
            Some(Tokn::Lprn) => {
                self.consume_token();
                match self.parse_aexp() {
                    Ok(i) => {
                        match self.get_target_token() {
                            Some(Tokn::Rprn) => {
                                self.consume_token();
                                // let p = Epiq::Pprn(i);
                                // self.vm.vctr.push(p);
                                // Ok(self.vm.vctr.len() - 1)
                                Ok(i)
                            },
                            _ => Err(ParseError::CanNotCloseParenError),
                        }
                    }
                    Err(e) => Err(e),
                }
            },
            _ => Err(ParseError::Next("DontStartWithLPRN".to_string())),
        }
    }

    fn parse_text(&mut self) -> Result<usize, ParseError> {
        // println!("parse_text self.tokens: {:?} self.p: {:?}", self.tokens, self.p);
        match self.get_target_token() {
            Some(Tokn::Dbqt) => {
                self.consume_token();
                self.sync_tokens(2);
                // println!("parse_text self.tokens: {:?} self.p: {:?}", self.tokens, self.p);
                let mut res = Ok(0);
                match &self.tokens[self.p..] {
                    &[Tokn::Text(ref s), Tokn::Dbqt, ..] => {
                        if !self.is_speculating() {
                            self.vm.vctr.push(Epiq::Text(s.to_string()));
                        }
                        res = Ok(self.vm.vctr.len() - 1);
                    },
                    _ => { res = Err(ParseError::TextError); },
                }
                if res.is_ok() {
                    self.consume_token();
                    self.consume_token();
                }
                res
            },
            _ => Err(ParseError::Next("DontStartWithDBQT".to_string())),
        }
    }

    fn parse_number(&mut self) -> Result<usize, ParseError> {
        // println!("parse_number self.tokens: {:?}", &self.tokens[self.p..]);
        match self.get_target_token() {
            Some(Tokn::Nmbr(ref s)) => {
                self.consume_token();
                if let Ok(n) = i64::from_str_radix(s, 10) {
                    // self.push_token(Epiq::Int8(n));
                    if !self.is_speculating() {
                        self.vm.vctr.push(Epiq::Int8(n));
                    }
                    Ok(self.vm.vctr.len() - 1)
                } else {
                    Err(ParseError::Int8CastError)
                }
            },
            _ => Err(ParseError::Next("DontNMBR".to_string())),
        }
    }

    fn parse_de_bruijn_index(&mut self) -> Result<usize, ParseError> {
        match self.get_target_token() {
            Some(Tokn::Usnm(ref s)) => {
                self.consume_token();
                if let Ok(n) = i64::from_str_radix(&s[1..], 10) {
                    if !self.is_speculating() {
                        self.vm.vctr.push(Epiq::Dbri(n as usize));
                    }
                    Ok(self.vm.vctr.len() - 1)
                } else {
                    Err(ParseError::NotDebruijnIndexError)
                }
            }
            _ => Err(ParseError::Next("DontDBRI".to_string())),
        }
    }

    /*
    fn push_token(&self, e: Epiq) {
    if !self.is_speculating() {
    self.vm.vctr.push(e);
    }
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
        if self.p == self.tokens.len() && !self.is_speculating() {
            self.p = 0;
            self.tokens.clear();
        }
        self.sync_tokens(1);
    }

    fn add_marker(&mut self) -> usize {
        self.markers.push(self.p);
        self.p
    }

    fn release_marker(&mut self) {
        if let Some(n) = self.markers.last() {
            // self.seek_marker(n.clone());
            self.p = n.clone();
        } else {
            return;
        }
        self.markers.pop();
    }

    fn seek_marker(&mut self, index: usize) {
        self.p = index;
    }

    fn is_speculating(&self) -> bool {
        !self.markers.is_empty()
    }


    /* PRINTING */

    pub fn print_aexp(&self, i: usize) -> String {
        let mut result = "".to_string();

        if let Some(c) = self.vm.vctr.get(i) {
            match c {
                &Epiq::Aexp { a, e } => {
                    result.push_str(&self.print_affx(a));
                    result.push_str(&self.print_epiq(e));
                },
                _ => {},
            }
        }
        result.push_str("\n");
        return result;
    }

    fn print_affx(&self, i: usize) -> String {
        let mut result = "".to_string();

        if let Some(c) = self.vm.vctr.get(i) {
            match c {
                &Epiq::Unit => {}, // case of 'has no affx'
                _ => {},
            }
        }

        return result;
    }

    fn print_epiq(&self, i: usize) -> String {
        let mut result = "".to_string();

        // check whether 'true list' or not
        let is_true_list = self.check_true_list(i);

        if let Some(c) = self.vm.vctr.get(i) {
            match c {
                &Epiq::Lpiq { p, q } => {
                    if is_true_list {
                        result.push_str("[");
                        result.push_str(&self.print_list(p, q));
                    } else {
                        result.push_str(&self.print_piq(" : ", p, q));
                    }
                },
                &Epiq::Fpiq { p, q } => result.push_str(&self.print_piq(" |^ ", p, q)),
                &Epiq::Apiq { p, q } => result.push_str(&self.print_piq(" |! ", p, q)),

                &Epiq::Prmt(i) => {
                    result.push_str("Prmt(");
                    result.push_str(&self.print_aexp(i));
                    result.push_str(")");
                },
                &Epiq::Aexp { a, e } => {
                    result.push_str(&self.print_affx(a));
                    result.push_str(&self.print_epiq(e));
                },
                &Epiq::Pprn(i) => {
                    result.push_str("(");
                    // print!("{:?}", self.check_true_list(i));
                    result.push_str(&self.print_aexp(i));
                    result.push_str(")");
                },
                _ => result.push_str(&format!("{:?}", c)),
            }
        }

        return result;
    }

    fn print_piq(&self, op: &str, p: usize, q: usize) -> String {
        let mut result = "".to_string();

        if self.vm.vctr.get(p).is_some() {
            result.push_str(&self.print_aexp(p));
        }
        print!("{:}",op);
        if self.vm.vctr.get(q).is_some() {
            result.push_str(&self.print_aexp(q));
        }

        return result;
    }

    /// check whether 'true list' or not
    fn check_true_list(&self, i: usize) -> bool {
        let mut idx = i;
        while let Some(c) = self.vm.vctr.get(idx) {
            match c {
                &Epiq::Aexp { a, e } => {
                    match self.vm.vctr.get(e) {
                        Some(&Epiq::Unit) => return true,
                        _ => return false,
                    }
                },
                &Epiq::Unit => return true, // もはやここは通らないと思うが、一旦残しておく
                &Epiq::Lpiq { p, q } => {
                    idx = q;
                },
                _ => return false,
            }
        }
        false
    }

    fn print_list(&self, pi: usize, qi: usize) -> String {
        let mut result = "".to_string();

        if self.vm.vctr.get(pi).is_some() {
            result.push_str(&self.print_aexp(pi));
        }

        match self.vm.vctr.get(qi) {
            Some(&Epiq::Aexp { a, e }) => {
                match self.vm.vctr.get(e) {
                    Some(&Epiq::Unit) => result.push_str("]"),
                    _ => {},
                }
            }
            Some(&Epiq::Lpiq { p, q }) => {
                result.push_str(" ");
                result.push_str(&self.print_list(p, q));
            },
            None => result.push_str(")"),
            _ => result.push_str(&format!("error on print vm: {:?}", self.vm.vctr)),
        }

        return result;
    }

    fn beta_reduct(&mut self, entry: usize) {
        /*
        // 仮引数に実引数を入れる
        let mut a = vec![1, 3, 5];
        a.remove(1);
        a.insert(1, 2);
        println!("{:?}", a);

        // confirm that target is "application-piq"
        match self.vm.vctr.get(entry) {
            Some(&Epiq::Apiq { p: p1, q: q1 }) => {
                /* 引数をまず、lambdaの中に入れる */

                // ApiqのpはFpiqである必要がある(=ラムダ適用の左側はラムダ式でなければならない)
                match self.vm.vctr.get(p1) {
                    Some(&Epiq::Fpiq { p: p2, q: q2 }) => {

                        // FpiqのpはPrmtである必要がある(=ラムダ式のは引数のシンボルが入る)
                        match self.vm.vctr.get(p2) {
                            Some(&Epiq::Prmt(_)) => {
                            },
                            Some(p3) => println!("not anonymous parameter {:?}", p3),
                            None => println!("{:?}", "index error"),
                        }
                    },
                    Some(p2) => println!("not function epiq {:?}", p2),
                    None => println!("{:?}", "index error"),
                }

                println!("v.len(): {:?} entry: {:?}", self.vm.vctr.len(), entry);
            },
            Some(p1) => println!("not application epiq {:?}", p1),
            None => println!("{:?}", "index error"),
        }
        */

        let mut is_ok: (usize, usize) = (0, 0);

        match apiq_f((self, entry))
            // .and_then(aexp_e).and_then(pprn)
            .and_then(aexp_e).and_then(fpiq_p)
            // .and_then(aexp_e).and_then(pprn)
            .and_then(aexp_e) {
                Some(t) => {
                    println!("{:?}", t.1);
                    match apiq_q((self, entry)) {
                        Some(t2) => {
                            is_ok = (t.1, t2.1);
                        }
                        _ => {},
                    }
                }
                None => {},
        }

        if is_ok != (0, 0) {
            self.vm.vctr.remove(is_ok.0);
            self.vm.vctr.insert(is_ok.0, Epiq::Prmt(is_ok.1));
        }
    }
}

fn aexp_e<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(&Epiq::Aexp { a, e }) => Some((t.0, e)),
        _ => None,
    }
}

fn apiq_f<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(&Epiq::Apiq { p, q }) => Some((t.0, p)),
        _ => None,
    }
}

fn apiq_q<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(&Epiq::Apiq { p, q }) => Some((t.0, q)),
        _ => None,
    }
}

fn fpiq_p<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(&Epiq::Fpiq { p, q }) => Some((t.0, p)),
        Some(a) => {
            println!("not fpiq: {:?}", a);
            None
        },
        _ => None,
    }
}

fn pprn<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(&Epiq::Pprn(i)) => Some((t.0, i)),
        Some(a) => {
            println!("not pprn: {:?}", a);
            None
        },
        _ => None,
    }
}

fn print_one_piq<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
    match t.0.vm.vctr.get(t.1) {
        Some(e) => {
            println!("print_one_piq: {:?}", e);
            Some(t)
        },
        _ => None,
    }
}
