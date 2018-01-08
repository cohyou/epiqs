use core::*;
use walker::*;

macro_rules! unwrap_text {
    ($s:ident, $e:expr) => {{
        match *$e.1 {
            Epiq::Text(ref t) => t,
            _ => {
                let from = $s.printer_printed($e.0);
                panic!("{}からtextは取り出せません", from);
            },
        }
    }}
}

macro_rules! unwrap_nmbr {
    ($s:ident, $e:expr) => {{
        match *$e.1 {
            Epiq::Uit8(n) => n,
            _ => {
                let from = $s.printer_printed($e.0);
                panic!("{}からnmbrは取り出せません", from);
            },
        }
    }}
}

macro_rules! first_nmbr {
    ($s:ident, $e:expr) => {{
        let first = $s.first($e.clone());
        unwrap_nmbr!($s, first)
    }}
}

macro_rules! second_nmbr {
    ($s:ident, $e:expr) => {{
        let first = $s.second($e.clone());
        unwrap_nmbr!($s, first)
    }}
}

impl Walker {
    pub fn eq_nmbr(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let n1 = first_nmbr!(self, args);
        let n2 = second_nmbr!(self, args);

        let new_epiq = if n1 == n2 { Epiq::Tval } else { Epiq::Fval };

        alloc_node!(self, new_epiq)
    }

    pub fn eq_text(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let t1 = self.first(args.clone());
        let text1 = unwrap_text!(self, t1);

        let t2 = self.second(args.clone());
        let text2 = unwrap_text!(self, t2);

        let new_epiq = if text1 == text2 { Epiq::Tval } else { Epiq::Fval };
        alloc_node!(self, new_epiq)
    }

    pub fn eq_name(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let t1 = self.first(args.clone());
        let name1 = unwrap_name!(self, t1);

        let t2 = self.second(args.clone());
        let name2 = unwrap_name!(self, t2);

        let new_epiq = if name1 == name2 { Epiq::Tval } else { Epiq::Fval };
        alloc_node!(self, new_epiq)
    }

    pub fn print(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let t = self.first(args.clone());
        let text = unwrap_text!(self, t);

        print!("{}", text);

        self.get_epiq(UNIT_INDX)
    }

    pub fn decrement(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let n = first_nmbr!(self, args);
        alloc_node!(self, Epiq::Uit8(n - 1))
    }

    pub fn plus(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let n1 = first_nmbr!(self, args);
        let n2 = second_nmbr!(self, args);
        alloc_node!(self, Epiq::Uit8(n1 + n2))
    }

    pub fn minus(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let n1 = first_nmbr!(self, args);
        let n2 = second_nmbr!(self, args);
        alloc_node!(self, Epiq::Uit8(n1 - n2))
    }

    pub fn le_or_eq_nmbr(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let n1 = first_nmbr!(self, args);
        let n2 = second_nmbr!(self, args);

        let new_epiq = if n1 <= n2 { Epiq::Tval } else { Epiq::Fval };

        alloc_node!(self, new_epiq)
    }

    pub fn concat(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        self.concat_internal(args, "")
    }

    fn concat_internal(&self, args: Node<Rc<Epiq>>, accum: &str) -> Node<Rc<Epiq>> {
        let t = self.first(args.clone());
        let text = match *t.1 {
            Epiq::Text(ref tt) => tt.clone(),
            Epiq::Name(ref tt) => tt.clone(),
            Epiq::Uit8(n) => n.to_string(),
            _ => {
                let from = self.printer_printed(t.0);
                panic!("{}からtextは取り出せません", from);
            },
        };

        let accuming = accum.to_string() + &text;
        match *self.qval(args.clone()).1 {
            Epiq::Unit => alloc_node!(self, Epiq::Text(accuming)),
            _ => self.concat_internal(self.qval(args.clone()), &accuming),
        }
    }

    pub fn dbqt(&self, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        alloc_node!(self, Epiq::Text("\"".to_string()))
    }

    fn first(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        self.pval(piq.clone())
    }

    fn second(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let second_lpiq = self.qval(piq);
        self.pval(second_lpiq)
    }



}
