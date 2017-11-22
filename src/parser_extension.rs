/*
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
            let mut res;
            match &self.tokens[self.p..] {
                &[Tokn::Chvc(ref s), Tokn::Dbqt, ..] => {
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
*/


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
