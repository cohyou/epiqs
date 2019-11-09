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
