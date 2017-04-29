use std::fmt::Write;

use super::lexer::{Lexer, Tokn};
use super::core::{Aexp, Epiq};

#[derive(Debug)]
pub enum ParseError {
    UnknownError,
    Int8CastError,
    TextError,
    Next(String),
}


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_tokens: Vec<Tokn>,
}

impl<'a> Parser<'a> {

    pub fn new(mut l: Lexer<'a>) -> Parser {
        let ts = vec![l.next_token().unwrap()];
        Parser { lexer: l, current_tokens: ts }
    }

    pub fn parse(&mut self) -> Result<Aexp, ParseError> {
        self.parse_aexp()
        /*
        while let Ok(t) = self.lexer.next_token() {
            println!("{:?}", t);
        }
        */
    }

    fn parse_aexp(&mut self) -> Result<Aexp, ParseError> {
        // println!("{:?}", ("parse_aexp", &self.current_tokens));
        match self.parse_epiq() {
            Ok(e) => Ok(Aexp::only_epiq(e)),
            Err(e) => Err(e),
        }
    }

    fn parse_epiq(&mut self) -> Result<Epiq, ParseError> {
        // println!("{:?}", ("parse_epiq", &self.current_tokens));
        match self.parse_list() {
            Err(ParseError::Next(_)) => self.parse_pexp(),
            Ok(e) => Ok(e),
            _ => Err(ParseError::UnknownError),
        }
    }

    fn parse_list(&mut self) -> Result<Epiq, ParseError> {
        println!("{:?}", ("parse_list", &self.current_tokens));
        match self.current_tokens[0] {
            Tokn::Lbkt => {
                self.consume_token();
                self.parse_list_internal()
            },

            Tokn::Rbkt => Ok(Epiq::Unit),

            _ => Err(ParseError::Next("DontStartWithLBKT".to_string())),
        }
    }

    fn parse_list_internal(&mut self) -> Result<Epiq, ParseError> {
        match self.parse_aexp() {
            Ok(ax) => {
                self.consume_token();
                Ok(Epiq: Cpiq { p: ax, q: self.parse_list_internal() }),
            },
            Err(e) => Err(e),
        }
    }

    fn parse_pexp(&mut self) -> Result<Epiq, ParseError> {
        println!("{:?}", ("parse_pexp", &self.current_tokens));
        match self.parse_text() {
            Err(ParseError::Next(_)) => self.parse_number(),
            Ok(e) => Ok(e),
            _ => Err(ParseError::UnknownError),
        }
    }

    fn parse_text(&mut self) -> Result<Epiq, ParseError> {
        println!("{:?}", ("parse_text", &self.current_tokens));
        let mut res = Err(ParseError::UnknownError);
        match self.current_tokens[0] {
            Tokn::Dbqt => {
                self.consume_two_tokens();
                match &self.current_tokens[..] {
                    &[Tokn::Text(ref s), Tokn::Dbqt, ..] => {
                        res = Ok(Epiq::Text(s.to_string()))
                    },
                    _ => return Err(ParseError::TextError),
                }
            },
            _ => return Err(ParseError::Next("DontStartWithDBQT".to_string())),
        }

        // self.consume_token();
        res
    }

    fn parse_number(&mut self) -> Result<Epiq, ParseError> {
        println!("{:?}", ("parse_nmbr", &self.current_tokens));
        let mut res = Err(ParseError::UnknownError);
        match self.current_tokens[0] {
            Tokn::Nmbr(ref s) => {
                if let Ok(n) = i64::from_str_radix(s, 10) {
                    res = Ok(Epiq::Int8(n));
                } else {
                    return Err(ParseError::Int8CastError);
                }
            },
            _ => return Err(ParseError::Next("DontNMBR".to_string())),
        }

        // self.consume_token();
        res
    }

    fn consume_two_tokens(&mut self) {
        let first = self.lexer.next_token().unwrap();
        let second = self.lexer.next_token().unwrap();
        self.current_tokens = vec![first, second];
    }

    fn consume_token(&mut self) {
        self.current_tokens = vec![self.lexer.next_token().unwrap()];
    }

    /*
    fn parse_aexp(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        Parser::parse_epiq(tokens)
    }

    fn parse_epiq(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match Parser::parse_list(tokens) {
            Ok((v, r)) => Ok((v, r)),
            Err(ParseError(ref s)) if s == "next" => {
                match Parser::parse_cpiq(tokens) {
                    Ok((v, r)) => Ok((v, r)),
                    Err(ParseError(ref s)) if s == "next" => {
                        match Parser::parse_pexp(tokens) {
                            Ok((v, r)) => Ok((v, r)),
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_pexp(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match tokens {
            // Number
            &[Tokn::Nmbr(ref s), ref cdr..] => {
                if let Ok(n) = i64::from_str_radix(s, 10) {
                    Ok((box Epiq::Int8(n), cdr))
                } else {
                    let mut desc = String::new();
                    write!(&mut desc, "CONVERT TO I64 INTEGER: {:?}", s).unwrap();
                    Err(ParseError::new(&desc))
                }
            },

            // Text
            &[Tokn::Dbqt, Tokn::Text(ref s), Tokn::Dbqt, ref cdr..] => {
                Ok((box Epiq::Text(s.to_string()), cdr))
            },

            _ => {
                let mut desc = String::new();
                write!(&mut desc, "UNKNOWN ERROR: [{:?}]", tokens[0]).unwrap();
                Err(ParseError::new(&desc))
            }
        }
    }

    fn parse_cpiq(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match Parser::parse_pexp(tokens) {
            Ok((p, r)) => {
                println!("(p, r):{:?}", (&p, r));
                match r {
                    &[Tokn::Coln, ref cdr..] => {
                        println!("cdr:{:?}", cdr);
                        match Parser::parse_aexp(cdr) {
                            Ok((q, r2)) => Ok((box Epiq::Cpiq { p: p, q: q }, r2)),
                            Err(e) => {
                                println!("e:{:?}", e);
                                Err(e)},
                        }
                    },

                    _ => Err(ParseError::new("next")),
                }
            }

            Err(_) => Err(ParseError::new("next")),
        }
    }

    fn parse_list(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match tokens {
            &[Tokn::Lbkt, Tokn::Rbkt, ref cdr..] => {
                Ok((box Epiq::Cpiq { p: box Epiq::Unit, q: box Epiq::Unit }, cdr))
            },

            &[Tokn::Lbkt, ref cdr..] => Parser::parse_list_internal(cdr),

            _ => Err(ParseError::new("next")),
        }
    }

    fn parse_list_internal(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match tokens {
            &[Tokn::Rbkt, ref cdr..] => Ok((box Epiq::Unit, cdr)),
            _ => {
                match Parser::parse_aexp(tokens) {
                    Ok((v, r)) => {
                        match Parser::parse_list_internal(r) {
                            Ok((v_next, r_next)) => Ok((box Epiq::Cpiq { p: v, q: v_next }, r_next)),
                            Err(_) => {
                                let mut desc = String::new();
                                write!(&mut desc, "LIST INTERNAL CALLED FROM LIST INTERNAL: [{:?}]", tokens[0]).unwrap();
                                Err(ParseError::new(&desc))
                            }
                        }
                    },

                    Err(_) => {
                        let mut desc = String::new();
                        write!(&mut desc, "E-EXP CALLED FROM LIST INTERNAL: [{:?}]", tokens[0]).unwrap();
                        Err(ParseError::new(&desc))
                    },
                }
            },
        }
    }
    */
}
