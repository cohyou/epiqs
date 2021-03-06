/*
pub struct Parser<'a> {
    lexer: Lexer<'a, 'a>,
    tokens: Vec<Tokn>,
    p: usize, // index of current token
    // markers: Vec<usize>, // use as stack of returning points when backtracking
    vm: Heliqs,
}

impl<'a> Parser<'a> {

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
}

/// for parsing of cons list
fn parse_aexp_excluding_cons(&mut self) -> Result<usize, Error> {
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


fn parse_epiq(&mut self) -> Result<usize, Error> {
    match self.parse_ctgr() {
        Ok(i) => Ok(i),
        Err(Error::Next(_)) => {
            match self.parse_list() {
                Ok(i) => {
                    println!("VM: {:?}", self.vm.vctr);
                    Ok(i)
                },

                Err(Error::Next(_)) => {
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
fn parse_epiq_excluding_cons(&mut self) -> Result<usize, Error> {
    // println!("parse_epiq_excluding_cons self.tokens: {:?}", &self.tokens[self.p..]);
    match self.parse_list() {
        Ok(i) => {
            println!("VM: {:#?}", self.vm.vctr);
            Ok(i)
        },

        Err(Error::Next(_)) => {
            self.parse_pexp()
        },

        Err(e) => Err(e),
    }
}


fn parse_ctgr(&mut self) -> Result<usize, Error> {
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
                _ => Err(Error::NotAffxError),
            }
        },
        _ => Err(Error::Next("DontCTGR".to_string())),
    }
}


fn speculate_cpiq(&mut self) -> bool {
    let res;
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

fn parse_cpiq(&mut self) -> Result<usize, Error> {
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
                        _ => Err(Error::NotCpiqError),
                    }
                },

                _ => {
                    // self.parse_pexp()
                    Err(Error::NotCpiqError)
                },
            }
        },
        Err(e) => Err(e),
    }
}

fn parse_pexp(&mut self) -> Result<usize, Error> {
    // println!("parse_pexp self.tokens: {:?}", &self.tokens[self.p..]);
    match self.parse_unit() {
        Ok(i) => return Ok(i),
        Err(Error::Next(_)) => {},
        Err(e) => return Err(e),
    }

    match self.parse_primary_parentheses() {
        Err(Error::Next(_)) => {
            match self.parse_text() {
                Err(Error::Next(_)) => {
                    match self.parse_number() {
                        Err(Error::Next(_)) => self.parse_de_bruijn_index(),
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


// これは優先順位表現用のカッコのparseなので、中に入れられる要素は一つだけ
fn parse_primary_parentheses(&mut self) -> Result<usize, Error> {
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
                        _ => Err(Error::CanNotCloseParenError),
                    }
                }
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::Next("DontStartWithLPRN".to_string())),
    }
}



fn parse_de_bruijn_index(&mut self) -> Result<usize, Error> {
    match self.get_target_token() {
        Some(Tokn::Usnm(ref s)) => {
            self.consume_token();
            if let Ok(n) = i64::from_str_radix(&s[1..], 10) {
                if !self.is_speculating() {
                    self.vm.vctr.push(Epiq::Dbri(n as usize));
                }
                Ok(self.vm.vctr.len() - 1)
            } else {
                Err(Error::NotDebruijnIndexError)
            }
        }
        _ => Err(Error::Next("DontDBRI".to_string())),
    }
}

/*

これらは中置記法用に必要なbasic functions

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

/*
fn seek_marker(&mut self, index: usize) {
    self.p = index;
}
*/

fn is_speculating(&self) -> bool {
    !self.markers.is_empty()
}
*/
