use std::cell::RefCell;
// use std::ops::Deref;
use core::*;
use lexer::*;
use parser::*;
use walker::*;

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
            Epiq::Unit => ";".to_string(),
            Epiq::Tval => "^T".to_string(),
            Epiq::Fval => "^F".to_string(),
            Epiq::Name(ref n) => n.to_string(),
            Epiq::Uit8(ref n) => format!("{}", n),
            Epiq::Prim(ref n) => format!("Prim({})", n),
            Epiq::Tpiq { ref o, p, q } => {
                format!("{}({} {})", o, self.print_aexp(p, nest_level + 1), self.print_aexp(q, nest_level + 1))
            },
            Epiq::Mpiq { ref o, p, q } => {
                format!("{}({} {})", o, self.print_aexp(p, nest_level + 1), self.print_aexp(q, nest_level + 1))
            },
            // _ => "".to_string(),
        }
    }
}

#[test]
#[ignore]
fn symbol() {
    print_str("abc", "abc");
}

#[test]
#[ignore]
fn number() {
    print_str("123", "123");
}

#[test]
fn unit() {
    print_str(";", ";");
}

#[test]
#[ignore]
fn tpiq() {
    // print_str("|: abc 123", ":< abc 123 >");
    print_str("|# abc 123", "#< abc 123 >");
}

#[test]
fn nested_tpiq() {
    print_str("|: |: cde |: abc 123 456", ":(:(cde :(abc 123)) 456)");
}

#[test]
fn list() {
    print_str("[abc 123]", ":(abc :(123 ;))");
}

#[test]
fn empty_env() {
    print_str("|% ; -1", "%(; -1)");
}

#[test]
fn resolve_piq() {
    print_str("|@ abc ;", "@(abc ;)");
}

#[test]
fn block() {
    print_str(
        r"|> ; |! |\ |% ; ; ^> -1 [|# abc 123 |@ ; abc] ;",
        r">(; !(\(%(; ;) >(-1 :(#(abc 123) :(@(; abc) ;)))) ;))"
    );
}

#[test]
#[ignore]
fn evaled_empty_ast() {
    let empty_ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut evaluator = Evaluator::new(empty_ast);
    let evaled_ast = evaluator.walk().unwrap();
    let printer = Printer::new(evaled_ast);
    assert_eq!(printer.print(), "");
}

#[test]
fn evaled_symbol_ast() {
    print_evaled_str("abc", "abc");
}

#[test]
fn evaled_number_ast() {
    print_evaled_str("123", "123");
}

#[test]
// #[ignore]
fn evaled_define_ast() {
    // define symbol is number
    print_evaled_str("|> ; |# abc 123", ";");
}

#[test]
fn evaled_apply() {
    print_evaled_str(r"|> ; |! |\ |% ; ; 0 ;", r"0");
}

#[test]
fn evaled_list() {
    print_evaled_str(r"|> ; ^> -1 [1 2 3]", r"3");
}

#[test]
// #[ignore]
fn evaled_defining_list() {
    // print_evaled_str(r"|> ; |# abc 123", r";");
    // print_evaled_str(r"|> ; |@ ; abc", r";");
    print_evaled_str(r"|> ; ^> -1 [|# abc 1234 |@ ; abc]", r"1234");
}

#[test]
fn exec_func() {
    // print_str(r"|% ; ;", ";a")
    // print_str(r"|> ; |! |\ |% ; ; 1 ;", ";a")
    print_evaled_str(r"|> ; |! |\ |% ; [a b c] |> ; ^> -1 [|@ ; c |@ ; b] [6667 6668 6669]", "6668")
}

#[test]
fn access() {
    // print_str("|. a p", ".(a p)");
    // print_evaled_str("|> ; |. |: 1 3 p", "p")
    print_evaled_str("|> ; |. |: 1 3 q", "3")
}

#[test]
fn condition() {
    // print_str("|? abc 123", "?(abc 123)");
    // print_str("^T", "^T");
    // print_str("^F", "^F");
    print_evaled_str("|> ; |? ^T |: 1 0", "1");
    print_evaled_str("|> ; |? ^F |: 1 0", "0");
}

#[test]
fn primitive_function() {
    print_evaled_str("|> ; |@ ; decr", "Prim(decr)")
    // print_evaled_str("|> ; |! |> ; |@ ; decr [4]", ";")
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
    let evaled_ast = evaluator.walk().unwrap();

    let printer = Printer::new(evaled_ast);

    assert_eq!(printer.print(), right);
}

/*
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
