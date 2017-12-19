// use std::cell::RefCell;
// use std::collections::HashMap;
use core::*;

struct SymbolTable<'a> {
    table: Vec<Vec<(String, Option<&'a Node<Epiq>>)>>,
    current_index: usize,
}

impl<'a> SymbolTable<'a> {
    fn define(&mut self, name: &str, value: &'a Node<Epiq>) {
        if {
            if let Some(&(_, ref _r)) = self.table[self.current_index].iter().find(|&&(ref n, _)| n == name) {
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

    fn resolve(&self, name: &str) -> Option<Option<&Node<Epiq>>> {
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
    ast: &'a NodeArena<Epiq>,
    symbol_table: SymbolTable<'a>,
}

enum Result {
    MakeEpiq(Option<Epiq>),
    NewIndex(NodeId),
}

impl<'a> Walker<'a> {
    pub fn new(ast: &'a NodeArena<Epiq>) -> Walker<'a> {
        let new_index = ast.alloc(Epiq::Prim("decr".to_string()));

        Walker {
            ast: ast,
            symbol_table: SymbolTable {
                table: vec![vec![
                    ("decr".to_string(), Some(ast.get(new_index)))
                ]],
                current_index: Default::default(),
            }
        }
    }

    pub fn walk(&mut self) -> Option<&'a NodeArena<Epiq>> {
        println!("\n");

        if let Some(entry) = self.ast.entry() {
            let e = self.ast.get(entry);
            let &Node(result, _) = self.walk_internal(e, 0);

            if result != entry {
                // なんらかの変化があったので反映する必要がある
                // ここだと、entrypointを変更する
                self.ast.set_entry(result);
            }
            Some(self.ast)
        } else {
            None
        }
    }

    fn walk_internal(&mut self, input: &Node<Epiq>, nest_level: u32) -> &Node<Epiq> {
        let lvl = (nest_level * 2) as usize;
        println!("{}walk ＿開始＿: {:?}", " ".repeat(lvl), input);

        let res = {
            let &Node(input_index, ref piq) = input;
            match piq {
                &Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ひとまずpは無視
                            let q_node = self.ast.get(q);
                            self.eval_internal(q_node, nest_level + 1)
                            /*
                            if new_q == q {
                                Result::MakeEpiq(None)
                            } else {
                                let &Node(_, ref new_q_piq) = self.ast.get(new_q);
                                Result::MakeEpiq(Some(new_q_piq.clone()))
                            }
                            */
                        },

                        _ => {

                            // その他のTpiqの場合は、pとq両方をwalkしてみて、
                            // 結果が両方とも変わらなければそのまま返す、
                            // そうでなければ新しくTpiqを作ってそのindexを返す
                            // println!("{}walk >以外 pに入ります", " ".repeat(lvl));
                            let p_node = self.ast.get(p);
                            let q_node = self.ast.get(q);
                            let &Node(new_p, _) = self.walk_internal(p_node, nest_level + 1);
                            // println!("{}walk >以外 qに入ります", " ".repeat(lvl));
                            let &Node(new_q, _) = self.walk_internal(q_node, nest_level + 1);
                            if new_p != p || new_q != q {
                                let &Node(input_index, _) = input;
                                let &mut node_mut = self.ast.get_mut(input_index);
                                node_mut.1 = Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q};
                            }
                            let &Node(input_index, _) = input;
                            self.ast.get(input_index)
                            /*
                            if new_p == p && new_q == q {
                                // println!("{}pもqも同じなので変化なし", " ".repeat(lvl));
                                Result::MakeEpiq(None)
                            } else {
                                // 新しくTpiqを作成

                                Result::MakeEpiq(Some(Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q}))
                            }*/
                        },
                    }
                },
                _ => {
                    self.ast.get(input_index)
                },
            }
        };

        res
        /*
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
        */
    }

    fn eval_internal(&mut self, input: &Node<Epiq>, nest_level: u32) -> &Node<Epiq> {
        let lvl = (nest_level * 2) as usize;
        println!("{}eval ＿開始＿: {:?}", " ".repeat(lvl), input);

        let res = {
            let &Node(input_index, ref piq) = input;

            match piq {
                &Epiq::Unit | &Epiq::Tval | &Epiq::Fval |
                &Epiq::Uit8(_) | &Epiq::Name(_) => /*Result::MakeEpiq(None)*/self.ast.get(input_index),

                // primitive function
                &Epiq::Prim(_) => {
                    /*
                    println!("primitive");

                    // まずは引き算

                    Result::MakeEpiq(Some(Epiq::Uit8(3)))
                    */
                    // と思ったけど、これはapplyから呼ばれるので、ここを通ることはなさそう
                    println!("Primはapplyから呼ばれるので、ここを通ることはなさそう");
                    // Result::MakeEpiq(None)
                    self.ast.get(input_index)
                },

                &Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        // bind
                        "#" => {
                            let p_val = self.ast.get(p);
                            let walked_p_val = self.walk_internal(p_val, nest_level + 1);
                            if let &Node(_, Epiq::Name(ref n)) = walked_p_val {
                                let q_val = self.ast.get(q);
                                // let walked_q_val = self.walk_internal(q_val, nest_level + 1);
                                self.symbol_table.define(n, q_val);

                                let new_index = self.ast.alloc(Epiq::Unit);
                                self.ast.get(new_index)
                            } else {
                                println!("#.p is not Name");
                                self.ast.get(input_index)
                            }
                        },

                        // environment
                        "%" => {
                            // ひとまずNoneを返しておく
                            // 本来は中身もwalkしてから返すべき？
                            self.ast.get(input_index)
                        },

                        // block
                        r"\" => {
                            // TODO: 一つ目の環境の中身はひとまず無視する
                            // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
                            // ただ、そもそも、blockをevalしても、何も変化はないはず。
                            self.ast.get(input_index)
                        },

                        // apply
                        "!" => {
                            // p: lambda q:arguments

                            let lambda_node = self.ast.get(p);
                            let &Node(_, ref walked_lambda_piq) = self.walk_internal(lambda_node, nest_level + 1);

                            let args_node = self.ast.get(q);
                            let args = self.walk_internal(args_node, nest_level + 1);

                            match walked_lambda_piq {
                                &Epiq::Tpiq{o:_, p:lambda_env, q:lambda_body} => {
                                    // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
                                    // まず環境を取得
                                    let env_node = self.ast.get(lambda_env);
                                    let &Node(_, ref walked_env_piq) = self.walk_internal(env_node, nest_level + 1);

                                    if let &Epiq::Tpiq{o:ref otag, p:_, q:symbol_table} = walked_env_piq {
                                        if otag == "%" {
                                            // pは無視
                                            // qはシンボルのリストになる
                                            let params = self.ast.get(symbol_table);

                                            // 新しい環境フレームを作る
                                            self.symbol_table.extend();

                                            // 束縛を追加する
                                            self.assign_arguments(params, args);

                                            // 2. p.q(関数本体)をそのまま返却する
                                            // walkを挟んでから返す
                                            let lambda_body_node = self.ast.get(lambda_body);
                                            let walked_lambda_body_node = self.walk_internal(lambda_body_node, nest_level + 1);

                                            // 環境フレームを削除する
                                            self.symbol_table.pop();

                                            // Result::NewIndex(new_lambda_body)
                                            walked_lambda_body_node
                                        } else {
                                            println!("env_piqが環境じゃないのでエラー");
                                            self.ast.get(input_index)
                                        }
                                    } else {
                                        println!("env_piqがTpiqじゃないのでエラー");
                                        self.ast.get(input_index)
                                    }
                                },

                                &Epiq::Prim(ref n) => {
                                    match n.as_ref() {
                                        "decr" => {
                                            // 面倒なので 1- を実装
                                            // Result::MakeEpiq(Some(Epiq::Uit8(3)))
                                            let new_index = self.ast.alloc(Epiq::Uit8(3));
                                            self.ast.get(new_index)
                                        },
                                        "ltoreq" => {
                                            // <=を実装
                                            // Result::MakeEpiq(None)
                                            self.ast.get(input_index)
                                        },
                                        _ => {
                                            println!("Primitive関数名が想定外なのでエラー");
                                            // Result::MakeEpiq(None)
                                            self.ast.get(input_index)
                                        }
                                    }
                                },

                                _ => {
                                    println!("関数部分がlambdaでもprimでもないのでエラー");
                                    // Result::MakeEpiq(None)
                                    self.ast.get(input_index)
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
                            let eval_target_node = self.ast.get(q);
                            self.eval_internal(eval_target_node, nest_level + 1)
                            /*
                            if new_q == q {
                                Result::MakeEpiq(None)
                            } else {
                                let &Node(_, ref new_q_piq) = self.ast.get(new_q);
                                Result::MakeEpiq(Some(new_q_piq.clone()))
                            }
                            */
                        },

                        // resolve
                        "@" => {
                            // p: 用途未定。ひとまず無視
                            // q: シンボルというか名前
                            let &Node(_, ref q_name) = self.walk_internal(self.ast.get(q), nest_level + 1);

                            if let &Epiq::Name(ref n) = q_name {
                                match self.symbol_table.resolve(n) {
                                    Some(Some(ref res)) => /*Result::MakeEpiq(Some(res))*/res,
                                    _ => {
                                        println!("resolve時に指定されたキーが見つからない: {:?}", n);
                                        // Result::MakeEpiq(None)
                                        self.ast.get(input_index)
                                    },
                                }
                            } else {
                                println!("resolve時のキーがNameじゃないのでエラー");
                                // Result::MakeEpiq(None)
                                self.ast.get(input_index)
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
                                                        "p" => self.walk_internal(self.ast.get(p), nest_level + 1)
                                                            /*Result::NewIndex(p)*/,
                                                        "q" => self.walk_internal(self.ast.get(q), nest_level + 1),
                                                        _ => {
                                                            /* Lpiqならばpとq以外はエラー */
                                                            println!("Lpiqならばpとq以外はエラー");
                                                            // Result::MakeEpiq(None)
                                                            self.ast.get(input_index)
                                                        },
                                                    }
                                                },

                                                _ => {
                                                    /* アクセッサがNameではないのでエラー */
                                                    println!("アクセッサがNameではないのでエラー");
                                                    // Result::MakeEpiq(None)
                                                    self.ast.get(input_index)
                                                },
                                            }
                                        },
                                        _ => {
                                            /* Lpiq以外はまだ定義されていないが、これから増える */
                                            println!("Lpiq以外はまだ定義されていないが、これから増える");
                                            // Result::MakeEpiq(None)
                                            self.ast.get(input_index)
                                        },
                                    }
                                },
                                _ => {
                                    /* レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから */
                                    println!("レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから");
                                    // Result::MakeEpiq(None)
                                    self.ast.get(input_index)
                                },
                            }
                        },

                        // condition
                        "?" => {
                            println!("condition");
                            // p: ^T or ^F(他の値の評価はひとまず考えない)
                            // q: Lpiq、^Tならpを返し、^Fならqを返す
                            let &Node(_, ref p_condition) = self.ast.get(p);
                            let &Node(_, ref q_result) = self.ast.get(q);

                            match p_condition {
                                &Epiq::Tval | &Epiq::Fval => {
                                    match q_result {
                                        &Epiq::Tpiq{ref o, p, q} => {
                                            if o == ":" {
                                                match p_condition {
                                                    &Epiq::Tval => {
                                                        self.walk_internal(self.ast.get(p), nest_level + 1)
                                                    },
                                                    &Epiq::Fval => {
                                                        self.walk_internal(self.ast.get(q), nest_level + 1)
                                                    },
                                                    _ => {
                                                        println!("condtion部分は^Tか^Fしか取れないが、事前に弾いているので、ここは通らないはず");
                                                        // Result::MakeEpiq(None)
                                                        self.ast.get(input_index)
                                                    },
                                                }
                                            } else {
                                                println!("result部分がLpiqじゃないのでエラー");
                                                // Result::MakeEpiq(None)
                                                self.ast.get(input_index)
                                            }
                                        },

                                        _ => {
                                            println!("result部分がTpiqじゃないのでエラー");
                                            // Result::MakeEpiq(None)
                                            self.ast.get(input_index)
                                        },
                                    }
                                },

                                _ => {
                                    println!("condtion部分は^Tか^Fしか取れないようにしたいのでエラー");
                                    // Result::MakeEpiq(None)
                                    self.ast.get(input_index)
                                },
                            }
                        },

                        _ => self.ast.get(input_index)/*Result::MakeEpiq(None)*/,
                    }
                },

                &Epiq::Mpiq{ref o, p: _p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ^> リストのeval
                            // リストの要素それぞれをevalする
                            // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)
                            // Result::MakeEpiq(None)
                            let eval_list_node = self.ast.get(q);
                            self.eval_list(eval_list_node, nest_level + 1)

                            /*
                            // 戻り値のindexがすでに存在するなら何もしない
                            if res <= self.ast.max_id().unwrap() {
                                // Result::NewIndex(res)
                                result_node
                            } else {
                                let &Node(_, res_piq) = self.ast.get(res);
                                Result::MakeEpiq(Some(res_piq))
                            }
                            */
                        },

                        // true
                        "T" => /*Result::MakeEpiq(Some(Epiq::Tval))*/{
                            let new_index = self.ast.alloc(Epiq::Tval);
                            self.ast.get(new_index)
                        },
                        // false
                        "F" => {
                            let new_index = self.ast.alloc(Epiq::Fval);
                            self.ast.get(new_index)
                        },

                        _ => /*Result::MakeEpiq(None)*/self.ast.get(input_index),
                    }
                },
            }
        };

        res
        /*
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
        */
    }

    fn eval_list(&mut self, input: &Node<Epiq>, nest_level: u32) -> &Node<Epiq> {
        let lvl = (nest_level * 2) as usize;
        println!("{}eval_list ＿開始＿: {:?}", " ".repeat(lvl), input);

        let &Node(input_index, ref piq) = input;

        match piq {
            &Epiq::Tpiq{ref o, p, q} => {
                match o.as_ref() {
                    ":" => {
                        let current_node = self.ast.get(p);
                        let evaled_current_node = self.eval_internal(current_node, nest_level + 1);
                        if let &Node(_, Epiq::Unit) = self.ast.get(q) {
                            // リストの最後なので評価の結果を返す
                            evaled_current_node
                        } else {
                            // 次の項目へ
                            let next_node = self.ast.get(q);
                            self.eval_list(next_node, nest_level + 1)
                        }
                    },
                    _ => self.ast.get(input_index),
                }
            },
            _ => self.ast.get(input_index),
        }
        /*
        if let Some(res_index) = {

        } {
            res_index
        } else {
            index
        }
        */
    }

    fn assign_arguments(&mut self, parameters_node: &Node<Epiq>, arguments_node: &Node<Epiq>) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        let &Node(_, ref params_piq) = parameters_node;
        let &Node(_, ref args_piq) = arguments_node;
        match args_piq {
            &Epiq::Tpiq{o: ref colon, p: content, q: next_args} => {
                if colon == ":" {
                    let next_args_node = self.ast.get(next_args.clone());
                    let &Node(_, ref next_args_piq) = next_args_node;
                    let content_node = self.ast.get(content);

                    println!("assign: {:?}", content_node);

                    match params_piq {
                        &Epiq::Tpiq{o: ref colon, p: param, q: next_params} => {
                            if colon == ":" {
                                let next_params_node = self.ast.get(next_params);
                                let &Node(_, ref _next_params_piq) = next_params_node;
                                let &Node(_, ref param_piq) = self.ast.get(param);

                                if let &Epiq::Name(ref s) = param_piq {
                                    self.symbol_table.define(s, content_node);

                                    // paramsとargs、両方のリストを回していくが、
                                    // ループの基準となるのはargs。
                                    // paramsが途中でなくなっても知らん。
                                    if next_args_piq == &Epiq::Unit {
                                        // 最後なので終了
                                        println!("assign終わりです");
                                    } else {
                                        // 次にいく
                                        self.assign_arguments(next_params_node, next_args_node);
                                    }
                                } else {
                                    // 文字列じゃない場合は初期値があるとか、
                                    // 他の可能性があるが今は実装しない
                                }
                            } else {
                                println!("assign parameters_piqがおかしい :じゃないTpiq");
                            }
                        },
                        _ => {
                            /* 普通は通らない */
                            println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_node);
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
    let mut walker = Walker::new(ast);
    walker.walk();
}
