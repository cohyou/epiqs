use core::*;
use walker::*;

macro_rules! alloc_node {
    ($s:ident, $n:expr) => {{
        let new_index = $s.vm.borrow_mut().alloc($n);
        $s.get_epiq(new_index)
    }}
}

macro_rules! unwrap_text {
    ($s:ident, $e:expr) => {{
        match *$e.1 {
            Epiq::Text(ref t) => t,
            _ => {
                let from = $s.printer_printed($e.0);
                panic!("{:?}からtextは取り出せません", from);
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
                panic!("{:?}からnmbrは取り出せません", from);
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


    fn first(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        self.pval(piq.clone())
    }

    fn second(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let second_lpiq = self.qval(piq);
        self.pval(second_lpiq)
    }

    fn pval(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        if let Epiq::Lpiq(p, _) = *piq.1 {
            self.get_epiq(p)
        } else {
            let from = self.printer_printed(piq.0);
            panic!("{:?}からpvalは取り出せません", from);
        }
    }

    fn qval(&self, piq: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        if let Epiq::Lpiq(_, q) = *piq.1 {
            self.get_epiq(q)
        } else {
            let from = self.printer_printed(piq.0);
            panic!("{:?}からqvalは取り出せません", from);
        }
    }

}
