/// E(lemantal) piq
#[derive(Debug)]
pub enum Epiq {
    Name(String),
    Tpiq { o: String, p: usize, q: usize}, // tagged piq

    // Unit,
    // Int8(i64),
    // Text(String),

    // Lpiq { p: usize, q: usize }, // (linked) list piq
    // Vpiq { p: usize, q: usize }, // vector piq
    // Fpiq { p: usize, q: usize }, // function piq
    // Apiq { p: usize, q: usize }, // application piq
    // Aexp { a: usize, e: usize }, // A-Expression
    // Prmt(usize), // anonymous parameter
    // Pprn(usize), // priority parentheses
    // Dbri(usize), // de bruijn index
}

pub struct Heliqs {
    pub vctr: Vec<Epiq>,
}

/*
use std::fmt;
use std::fmt::Debug;

/// A(ffix)-expression
#[derive(Debug)]
pub struct Aexp {
    mark: Vec<Epiq>, // @ annotation
    envr: Vec<Epiq>, // $ attribute
    evnt: Vec<Epiq>, // % attribute
    cond: Vec<Epiq>, // ? attribute
    appl: Vec<Epiq>, // ! action
    ctgr: Vec<Epiq>, // ^ annotation
    epiq: Epiq,
    slce: Vec<Epiq>, // / action
}
*/
