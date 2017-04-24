use std::fmt::Write;

use super::lexer::Tokn;
use super::core::Epiq;

#[derive(Debug)]
pub struct ParseError(pub String);

impl ParseError {
    pub fn new(s: &str) -> ParseError {
        ParseError(s.to_string())
    }
}


pub struct Parser {}

impl Parser {
    pub fn parse(tokens: &[Tokn]) -> Result<Box<Epiq>, ParseError> {
        match Parser::parse_eexp(tokens) {
            Ok((v, _)) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn parse_eexp(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        Parser::parse_epiq(tokens)
    }

    fn parse_epiq(tokens: &[Tokn]) -> Result<(Box<Epiq>, &[Tokn]), ParseError> {
        match Parser::parse_list(tokens) {
            Ok((v, r)) => Ok((v, r)),
            Err(ParseError(ref s)) if s == "next" => {
                match Parser::parse_pexp(tokens) {
                    Ok((v, r)) => Ok((v, r)),
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
                match Parser::parse_eexp(tokens) {
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
}
