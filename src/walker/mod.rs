// use std::cell::RefCell;
// use std::collections::HashMap;
use core::*;

struct SymbolTable<'a> {
    table: Vec<Vec<(String, Option<&'a Epiq>)>>,
    current_index: usize,
}

impl<'a> SymbolTable<'a> {
    fn define(&mut self, name: &str, value: &'a Epiq) {
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

    fn resolve(&self, name: &str) -> Option<Option<&Epiq>> {
        if let Some(&( _, Some(ref r) )) = self.table[self.current_index].iter().find(|&&(ref n, _)| n == name) {
            Some(Some(r))
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

pub struct Walker<'a> {
    ast : /*&'a RefCell<AbstractSyntaxTree>*/NodeArena<Epiq>,
    symbol_table: SymbolTable<'a>,
}

enum Result {
    MakeEpiq(Option<Epiq>),
    NewIndex(NodeId),
}

impl<'a> Walker<'a> {
    pub fn new(ast :/*&'a RefCell<AbstractSyntaxTree>*/NodeArena<Epiq>) -> Walker<'a> {
        Walker {
            ast: ast,
            symbol_table: SymbolTable {
                table: vec![vec![("decr".to_string(), Some(&Epiq::Prim("decr".to_string())))]],
                current_index: Default::default(),
            }
        }
    }

    pub fn walk(&mut self) -> Option</*&RefCell<AbstractSyntaxTree>*/NodeArena<Epiq>> {
        println!("\n");
        if let Some(index) = self.ast.entry() {
            let result = self.walk_internal(index, 0);
            // println!("max: {:?} index: {:?}", result, index);
            if result != index {
                // なんらかの変化があったので反映する必要がある
                // ここだと、entrypointを変更する
                self.ast.set_entry(result);
                /*
                let mut ast = self.ast.borrow_mut();
                (*ast).entrypoint = Some(result);
                */
            }
            Some(self.ast)
        } else {
            None
        }
    }

    fn walk_internal(&mut self, index: NodeId, nest_level: u32) -> NodeId {
        let lvl = (nest_level * 2) as usize;

        let res = {
            let &Node(_, ref piq) = self.ast.get(index);
            println!("{}walk ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                &Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ひとまずpは無視
                            let new_q = self.eval_internal(q, nest_level+1);
                            if new_q == q {
                                Result::MakeEpiq(None)
                            } else {
                                let &Node(_, ref new_q_piq) = self.ast.get(new_q);
                                Result::MakeEpiq(Some(new_q_piq.clone()))
                            }
                        },

                        _ => {

                            // その他のTpiqの場合は、pとq両方をwalkしてみて、
                            // 結果が両方とも変わらなければそのまま返す、
                            // そうでなければ新しくTpiqを作ってそのindexを返す
                            // println!("{}walk >以外 pに入ります", " ".repeat(lvl));
                            let new_p = self.walk_internal(p, nest_level+1);
                            // println!("{}walk >以外 qに入ります", " ".repeat(lvl));
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
                self.ast.alloc(new_epiq)
            },
            Result::MakeEpiq(None) => {
                // 変化なし
                let &Node(_, ref piq) = self.ast.get(index);
                println!("{}walk 変化なし: {:?}", " ".repeat(lvl), piq);
                index
            },
            Result::NewIndex(i) => {
                // let borrowed_ast = self.ast.borrow();
                let &Node(_, ref piq1) = self.ast.get(index);
                let &Node(_, ref piq2) = self.ast.get(i);
                println!("{}walk 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
                i
            },
        }
    }

    fn eval_internal(&mut self, index: NodeId, nest_level: u32) -> NodeId {
        let lvl = (nest_level * 2) as usize;

        let res = {
            let &Node(_, ref piq) = self.ast.get(index);
            println!("{}eval ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                &Epiq::Unit |
                &Epiq::Tval |
                &Epiq::Fval |
                &Epiq::Uit8(_) |
                &Epiq::Name(_) => Result::MakeEpiq(None),

                // primitive function
                &Epiq::Prim(_) => {
                    /*
                    println!("primitive");

                    // まずは引き算

                    Result::MakeEpiq(Some(Epiq::Uit8(3)))
                    */
                    // と思ったけど、これはapplyから呼ばれるので、ここを通ることはなさそう
                    println!("Primはapplyから呼ばれるので、ここを通ることはなさそう");
                    Result::MakeEpiq(None)
                },

                &Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        // bind
                        "#" => {
                            let p_name = self.ast.get(p);
                            if let &Node(_, Epiq::Name(ref n)) = p_name {
                                let &Node(_, ref q_val) = self.ast.get(q);
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

                            let lambda_piq = self.ast.get(p);
                            let &Node(_, ref args) = self.ast.get(q);

                            match lambda_piq {
                                &Node(_, Epiq::Tpiq{o:_, p:lambda_env, q:lambda_body}) => {
                                    // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
                                    // まず環境を取得
                                    let env_piq = self.ast.get(lambda_env);

                                    if let &Node(_, Epiq::Tpiq{o:ref otag, p:_, q:symbol_table}) = env_piq {
                                        if otag == "%" {
                                            // pは無視
                                            // qはシンボルのリストになる
                                            let &Node(_, ref params) = self.ast.get(symbol_table);

                                            // 新しい環境フレームを作る
                                            self.symbol_table.extend();

                                            // 束縛を追加する
                                            self.assign_arguments(params, args);

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
                                        println!("env_piqがTpiqじゃないのでエラー");
                                        Result::MakeEpiq(None)
                                    }
                                },

                                &Node(_, Epiq::Prim(ref n)) => {
                                    match n.as_ref() {
                                        "decr" => {
                                            // 面倒なので 1- を実装
                                            Result::MakeEpiq(Some(Epiq::Uit8(3)))
                                        },
                                        "ltoreq" => {
                                            // <=を実装
                                            Result::MakeEpiq(None)
                                        },
                                        _ => {
                                            println!("Primitive関数名が想定外なのでエラー");
                                            Result::MakeEpiq(None)
                                        }
                                    }
                                },

                                _ => {
                                    println!("関数部分がlambdaでもprimでもないのでエラー");
                                    Result::MakeEpiq(None)
                                },
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
                                let &Node(_, ref new_q_piq) = self.ast.get(new_q);
                                Result::MakeEpiq(Some(new_q_piq.clone()))
                            }
                        },

                        // resolve
                        "@" => {
                            // p: 用途未定。ひとまず無視
                            // q: シンボルというか名前
                            let &Node(_, ref q_name) = self.ast.get(q);

                            if let &Epiq::Name(ref n) = q_name {
                                match self.symbol_table.resolve(n) {
                                    Some(Some(ref res)) => Result::MakeEpiq(Some(res)),
                                    _ => {
                                        println!("resolve時に指定されたキーが見つからない: {:?}", n);
                                        Result::MakeEpiq(None)
                                    },
                                }
                            } else {
                                println!("resolve時のキーがNameじゃないのでエラー");
                                Result::MakeEpiq(None)
                            }
                        },

                        // access
                        "." => {
                            println!("access");
                            // p: レシーバ
                            // q: アクセッサ
                            let &Node(_, ref p_reciever) = self.ast.get(p);
                            let &Node(_, ref q_accessor) = self.ast.get(q);

                            // レシーバの種類によってできることが変わる
                            match p_reciever {
                                &Epiq::Tpiq{ref o, p, q} => {
                                    match o.as_ref() {
                                        ":" => {
                                            // Lpiqならば、pとqが使える、それ以外は無理
                                            match q_accessor {
                                                &Epiq::Name(ref n) => {
                                                    match n.as_ref() {
                                                        "p" => Result::NewIndex(p),
                                                        "q" => Result::NewIndex(q),
                                                        _ => {
                                                            /* Lpiqならばpとq以外はエラー */
                                                            println!("Lpiqならばpとq以外はエラー");
                                                            Result::MakeEpiq(None)
                                                        },
                                                    }
                                                },

                                                _ => {
                                                    /* アクセッサがNameではないのでエラー */
                                                    println!("アクセッサがNameではないのでエラー");
                                                    Result::MakeEpiq(None)
                                                },
                                            }
                                        },
                                        _ => {
                                            /* Lpiq以外はまだ定義されていないが、これから増える */
                                            println!("Lpiq以外はまだ定義されていないが、これから増える");
                                            Result::MakeEpiq(None)
                                        },
                                    }
                                },
                                _ => {
                                    /* レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから */
                                    println!("レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから");
                                    Result::MakeEpiq(None)
                                },
                            }
                        },

                        // condition
                        "?" => {
                            println!("condition");
                            // p: ^T or ^F(他の値の評価はひとまず考えない)
                            // q: Lpiq、^Tならpを返し、^Fならqを返す
                            let &Node(_, p_condition) = self.ast.get(p);
                            let &Node(_, q_result) = self.ast.get(q);

                            match p_condition {
                                Epiq::Tval | Epiq::Fval => {
                                    match q_result {
                                        Epiq::Tpiq{ref o,p,q} => {
                                            if o == ":" {
                                                match p_condition {
                                                    Epiq::Tval => Result::NewIndex(p),
                                                    Epiq::Fval => Result::NewIndex(q),
                                                    _ => {
                                                        println!("condtion部分は^Tか^Fしか取れないが、事前に弾いているので、ここは通らないはず");
                                                        Result::MakeEpiq(None)
                                                    },
                                                }
                                            } else {
                                                println!("result部分がLpiqじゃないのでエラー");
                                                Result::MakeEpiq(None)
                                            }
                                        },

                                        _ => {
                                            println!("result部分がTpiqじゃないのでエラー");
                                            Result::MakeEpiq(None)
                                        },
                                    }
                                },

                                _ => {
                                    println!("condtion部分は^Tか^Fしか取れないようにしたいのでエラー");
                                    Result::MakeEpiq(None)
                                },
                            }
                        },

                        _ => Result::MakeEpiq(None),
                    }
                },

                &Epiq::Mpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ^> リストのeval
                            // リストの要素それぞれをevalする
                            // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)
                            // Result::MakeEpiq(None)
                            let res = self.eval_list(q, nest_level+1);
                            // 戻り値のindexがすでに存在するなら何もしない
                            if res <= self.ast.max_id().unwrap() {
                                Result::NewIndex(res)
                            } else {
                                let &Node(_, res_piq) = self.ast.get(res);
                                Result::MakeEpiq(Some(res_piq))
                            }
                        },

                        // true
                        "T" => Result::MakeEpiq(Some(Epiq::Tval)),
                        // false
                        "F" => Result::MakeEpiq(Some(Epiq::Fval)),

                        _ => Result::MakeEpiq(None),
                    }
                },
            }
        };

        match res {
            Result::MakeEpiq(Some(new_epiq)) => {
                // まずpushだけ
                println!("{}eval 生み出す: {:?}", " ".repeat(lvl), new_epiq);
                self.ast.alloc(new_epiq)
            },
            Result::MakeEpiq(None) => {
                // 変化なし
                let piq = self.ast.get(index);
                println!("{}eval 変化なし: {:?}", " ".repeat(lvl), piq);
                index
            },
            Result::NewIndex(i) => {
                let &Node(_, piq1) = self.ast.get(index);
                let &Node(_, piq2) = self.ast.get(i);
                println!("{}eval 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
                i
            },
        }
    }

    fn eval_list(&mut self, index: NodeId, nest_level: u32) -> NodeId {
        let lvl = (nest_level * 2) as usize;

        if let Some(res_index) = {
            let &Node(_, piq) = self.ast.get(index);
            println!("{}eval_list ＿開始＿: {:?}", " ".repeat(lvl), piq);

            match piq {
                Epiq::Tpiq{ref o,p,q} => {
                    match o.as_ref() {
                        ":" => {
                            let &Node(_, q_piq) = self.ast.get(q);

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

    fn assign_arguments(&mut self, parameters_piq: &Epiq, arguments_piq: &Epiq) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        match arguments_piq {
            &Epiq::Tpiq{o: ref colon, p: content, q: next_args} => {
                if colon == ":" {
                    let &Node(_, ref next_args_piq) = self.ast.get(next_args.clone());
                    let &Node(_, ref content_piq) = self.ast.get(content.clone());

                    println!("assign: {:?}", content_piq);

                    match parameters_piq {
                        &Epiq::Tpiq{o: ref colon, p: param, q: next_params} => {
                            if colon == ":" {
                                let &Node(_, ref next_params_piq) = self.ast.get(next_params);
                                let &Node(_, ref param_piq) = self.ast.get(param);

                                if let &Epiq::Name(ref s) = param_piq {
                                    self.symbol_table.define(s, content_piq);

                                    // paramsとargs、両方のリストを回していくが、
                                    // ループの基準となるのはargs。
                                    // paramsが途中でなくなっても知らん。
                                    if next_args_piq == &Epiq::Unit {
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
fn new() {
    let ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut evaluator = Evaluator::new(ast);
    evaluator.walk();
}
