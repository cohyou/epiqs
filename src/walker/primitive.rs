use core::*;
use walker::*;

macro_rules! alloc_node {
    ($s:ident, $n:expr) => {{
        let new_index = $s.vm.borrow_mut().alloc($n);
        $s.get_epiq(new_index)
    }}
}

macro_rules! unwrap_text {
    ($s:ident, $e:expr) => (match *$e.1 {
        Epiq::Text(ref t) => t,
        _ => {
            let from = $s.printer_printed($e.0);
            panic!("{:?}からtextは取り出せません", from);
        },
    });
}

impl Walker {
    pub fn eq_text(&self, input: Node<Rc<Epiq>>, args: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        let first = self.pval(args.clone());
        let text1 = unwrap_text!(self, first);

        let second_lpiq = self.qval(args);
        let second = self.pval(second_lpiq);
        let text2 = unwrap_text!(self, second);

        let new_epiq = if text1 == text2 { Epiq::Tval } else { Epiq::Fval };
        alloc_node!(self, new_epiq)
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
