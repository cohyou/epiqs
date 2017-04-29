use std::fmt;
use std::fmt::Debug;

/// A(ffix)-expression
// #[derive(Debug)]
pub struct Aexp {
    mark: Vec<Epiq>, // @ annotation
    envr: Vec<Epiq>, // $ attribute
    evnt: Vec<Epiq>, // % attribute
    cond: Vec<Epiq>, // ? attribute
    appl: Vec<Epiq>, // ! action
    ctgr: Vec<Epiq>, // \ annotation
    epiq: Epiq,
    slce: Vec<Epiq>, // # action
}

impl Aexp {
    pub fn only_epiq(e: Epiq) -> Aexp {
        Aexp {mark: vec![], envr: vec![], evnt: vec![], cond: vec![],
              appl: vec![], ctgr: vec![], epiq: e     , slce: vec![]}
    }
}

impl Debug for Aexp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ss = vec![];
        if !self.mark.is_empty() {
            ss.push(&self.mark);
        }
        if !self.envr.is_empty() {
            ss.push(&self.envr);
        }
        if !self.evnt.is_empty() {
            ss.push(&self.evnt);
        }
        if !self.cond.is_empty() {
            ss.push(&self.cond);
        }
        if !self.appl.is_empty() {
            ss.push(&self.appl);
        }
        if !self.ctgr.is_empty() {
            ss.push(&self.ctgr);
        }
        if !self.slce.is_empty() {
            ss.push(&self.slce);
        }
        if ss.is_empty() {
            write!(f, "{:?}", self.epiq)
        } else {
            write!(f, "Aexp {{ {:?} {:?} }}", self.epiq, ss)
        }

    }
}

/// E(lemantal) piq
#[derive(Debug)]
pub enum Epiq {
    Unit,
    Int8(i64),
    Text(String),

    Cpiq{ p: Box<Aexp>, q: Box<Aexp> }, // cons cell piq
}
