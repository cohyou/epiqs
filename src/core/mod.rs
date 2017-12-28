mod graph;

// use std::cell::Cell;
pub use self::graph::*;

/// E(lemantal) piq
#[derive(Eq, PartialEq, Clone)]
pub enum Epiq {
    Unit,
    Tval,
    Fval,
    Name(String),
    Uit8(i64),
    Prim(String), // primitive function

    Tpiq { o: String, p: NodeId, q: NodeId}, // tagged piq
    Mpiq { o: String, p: NodeId, q: NodeId}, // metadata piq

    Eval(NodeId, NodeId), // eval piq
    // Apiq { p: u32, q: u32 }, // application piq

    // Text(String),

    // Lpiq { p: usize, q: usize }, // (linked) list piq
    // Vpiq { p: usize, q: usize }, // vector piq
    // Fpiq { p: usize, q: usize }, // function piq

    // Aexp { a: usize, e: usize }, // A-Expression
    // Prmt(usize), // anonymous parameter
    // Pprn(usize), // priority parentheses
    // Dbri(usize), // de bruijn index
}

impl fmt::Debug for Epiq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Epiq::Unit => write!(f, "Unit"),
            Epiq::Tval => write!(f, "Tval"),
            Epiq::Fval => write!(f, "Fval"),
            Epiq::Name(ref n) => write!(f, "Name<{}>", n),
            Epiq::Uit8(n) => write!(f, "Uit8<{}>", n),
            Epiq::Prim(ref n) => write!(f, "Prim<{}>", n),
            Epiq::Tpiq { ref o, p, q} => write!(f, "{}({} {})", o, p, q),
            Epiq::Mpiq { ref o, p, q} => write!(f, "^{}({} {})", o, p, q),
            Epiq::Eval(p, q) => write!(f, ">({} {})", p, q),
        }
    }
}
pub struct Heliqs {
    ast: NodeArena<Epiq>,
    // symbol_table: SymbolTable<'a>,
}

impl Heliqs {
    pub fn new() -> Heliqs {
        Heliqs {
            ast: NodeArena::new(),
            // symbol_table: SymbolTable::new(vec![]),
        }
    }

    pub fn alloc(&mut self, value: Epiq) -> NodeId {
        self.ast.alloc(value)
    }

    pub fn entry(&self) -> Option<NodeId> {
        self.ast.entry()
    }

    pub fn set_entry(&mut self, id: NodeId) {
        self.ast.set_entry(id)
    }

    pub fn get_epiq(&self, id: NodeId) -> &Node<Epiq> {
        self.ast.get(id)
    }

    pub fn get_epiq_mut(&mut self, id: NodeId) -> &mut Node<Epiq> {
        self.ast.get_mut(id)
    }

    pub fn define(&mut self, name: &str, value: NodeId) {
        self.ast.define(name, value)
    }

    pub fn resolve(&self, name: &str) -> Option<Option<&Node<Epiq>>> {
        self.ast.resolve(name)
    }

    pub fn extend(&mut self) {
        self.ast.extend()
    }

    pub fn pop(&mut self) {
        self.ast.pop()
    }
}
/*
pub struct AbstractSyntaxTree {
    pub entrypoint: Option<u32>,
    tree: Vec<Epiq>,
    pub max_index: Cell<u32>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree{ entrypoint: None, tree: vec![], max_index: Cell::new(0) }
    }

    pub fn get(&self, index: u32) -> &Epiq {
        &(self.tree[index as usize])
    }

    pub fn push(&mut self, epiq: Epiq) {
        self.tree.push(epiq);
        self.max_index.set((self.tree.len() - 1) as u32);
    }

    pub fn push_and_entry(&mut self, epiq: Epiq) {
        self.push(epiq);
        self.entrypoint = Some(self.max_index.get());
    }
}
*/
use std::fmt;

#[derive(Eq, PartialEq, Clone)]
pub enum Tokn {
    /* dispatcher */
    Pipe, // | vertical bar
    Crrt, // ^ carret

    // Coln, // : colon

    /* otag */
    Otag(String), // Otag

    /* literal */
    Chvc(String), // Charactor Vector 単なる文字の並び
    Nmbr(String), // Number 数値（様々な形式を含むが、まずは整数のみ）

    Smcl, // ; semi colon

    Lbkt, // [ left bracket
    Rbkt, // ] right bracket

    /*

    Usnm(String), // under score and number (e.g. _0 _34)

    // asciiのうち、記号は32(スペースを除く、また7Fも対象外)

    Dbqt, // " double quotation

    Lprn, // ( left parentheses
    Rprn, // ) right parentheses
    Lcrl, // { left curly brace
    Rcrl, // } right curly brace

    Dllr, // $ dollar

    Bang, // ! exclamation

    // 残りの記号も列挙
    Plus, // + plus
    Star, // * asterisk
    Bksl, // \ back slash
    Stop, // . full stop (period)

    Pcnt, // % percent
    Qstn, // ? question mark
    Amps, // & ampersand
    Atsm, // @ at symbol
    Hash, // # hash

    Comm, // , comma
    */

    /*
    Slsh, // / slash

    Sgqt, // ' single quotation

    Hphn, // - hyphen-minus
    Less, // < less than
    Grtr, // > greater than
    Eqls, // = equal sign

    Udsc, // _ underscore Usnmとは別に一度定義しておく
    Tild, // ~ tilde
    Bkqt, // ` back quote
    */
}

impl fmt::Debug for Tokn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tokn::Pipe => write!(f, "Pipe"),
            Tokn::Crrt => write!(f, "Crrt"),

            Tokn::Otag(ref s) => write!(f, "Otag<{}>", s),

            Tokn::Chvc(ref s) => write!(f, "Chvc<{}>", s),
            Tokn::Nmbr(ref s) => write!(f, "Nmbr<{}>", s),

            Tokn::Smcl => write!(f, "Smcl"),

            Tokn::Lbkt => write!(f, "Lbkt"),
            Tokn::Rbkt => write!(f, "Rbkt"),

            // &Tokn::Coln => write!(f, "Coln"),

            /*
            &Tokn::Usnm(ref s) => write!(f, "Usnm<{}>", s),

            &Tokn::Dbqt => write!(f, "Dbqt"),

            &Tokn::Lprn => write!(f, "Lprn"),
            &Tokn::Rprn => write!(f, "Rprn"),
            &Tokn::Lcrl => write!(f, "Lcrl"),
            &Tokn::Rcrl => write!(f, "Rcrl"),


            &Tokn::Dllr => write!(f, "Dllr"),

            &Tokn::Bang => write!(f, "Bang"),

            // 扱いが不明瞭だがひとまず足しておく
            &Tokn::Plus => write!(f, "Plus"),
            &Tokn::Star => write!(f, "Star"),
            &Tokn::Stop => write!(f, "Stop"),
            &Tokn::Bksl => write!(f, "Bksl"),

            &Tokn::Pcnt => write!(f, "Pcnt"),
            &Tokn::Qstn => write!(f, "Qstn"),
            &Tokn::Amps => write!(f, "Amps"),
            &Tokn::Atsm => write!(f, "Atsm"),
            &Tokn::Hash => write!(f, "Hash"),

            &Tokn::Comm => write!(f, "Comm"),
            */

            // _ => write!(f, "????"),
        }
    }
}

#[test]
fn epiq_arena_get() {
    let mut arena = NodeArena::<Epiq>::new();
    let node_id = arena.alloc(Epiq::Unit);
    {
        let mut node = arena.get_mut(node_id);
        node.1 = Epiq::Name("wowow".to_string());
    }
    assert_eq!(arena.get(node_id).1, Epiq::Name("wowow".to_string()));
}

/*
#[test]
fn symbol_table() {
    let prim = Node(0, Epiq::Prim("decr".to_string()));
    let prims = vec![("decr".to_string(), Some(&prim)),];
    let _table = SymbolTable::new(prims);
}
*/
