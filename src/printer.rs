use std::cell::RefCell;
// use std::ops::Deref;
use core::*;
use lexer::*;
use parser::*;
use evaluator::*;

struct Printer<'a> {
    ast: &'a RefCell<AbstractSyntaxTree>,
}

impl<'a> Printer<'a> {
    pub fn new(ast: &'a RefCell<AbstractSyntaxTree>) -> Self {
        Printer{ ast: ast }
    }

    pub fn print(&self) -> String {
        if let Some(entrypoint) = self.ast.borrow().entrypoint {
            self.print_aexp(entrypoint, 0)
        } else {
            "".to_string()
        }
    }

    fn print_aexp(&self, i: u32, nest_level: u32) -> String {
        let ast = self.ast.borrow();
        let epiq = ast.get(i);
        match *epiq {
            Epiq::Name(ref n) => n.to_string(),
            Epiq::Uit8(ref n) => format!("{}", n),
            Epiq::Unit => ";".to_string(),
            Epiq::Tpiq { ref o, p, q } => {
                format!("{}< {} {} >", o, self.print_aexp(p, nest_level + 1), self.print_aexp(q, nest_level + 1))
            }
            // _ => "".to_string(),
        }
    }
}

#[test]
#[ignore]
fn test_print_symbol() {
    print_str("abc", "abc");
}

#[test]
#[ignore]
fn test_print_number() {
    print_str("123", "123");
}

#[test]
fn test_print_unit() {
    print_str(";", ";");
}

#[test]
#[ignore]
fn test_print_tpiq() {
    // print_str("|: abc 123", ":< abc 123 >");
    print_str("|# abc 123", "#< abc 123 >");
}

#[test]
fn test_print_nested_tpiq() {
    print_str("|: |: cde |: abc 123 456", ":< :< cde :< abc 123 > > 456 >");
}

#[test]
fn test_print_list() {
    print_str("[abc 123]", ":< abc :< 123 ; > >");
}

#[test]
fn test_print_empty_env() {
    print_str("|% ; -1", "%< ; -1 >");
}

#[test]
fn test_print_resolve_piq() {
    print_str("|@ abc ;", "@< abc ; >");
}

#[test]
fn test_print_block() {
    print_str(r"|\ |% ; -1 [|# abc 123 |@ abc ;]", r"\< %< ; -1 > :< #< abc 123 > :< @< abc ; > ; > > >");
}

#[test]
#[ignore]
fn test_print_evaled_empty_ast() {
    let empty_ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut evaluator = Evaluator::new(empty_ast);
    let evaled_ast = evaluator.eval().unwrap();
    let printer = Printer::new(evaled_ast);
    assert_eq!(printer.print(), "");
}

#[test]
fn test_print_evaled_symbol_ast() {
    print_evaled_str("abc", "abc");
}

#[test]
fn test_print_evaled_number_ast() {
    print_evaled_str("123", "123");
}

#[test]
fn test_print_evaled_define_ast() {
    // define symbol is number
    print_evaled_str("|# abc 123", ";");
}

pub fn print_str(left: &str, right: &str) {
    let mut iter = left.bytes();
    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new(&mut iter, scanners);

    let empty_ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut parser = Parser::new(lexer, empty_ast);
    let parsed_ast = parser.parse();

    let printer = Printer::new(parsed_ast);

    assert_eq!(printer.print(), right);
}

fn print_evaled_str(left: &str, right: &str) {
    let mut iter = left.bytes();
    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new(&mut iter, scanners);

    let empty_ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut parser = Parser::new(lexer, empty_ast);
    let parsed_ast = parser.parse();

    let mut evaluator = Evaluator::new(parsed_ast);
    let evaled_ast = evaluator.eval().unwrap();

    let printer = Printer::new(evaled_ast);

    assert_eq!(printer.print(), right);
}

/*
use super::core::{Epiq, Heliqs};

pub struct Printer<'a> {
    pub vm: &'a Heliqs,
}

impl<'a> Printer<'a> {
    pub fn print_aexp(&self, i: usize) -> String {
        if let Some(c) = self.vm.vctr.get(i) {
            match *c {
                Epiq::Name(ref n) => n.to_string(),
                Epiq::Tpiq { ref o, p, q } => {
                    format!(" {}<{} {}>", o, self.print_aexp(p), self.print_aexp(q))
                }
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }
}
*/

/*
fn print_affx(&self, i: usize) -> String {
    let result = "".to_string();

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
            &Epiq::Aexp { a:_, e } => {
                match self.vm.vctr.get(e) {
                    Some(&Epiq::Unit) => return true,
                    _ => return false,
                }
            },
            &Epiq::Unit => return true, // もはやここは通らないと思うが、一旦残しておく
            &Epiq::Lpiq { p:_, q } => {
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
        Some(&Epiq::Aexp { a:_, e }) => {
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
*/

/*
fn aexp_e<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
match t.0.vm.vctr.get(t.1) {
    Some(&Epiq::Aexp { a:_, e }) => Some((t.0, e)),
    _ => None,
}
}

fn apiq_f<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
match t.0.vm.vctr.get(t.1) {
    Some(&Epiq::Apiq { p, q:_ }) => Some((t.0, p)),
    _ => None,
}
}

fn apiq_q<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
match t.0.vm.vctr.get(t.1) {
    Some(&Epiq::Apiq { p:_, q }) => Some((t.0, q)),
    _ => None,
}
}

fn fpiq_p<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
match t.0.vm.vctr.get(t.1) {
    Some(&Epiq::Fpiq { p, q:_ }) => Some((t.0, p)),
    Some(a) => {
        println!("not fpiq: {:?}", a);
        None
    },
    _ => None,
}
}
*/
/*
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
*/
/*
fn print_one_piq<'p, 'a>(t: (&'p Parser<'a>, usize)) -> Option<(&'p Parser<'a>, usize)> {
match t.0.vm.vctr.get(t.1) {
    Some(e) => {
        println!("print_one_piq: {:?}", e);
        Some(t)
    },
    _ => None,
}
}
*/
