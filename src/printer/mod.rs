mod test;

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, BufReader};

use core::*;
use lexer::*;
use parser::*;
use walker::*;

pub struct Printer {
    vm: Rc<RefCell<Heliqs>>,
}

impl Printer {
    pub fn new(vm: Rc<RefCell<Heliqs>>) -> Self {
        Printer{ vm: vm }
    }

    pub fn print(&self) -> String {
        if let Some(entrypoint) = self.vm.borrow().entry() {
            self.print_aexp(entrypoint, 0)
        } else {
            "".to_string()
        }
    }

    pub fn print_aexp(&self, i: NodeId, nest_level: u32) -> String {
        let vm = self.vm.borrow();
        let &Node(_, ref epiq) = vm.get_epiq(i);
        match **epiq {
            Epiq::Unit => ";".to_string(),
            Epiq::Tval => "^T".to_string(),
            Epiq::Fval => "^F".to_string(),
            Epiq::Name(ref n) => n.to_string(),
            Epiq::Uit8(ref n) => format!("{}", n),
            Epiq::Text(ref n) => format!("\"{}\"", n),
            Epiq::Prim(ref n) => format!("Prim({})", n),
            Epiq::Tpiq { ref o, p, q } => self.print_piq(o, p, q, nest_level),
            Epiq::Mpiq { ref o, p, q } => self.print_piq(&format!("^{}", o), p, q, nest_level),
            Epiq::Eval(p, q) => self.print_piq(">", p, q, nest_level),
            Epiq::Quot(p, q) => self.print_piq("|", p, q, nest_level),
            Epiq::Lpiq(p, q) => self.print_piq(":", p, q, nest_level),
            Epiq::Appl(p, q) => self.print_piq("!", p, q, nest_level),
            Epiq::Rslv(p, q) => self.print_piq("@", p, q, nest_level),
            Epiq::Cond(p, q) => self.print_piq("?", p, q, nest_level),
            Epiq::Envn(p, q) => self.print_piq("%", p, q, nest_level),
            Epiq::Bind(p, q) => self.print_piq("#", p, q, nest_level),
            Epiq::Accs(p, q) => self.print_piq(".", p, q, nest_level),
            Epiq::Lmbd(p, q) => self.print_piq(r"\", p, q, nest_level),

            // _ => "".to_string(),
        }
    }

    fn print_piq(&self, otag: &str, p: NodeId, q:NodeId, nest_level: u32) -> String {
        format!("{}({} {})", otag, self.print_aexp(p, nest_level + 1), self.print_aexp(q, nest_level + 1))
    }
}

pub fn print_str(left: &str, right: &str) {
    let mut iter = left.bytes();
    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new(&mut iter, scanners);

    let vm = Rc::new(RefCell::new(Heliqs::new()));
    let vm2 = vm.clone();
    let mut parser = Parser::new(lexer, vm);
    let _parsed_ast = parser.parse();

    let printer = Printer::new(vm2);

    assert_eq!(printer.print(), right);
}

pub fn evaled_str(left: &str) -> String {
    let mut iter = left.bytes();
    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new(&mut iter, scanners);

    evaled_lexer(lexer)
}

pub fn evaled_reader(file_name: &str) -> String {
    let file = File::open(file_name).unwrap();
    let mut bytes = BufReader::new(file).bytes();

    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new2(&mut bytes, scanners);

    evaled_lexer(lexer)
}

fn evaled_lexer<'a>(lexer :Lexer<'a, 'a>) -> String {
    let vm = Rc::new(RefCell::new(Heliqs::new()));
    let vm2 = vm.clone();
    let vm3 = vm.clone();
    let mut parser = Parser::new(lexer, vm);
    parser.parse();

    let walker = Walker::new(vm2);
    walker.walk();

    let printer = Printer::new(vm3);
    printer.print()
}

pub fn print_evaled_str(left: &str, right: &str) {
    let result = evaled_str(left);
    assert_eq!(result, right);
}

/// leftを評価したらちゃんとrightになるかどうかのテスト
pub fn print_syntax_sugar(left: &str, right: &str) {
    let result_left = evaled_str(left);
    let result_right = evaled_str(right);
    assert_eq!(result_left, result_right);
}

pub fn only_evaluate(s: &str) {
    let mut iter = s.bytes();
    let scanners: &Vec<&Scanner> = &all_scanners!();
    let lexer = Lexer::new(&mut iter, scanners);

    let vm = Rc::new(RefCell::new(Heliqs::new()));
    let vm2 = vm.clone();

    let mut parser = Parser::new(lexer, vm);
    parser.parse();

    let walker = Walker::new(vm2);
    walker.walk();
}
