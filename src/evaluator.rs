use std::cell::RefCell;
use std::collections::HashMap;
use core::*;

struct SymbolTable {
    table: Vec<Vec<(String, Option<Epiq>)>>,
    current_index: usize,
}

impl SymbolTable {
    fn define(&mut self, name: &str, value: Epiq) {
        if {
            if let Some(&(_, ref r)) = self.table[self.current_index].iter().find(|&&(ref n, _)| n == name) {
                // すでに含まれていたら上書きしたいが、方法がわからないので何もせずにおく
                // *r = Some(value);
                false
            } else {
                true
            }
        } {
            self.table[self.current_index].push( (name.to_string(), Some(value)) );
        }
    }

    fn resolve(&self, name: &str) -> Option<Option<Epiq>> {
        if let Some(&(_, ref r)) = self.table[self.current_index].iter().find(|&&(ref n, _)| n == name) {
            Some(r.clone())
        } else {
            None
        }
    }

    fn extend(&mut self) {
        let new_frame = vec![];
        self.table.push(new_frame);
        self.current_index = self.table.len() - 1;
    }

    fn pop(&mut self) {
        let _ = self.table.pop();
        self.current_index = self.table.len() - 1;
    }
}

pub struct Evaluator<'a> {
    ast :&'a RefCell<AbstractSyntaxTree>,
    symbol_table: SymbolTable,
}

enum Result {
    MakeEpiq(Option<Epiq>),
    NewIndex(u32),
}

impl<'a> Evaluator<'a> {
    pub fn new(ast :&'a RefCell<AbstractSyntaxTree>) -> Evaluator {
        Evaluator {
            ast: ast,
            symbol_table: SymbolTable {
                table: vec![vec![]],
                current_index: Default::default(),
            }
        }
    }

    pub fn walk(&mut self) -> Option<&RefCell<AbstractSyntaxTree>> {
        println!("\n");
        let index;
        {
            let borrowed_ast = self.ast.borrow();
            index = borrowed_ast.entrypoint.unwrap();
        }
        let result = self.walk_internal(index, 0);
        // println!("max: {:?} index: {:?}", result, index);
        if result != index {
            // なんらかの変化があったので反映する必要がある
            // ここだと、entrypointを変更する
            let mut ast = self.ast.borrow_mut();
            (*ast).entrypoint = Some(result);
        }

        Some(self.ast)
    }

    fn walk_internal(&mut self, index: u32, nest_level: u32) -> u32 {
        let lvl = (nest_level * 2) as usize;

        // self.eval_internal(index)
        let res = {
            let piq = {
                let borrowed_ast = self.ast.borrow();
                borrowed_ast.get(index).clone()
            };
            println!("{}walk ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ひとまずpは無視
                            let new_q = self.eval_internal(q, nest_level+1);
                            if new_q == q {
                                Result::MakeEpiq(None)
                            } else {
                                let borrowed_ast = self.ast.borrow();
                                Result::MakeEpiq(Some(borrowed_ast.get(new_q).clone()))
                            }
                        },

                        _ => {

                            // その他のTpiqの場合は、pとq両方をwalkしてみて、
                            // 結果が両方とも変わらなければそのまま返す、
                            // そうでなければ新しくTpiqを作ってそのindexを返す
                            println!("{}walk >以外 pに入ります", " ".repeat(lvl));
                            let new_p = self.walk_internal(p, nest_level+1);
                            println!("{}walk >以外 qに入ります", " ".repeat(lvl));
                            let new_q = self.walk_internal(q, nest_level+1);
                            if new_p == p && new_q == q {
                                // println!("{}pもqも同じなので変化なし", " ".repeat(lvl));
                                Result::MakeEpiq(None)
                            } else {
                                // 新しくTpiqを作成

                                Result::MakeEpiq(Some(Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q}))
                            }
                        },
                    }
                },
                _ => Result::MakeEpiq(None),
            }
        };

        match res {
            Result::MakeEpiq(Some(new_epiq)) => {
                // まずpushだけ
                println!("{}walk 生み出す: {:?}", " ".repeat(lvl), new_epiq);
                self.ast.borrow_mut().push(new_epiq);
                self.ast.borrow().max_index.get()
            },
            Result::MakeEpiq(None) => {
                // 変化なし
                let borrowed_ast = self.ast.borrow();
                let piq = borrowed_ast.get(index);
                println!("{}walk 変化なし: {:?}", " ".repeat(lvl), piq);
                index
            },
            Result::NewIndex(i) => {
                let borrowed_ast = self.ast.borrow();
                let piq1 = borrowed_ast.get(index);
                let piq2 = borrowed_ast.get(i);
                println!("{}walk 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
                i
            },
        }
    }

    fn eval_internal(&mut self, index: u32, nest_level: u32) -> u32 {
        let lvl = (nest_level * 2) as usize;

        let res = {
            let piq = {
                let borrowed_ast = self.ast.borrow();
                borrowed_ast.get(index).clone()
            };
            println!("{}eval ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                Epiq::Unit | Epiq::Uit8(_) | Epiq::Name(_) => Result::MakeEpiq(None),
                Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        // bind
                        "#" => {
                            let p_name = {
                                let borrowed_ast = self.ast.borrow();
                                borrowed_ast.get(p).clone()
                            };
                            if let Epiq::Name(ref n) = p_name {
                                let q_val = {
                                    let borrowed_ast = self.ast.borrow();
                                    borrowed_ast.get(q).clone()
                                };
                                self.symbol_table.define(n, q_val);

                                Result::MakeEpiq(Some(Epiq::Unit))
                            } else {
                                Result::MakeEpiq(None)
                            }
                        },

                        // environment
                        "%" => {
                            // ひとまずNoneを返しておく
                            Result::MakeEpiq(None)
                        },

                        // block
                        r"\" => {
                            // TODO: 一つ目の環境の中身はひとまず無視する
                            // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
                            // ただ、そもそも、blockをevalしても、何も変化はないはず。
                            Result::MakeEpiq(None)
                        },

                        // apply
                        "!" => {
                            // p: lambda q:arguments

                            let lambda_piq = {
                                let borrowed_ast = self.ast.borrow();
                                borrowed_ast.get(p).clone()
                            };

                            let arguments_piq = {
                                let borrowed_ast = self.ast.borrow();
                                borrowed_ast.get(q).clone()
                            };

                            if let Epiq::Tpiq{o:_, p:lambda_env, q:lambda_body} = lambda_piq {
                                // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
                                // まず環境を取得
                                let env_piq = {
                                    let borrowed_ast = self.ast.borrow();
                                    borrowed_ast.get(lambda_env).clone()
                                };

                                if let Epiq::Tpiq{o:ref otag, p:_, q:symbol_table} = env_piq {
                                    if otag == "%" {
                                        // pは無視
                                        // qはシンボルのリストになる
                                        let parameters_piq = {
                                            let borrowed_ast = self.ast.borrow();
                                            borrowed_ast.get(symbol_table).clone()
                                        };

                                        // 新しい環境フレームを作る
                                        self.symbol_table.extend();

                                        // 束縛を追加する
                                        self.assign_arguments(parameters_piq, arguments_piq);


                                        // 2. p.q(関数本体)をそのまま返却する
                                        // walkを挟んでから返す
                                        let new_lambda_body = self.walk_internal(lambda_body, nest_level+1);

                                        // 環境フレームを削除する
                                        self.symbol_table.pop();

                                        Result::NewIndex(new_lambda_body)
                                    } else {
                                        println!("{:?}", 1);
                                        Result::MakeEpiq(None)
                                    }
                                } else {
                                    println!("{:?}", 2);
                                    Result::MakeEpiq(None)
                                }
                            } else {
                                println!("{:?}", 3);
                                Result::MakeEpiq(None)
                            }
                        },

                        // eval
                        ">" => {
                            println!("eval");
                            // p: 用途未定。新しく何かを限定したりとかかなあ。
                            //    アイディアは思いつくけど、
                            //    そもそもパーサを変えたりとか？
                            //    環境を変更したりとか？
                            //    継続っぽく使うとか？
                            // q: evalされる本体。
                            let new_q = self.eval_internal(q, nest_level+1);
                            if new_q == q {
                                Result::MakeEpiq(None)
                            } else {
                                let borrowed_ast = self.ast.borrow();
                                Result::MakeEpiq(Some(borrowed_ast.get(new_q).clone()))
                            }
                        },

                        // resolve
                        "@" => {
                            // p: 用途未定。ひとまず無視
                            // q: シンボルというか名前
                            let p_name = {
                                let borrowed_ast = self.ast.borrow();
                                borrowed_ast.get(q).clone()
                            };
                            if let Epiq::Name(ref n) = p_name {
                                match self.symbol_table.resolve(n) {
                                    Some(Some(ref res)) => Result::MakeEpiq(Some(res.clone())),
                                    _ => Result::MakeEpiq(None),
                                }
                            } else {
                                Result::MakeEpiq(None)
                            }
                        }

                        _ => Result::MakeEpiq(None),
                    }
                },

                Epiq::Mpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ^> リストのeval
                            // リストの要素それぞれをevalする
                            // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)
                            // Result::MakeEpiq(None)
                            let res = self.eval_list(q, nest_level+1);
                            // 戻り値のindexがすでに存在するなら何もしない
                            if res <= self.ast.borrow().max_index.get() {
                                Result::NewIndex(res)
                            } else {
                                let borrowed_ast = self.ast.borrow();
                                Result::MakeEpiq(Some(borrowed_ast.get(res).clone()))
                            }
                        },
                        _ => Result::MakeEpiq(None),
                    }
                },
            }
        };

        match res {
            Result::MakeEpiq(Some(new_epiq)) => {
                // まずpushだけ
                println!("{}eval 生み出す: {:?}", " ".repeat(lvl), new_epiq);
                self.ast.borrow_mut().push(new_epiq);
                self.ast.borrow().max_index.get()
            },
            Result::MakeEpiq(None) => {
                // 変化なし
                let borrowed_ast = self.ast.borrow();
                let piq = borrowed_ast.get(index);
                println!("{}eval 変化なし: {:?}", " ".repeat(lvl), piq);
                index
            },
            Result::NewIndex(i) => {
                let borrowed_ast = self.ast.borrow();
                let piq1 = borrowed_ast.get(index);
                let piq2 = borrowed_ast.get(i);
                println!("{}eval 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
                i
            },
        }
    }

    fn eval_list(&mut self, index: u32, nest_level: u32) -> u32 {
        let lvl = (nest_level * 2) as usize;

        if let Some(res_index) = {
            let piq = {
                let borrowed_ast = self.ast.borrow();
                borrowed_ast.get(index).clone()
            };
            println!("{}eval_list ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                Epiq::Tpiq{ref o,p,q} => {
                    match o.as_ref() {
                        ":" => {
                            let q_piq = {
                                let borrowed_ast = self.ast.borrow();
                                borrowed_ast.get(q).clone()
                            };

                            if q_piq == Epiq::Unit {
                                // リストの最後なので評価の結果を返す
                                Some(self.eval_internal(p, nest_level+1))
                            } else {
                                // 評価はするが返さない
                                let _p_ret = self.eval_internal(p, nest_level+1);
                                // 次の項目へ
                                Some(self.eval_list(q, nest_level+1))
                            }
                        },
                        _ => None,
                    }
                },
                _ => None,
            }
        } {
            res_index
        } else {
            index
        }
    }

    fn assign_arguments(&mut self, parameters_piq: Epiq, arguments_piq: Epiq) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        match arguments_piq {
            Epiq::Tpiq{o: ref colon, p: content, q: next_args} => {
                if colon == ":" {
                    let next_args_piq = {
                        let borrowed_ast = self.ast.borrow();
                        borrowed_ast.get(next_args).clone()
                    };

                    let content_piq = {
                        let borrowed_ast = self.ast.borrow();
                        borrowed_ast.get(content).clone()
                    };

                    { println!("assign: {:?}", content_piq); }

                    match parameters_piq {
                        Epiq::Tpiq{o: ref colon, p: param, q: next_params} => {
                            if colon == ":" {
                                let next_params_piq = {
                                    let borrowed_ast = self.ast.borrow();
                                    borrowed_ast.get(next_params).clone()
                                };

                                let param_piq = {
                                    let borrowed_ast = self.ast.borrow();
                                    borrowed_ast.get(param).clone()
                                };
                                if let Epiq::Name(ref s) = param_piq {
                                    self.symbol_table.define(s, content_piq);

                                    // paramsとargs、両方のリストを回していくが、
                                    // ループの基準となるのはargs。
                                    // paramsが途中でなくなっても知らん。
                                    if next_args_piq == Epiq::Unit {
                                        // 最後なので終了
                                        println!("assign終わりです");
                                    } else {
                                        // 次にいく
                                        self.assign_arguments(next_params_piq, next_args_piq);
                                    }
                                } else {
                                    // 文字列じゃない場合は初期値があるとか、
                                    // 他の可能性があるけど今は実装しない
                                }
                            } else {
                                println!("assign parameters_piqがおかしい :じゃないTpiq");
                            }
                        },
                        _ => {
                            /* 普通は通らない */
                            println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_piq);
                        },
                    }
                }
            },
            _ => {
                /* 普通は通らない */
                println!("assign arguments_piqがおかしい");
            },
        }
    }
}

#[test]
#[ignore]
fn test() {
    let ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut evaluator = Evaluator::new(ast);
    evaluator.walk();
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
