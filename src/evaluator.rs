/*
fn beta_reduct(&mut self, entry: usize) {
    /*
    // 仮引数に実引数を入れる
    let mut a = vec![1, 3, 5];
    a.remove(1);
    a.insert(1, 2);
    println!("{:?}", a);

    // confirm that target is "application-piq"
    match self.vm.vctr.get(entry) {
        Some(&Epiq::Apiq { p: p1, q: q1 }) => {
            /* 引数をまず、lambdaの中に入れる */

            // ApiqのpはFpiqである必要がある(=ラムダ適用の左側はラムダ式でなければならない)
            match self.vm.vctr.get(p1) {
                Some(&Epiq::Fpiq { p: p2, q: q2 }) => {

                    // FpiqのpはPrmtである必要がある(=ラムダ式のは引数のシンボルが入る)
                    match self.vm.vctr.get(p2) {
                        Some(&Epiq::Prmt(_)) => {
                        },
                        Some(p3) => println!("not anonymous parameter {:?}", p3),
                        None => println!("{:?}", "index error"),
                    }
                },
                Some(p2) => println!("not function epiq {:?}", p2),
                None => println!("{:?}", "index error"),
            }

            println!("v.len(): {:?} entry: {:?}", self.vm.vctr.len(), entry);
        },
        Some(p1) => println!("not application epiq {:?}", p1),
        None => println!("{:?}", "index error"),
    }
    */

    let mut is_ok: (usize, usize) = (0, 0);

    match apiq_f((self, entry))
        // .and_then(aexp_e).and_then(pprn)
        .and_then(aexp_e).and_then(fpiq_p)
        // .and_then(aexp_e).and_then(pprn)
        .and_then(aexp_e) {
            Some(t) => {
                println!("{:?}", t.1);
                match apiq_q((self, entry)) {
                    Some(t2) => {
                        is_ok = (t.1, t2.1);
                    }
                    _ => {},
                }
            }
            None => {},
    }

    if is_ok != (0, 0) {
        self.vm.vctr.remove(is_ok.0);
        self.vm.vctr.insert(is_ok.0, Epiq::Prmt(is_ok.1));
    }
}
*/
