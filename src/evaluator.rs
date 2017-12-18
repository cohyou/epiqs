use std::cell::RefCell;
use std::collections::HashMap;
use core::*;

struct SymbolTable {
    pub table: HashMap<String, Option<Epiq>>,
}

pub struct Evaluator<'a> {
    ast :&'a RefCell<AbstractSyntaxTree>,
    symbol_table: SymbolTable,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast :&'a RefCell<AbstractSyntaxTree>) -> Evaluator {
        Evaluator{ ast: ast, symbol_table: SymbolTable{ table: HashMap::new() } }
    }

    pub fn eval(&mut self) -> Option<&RefCell<AbstractSyntaxTree>> {
        let index;
        {
            let borrowed_ast = self.ast.borrow();
            index = borrowed_ast.entrypoint.unwrap();
        }
        let result = self.eval_internal(index);
        println!("max: {:?} index: {:?}", result, index);
        if result != index {
            // なんらかの変化があったので反映する必要がある
            // ここだと、entrypointを変更する
            let mut ast = self.ast.borrow_mut();
            (*ast).entrypoint = Some(result);
        }

        Some(self.ast)
    }

    fn eval_internal(&mut self, index: u32) -> u32 {
        if let Some(new_epiq) = {
            let borrowed_ast = self.ast.borrow();
            let piq = borrowed_ast.get(index);

            match piq {
                &Epiq::Unit | &Epiq::Uit8(_) | &Epiq::Name(_) => None,
                &Epiq::Tpiq{ref o,p:_p,q:_q} => {
                    match o.as_ref() {
                        "#" => {
                            // bind
                            // TODO: 実際のbindではなく、適当に固定値をbindしている
                            self.symbol_table.table.insert("abc".to_string(), Some(Epiq::Uit8(123)));

                            Some(Epiq::Unit)
                        },

                        // environment
                        "%" => {
                            Some(Tpiq)
                        },

                        // block
                        r"\" => {
                            // TODO: 一つ目の環境の中身も、返す-1もひとまず無視する
                            // qのリストだけを逐次実行して、勝手に最後の値を返却するようにする
                            // ただ、そもそも、blockを評価しても実行されるわけではなく、
                            // 実行形式になるだけだ。
                            None
                        }
                        _ => None,
                    }
                },
            }
        } {
            // まずpushだけ
            self.ast.borrow_mut().push(new_epiq);
            self.ast.borrow().max_index.get()
        } else {
            index
        }
    }
}

#[test]
#[ignore]
fn test() {
    let ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut evaluator = Evaluator::new(ast);
    evaluator.eval();
}

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
