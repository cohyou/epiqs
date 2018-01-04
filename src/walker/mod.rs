use std::rc::Rc;
use std::cell::RefCell;
use core::*;
use printer::*;

const DEGUGGING_NOW: bool = false;
const UNIT_INDX: usize = 5;

pub struct Walker {
    vm: Rc<RefCell<Heliqs>>,
}

impl Walker {
    pub fn new(vm: Rc<RefCell<Heliqs>>) -> Walker {
        Walker {
            vm: vm,
        }
    }

    pub fn walk(&self) {
        self.log("\n");

        if let Some((entry, eee)) = {
            let borrowed_vm = self.vm.borrow();
            if let Some(entry) = borrowed_vm.entry() {
                let Node(e0, e1) = self.get_epiq(entry);
                let eee = (entry, Node(e0, e1.clone()));
                Some(eee)
            } else {
                None
            }
        } {
            let walked = self.walk_internal(eee, 0);
            let result = walked.0;
            if result != entry {
                // なんらかの変化があったので反映する必要がある
                // ここだと、entrypointを変更する
                // // println!("borrow_mut: {:?}", 1);
                self.vm.borrow_mut().set_entry(result);
            }
        }
    }

    fn walk_internal<'a>(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        // println!("{:?}{}walk ＿開始＿: ", input, " ".repeat(lvl));

        let Node(_, piq) = input.clone();

        match *piq {
            Epiq::Eval(_, q) => {
                // ひとまずpは無視

                // そのまま返すとNG
                let q_node = self.get_epiq(q);

                let result = self.eval_internal(q_node, nest_level + 1);

                self.log_piq(nest_level, "walk: eval完前", q);
                self.log_piq(nest_level, "walk: eval完後", result.0);

                result
            },

            Epiq::Tpiq{ref o, p, q} => self.walk_piq(input, o, p, q, nest_level),
            Epiq::Lpiq(p, q) => self.walk_piq(input, ":", p, q, nest_level),
            Epiq::Appl(p, q) => self.walk_piq(input, "!", p, q, nest_level),
            Epiq::Rslv(p, q) => self.walk_piq(input, "@", p, q, nest_level),
            Epiq::Cond(p, q) => self.walk_piq(input, "?", p, q, nest_level),
            Epiq::Envn(p, q) => self.walk_piq(input, "%", p, q, nest_level),
            Epiq::Bind(p, q) => self.walk_piq(input, "#", p, q, nest_level),
            Epiq::Lmbd(p, q) => self.walk_piq(input, r"\", p, q, nest_level),

            _ => input,
        }
    }

    fn walk_piq(&self, input: Node<Rc<Epiq>>, o: &str, p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        // pとq両方をwalkしてみて、
        // 結果が両方とも変わらなければそのまま返す、
        // そうでなければ新しくpiqを作って返す

        let p_node = self.get_epiq(p);
        let q_node = self.get_epiq(q);

        let p_result = self.walk_internal(p_node, nest_level + 1);
        let new_p = p_result.0;

        let q_result = self.walk_internal(q_node, nest_level + 1);
        let new_q = q_result.0;

        if new_p == p && new_q == q {
            input
        } else {
            let new_epiq_index = {
                let new_epiq = match o {
                    ":" => Epiq::Lpiq(new_p, new_q),
                    "!" => Epiq::Appl(new_p, new_q),
                    "@" => Epiq::Rslv(new_p, new_q),
                    "?" => Epiq::Cond(new_p, new_q),
                    "%" => Epiq::Envn(new_p, new_q),
                    "#" => Epiq::Bind(new_p, new_q),
                    r"\" => Epiq::Lmbd(new_p, new_q),
                    _   => Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q},
                };
                let mut borrow_mut_vm = self.vm.borrow_mut();
                borrow_mut_vm.alloc(new_epiq)
            };
            let new_epiq_node = self.vm.borrow().get_epiq(new_epiq_index).clone();
            self.log_piq(nest_level, "new_epiq_node: ", new_epiq_index);
            new_epiq_node
        }
    }

    fn eval_internal<'a>(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {

        // if nest_level == 30 {
        //     println!("{:?}", "evalがたくさん回ったのでstack overflow");
        //     return Box::new(Node(0, Epiq::Unit));
        // }

        let lvl = (nest_level * 2) as usize;
        // println!("{:?}{}eval ＿開始＿: ", input, " ".repeat(lvl));
        let Node(input_index, piq) = input.clone();

        match *piq {
            Epiq::Unit | Epiq::Tval | Epiq::Fval |
            Epiq::Uit8(_) | Epiq::Name(_) | Epiq::Text(_) => input,

            // eval
            // もしかしてこっちはあまり通らないかもしれない
            Epiq::Eval(_, q) => {
                // ひとまずpは無視

                // そのまま返すとNG
                let q_node = self.get_epiq(q);

                let result = self.eval_internal(q_node, nest_level + 1);
                // println!("{}eval: origin: {:?} result: {:?}", " ".repeat(lvl), q, result);

                result
            },

            // consは何もしない
            Epiq::Lpiq(_, _) => input,

            // apply
            Epiq::Appl(p, q) => {
                // p: lambda q:arguments
                // println!("apply: {:?}", "start!!");

                let lambda_node = self.get_epiq(p);

                // println!("apply: lambda_node: {:?}", lambda_node);


                let walked_lambda_box = self.walk_internal(lambda_node, nest_level + 1);
                let walked_lambda_piq = walked_lambda_box.1;

                let args_node = self.get_epiq(q);

                let args = self.walk_internal(args_node, nest_level + 1);

                self.log_piq(nest_level, "args: ", args.0);

                match *walked_lambda_piq {
                    Epiq::Lmbd(lambda_env, lambda_body) => {
                        self.eval_lambda(input, lambda_env, lambda_body, args, nest_level)
                    },

                    Epiq::Prim(ref n) => self.eval_primitive(input, args, n),

                    _ => {
                        self.log("関数部分がlambdaでもprimでもないのでエラー");
                        input
                    },
                }
            },

            // resolve
            Epiq::Rslv(_, q) => {
                // p: 用途未定。ひとまず無視
                // q: シンボルというか名前
                let node = self.get_epiq(q);

                let result = self.walk_internal(node, nest_level + 1);
                let q_name = result.1;

                if let Epiq::Name(ref n) = *q_name {
                    let vm = self.vm.borrow();
                    match vm.resolve(n) {
                        Some(Some(res)) => res.clone().clone(),
                        _ => {
                            self.log(&format!("resolve時に指定されたキーが見つからない: {:?}", n));
                            input
                        },
                    }
                } else {
                    // println!("resolve時のキーがNameじゃないのでエラー");
                    input
                }
            },

            // condition
            Epiq::Cond(p, q) => self.eval_condition(input, input_index, "?", p, q, nest_level),

            // environment
            Epiq::Envn(_, _) => {
                // ひとまずNoneを返しておく
                // 本来は中身もwalkしてから返すべき？
                input
            }

            // bind
            Epiq::Bind(p, q) => {
                let result;
                if let Some((n, walked_q_val)) = {

                    let p_val = self.get_epiq(p);

                    let walked_p_val = self.walk_internal(p_val, nest_level + 1);
                    if let Epiq::Name(ref n) = *walked_p_val.1 {

                        let q_val = self.get_epiq(q);

                        let result = self.walk_internal(q_val, nest_level + 1);
                        let walked_q_val = result.0;
                        Some((n.clone(), walked_q_val.clone()))

                    } else {
                        None
                    }
                } {
                    // println!("#.p is Name");
                    result = {
                        // println!("borrow_mut: {:?}", 3);
                        self.vm.borrow_mut().define(n.as_ref(), walked_q_val);
                        // println!("borrow_mut: {:?}", 4);
                        let new_index = UNIT_INDX; // self.vm.borrow_mut().alloc(Epiq::Unit);
                        self.get_epiq(new_index)
                    };

                    result
                } else {

                    // println!("#.p is not Name");
                    input
                }
            },

            // access
            Epiq::Accs(p, q) => self.eval_access(input, p, q, nest_level),

            // block
            Epiq::Lmbd(_, _) => {
                // TODO: 一つ目の環境の中身はひとまず無視する
                // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
                // ただ、そもそも、blockをevalしても、何も変化はないはず。
                input
            },

            // // primitive function
            // &Epiq::Prim(_) => {
            //     /*
            //     println!("primitive");
            //
            //     // まずは引き算
            //
            //     Result::MakeEpiq(Some(Epiq::Uit8(3)))
            //     */
            //     // と思ったけど、これはapplyから呼ばれるので、ここを通ることはなさそう
            //     println!("Primはapplyから呼ばれるので、ここを通ることはなさそう");
            //     // Result::MakeEpiq(None)
            //     input
            // },

            Epiq::Tpiq{ref o, p:_, q:_} => {
                match o.as_ref() as &str {
                    _ => input,
                }
            },

            Epiq::Mpiq{ref o, p: _p, q} => {
                match o.as_ref() {
                    ">" => {
                        // ^> リストのeval
                        // リストの要素それぞれをevalする
                        // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)

                        let eval_list_node = self.get_epiq(q);
                        let result = self.eval_list(eval_list_node, nest_level + 1);
                        // println!("eval_list result: {:?}", result);
                        result
                    },

                    _ => input,
                }
            },

            _ => input,
        }
    }

    fn eval_lambda(&self, input: Node<Rc<Epiq>>,
                          lambda_env: usize, lambda_body: usize, args: Node<Rc<Epiq>>,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
        // まず環境を取得
        self.log_piq(nest_level, "eval lambda: ", lambda_env);
        let env_node = self.get_epiq(lambda_env);
        let walked_env_box = self.walk_internal(env_node, nest_level + 1);
        let walked_env_piq = walked_env_box.1;

        if let Epiq::Envn(_, symbol_table) =*walked_env_piq {
            // pは無視
            // qはシンボルのリストになる
            let params = self.get_epiq(symbol_table);

            // 新しい環境フレームを作る
            // println!("borrow_mut: {:?}", 5);
            self.vm.borrow_mut().extend();

            // 束縛を追加する
            self.assign_arguments(params, args);

            // 2. p.q(関数本体)をそのまま返却する
            let lambda_body_node = self.get_epiq(lambda_body);

            // walkを挟んでから返す
            // TODO: walkにするとLambdaをそのまま返してしまうので、マクロのような扱いになる
            // 実行したければevalしてから返す、
            // しかしできればマクロ展開・関数適用を両方ともこの中でやってしまいたい。。。
            // 今のところはひとまず関数適用しておく（普通にevalを通す）
            // println!("apply: {:?}", "Lambdaの評価開始");
            let walked_lambda_body_box = self.eval_internal(lambda_body_node, nest_level + 1);

            // 環境フレームを削除する
            // println!("borrow_mut: {:?}", 6);
            self.vm.borrow_mut().pop();

            self.log_piq(nest_level, "apply 正常終了: ", walked_lambda_body_box.0);
            walked_lambda_body_box
        } else {
            self.log("apply env_piqがTpiqじゃないのでエラー");
            input
        }
    }

    fn eval_primitive(&self, input: Node<Rc<Epiq>>, args: Node<Rc<Epiq>>, n: &str) -> Node<Rc<Epiq>> {
        // println!("{:?}", "primitive");

        match n.as_ref() {
            "decr" => {
                // 面倒なので 1- を実装
                // 引数取得
                if let Epiq::Lpiq(p, _) = *args.1 {
                    if let Epiq::Uit8(n) = *self.get_epiq(p).1 {
                        // 1を引く
                        let new_index = self.vm.borrow_mut().alloc(Epiq::Uit8(n - 1));
                        self.get_epiq(new_index)
                    } else {
                        input // 中身が数値じゃなかった
                    }
                } else {
                    input // 引数がリストじゃなかった
                }
            },

            prim_name @ "ltoreq" |
            prim_name @ "eq" |
            prim_name @ "plus" |
            prim_name @ "minus" => {
                // <=を実装
                // 一つ目の引数
                if let Epiq::Lpiq(p1, q1) = *args.1 {
                    if let Epiq::Uit8(n1) = *self.get_epiq(p1).1 {
                        // 二つ目の引数
                        if let Epiq::Lpiq(p2, _) = *self.get_epiq(q1).1 {
                            if let Epiq::Uit8(n2) = *self.get_epiq(p2).1 {
                                let new_index;

                                let new_epiq = match prim_name {
                                    pred @ "ltoreq" | pred @ "eq" => {
                                        let boolean = match pred {
                                            "ltoreq" => n1 <= n2,
                                            "eq" => n1 == n2,
                                            _ => false,
                                        };
                                        if boolean { Epiq::Tval } else { Epiq::Fval }
                                    },

                                    "plus" => Epiq::Uit8(n1 + n2),
                                    "minus" => Epiq::Uit8(n1 - n2),
                                    _ => Epiq::Unit,
                                };

                                // Unitだけ最適化
                                if new_epiq == Epiq::Unit {
                                    self.get_epiq(UNIT_INDX)
                                } else {
                                    new_index = self.vm.borrow_mut().alloc(new_epiq);
                                    self.get_epiq(new_index)
                                }
                            } else {
                                self.log("primitive ltoreq 2つ目の引数の中身が数値じゃなかった");
                                input
                            }
                        } else {
                            self.log("primitive ltoreq 2つ目の引数がリストじゃなかった");
                            input
                        }
                    } else {
                        self.log("primitive ltoreq 1つ目の引数の中身が数値じゃなかった");
                        input
                    }
                } else {
                    self.log("primitive ltoreq 1つ目の引数がリストじゃなかった");
                    input
                }
            },

            _ => {
                // println!("Primitive関数名が想定外なのでエラー");
                input
            },
        }
    }

    fn eval_access(&self, input: Node<Rc<Epiq>>,
                          p: usize, q: usize,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        // println!("access");
        // p: レシーバ
        // q: アクセッサ
        let Node(_, p_reciever) = self.walk_internal(self.get_epiq(p), nest_level + 1);
        let Node(_, q_accessor) = self.walk_internal(self.get_epiq(q), nest_level + 1);

        // レシーバの種類によってできることが変わる
        match *p_reciever {
            Epiq::Lpiq(p, q) => {
                // Lpiqならば、pとqが使える、それ以外は無理
                match *q_accessor {
                    Epiq::Name(ref n) => {
                        match n.as_ref() {
                            "p" => {
                                let p_node = self.get_epiq(p);
                                self.walk_internal(p_node, nest_level + 1)
                            },
                            "q" => {
                                let q_node = self.get_epiq(q);
                                self.walk_internal(q_node, nest_level + 1)
                            },
                            _ => {
                                self.log("Lpiqならばpとq以外はエラー");
                                input
                            },
                        }
                    },

                    _ => {
                        self.log("アクセッサがNameではないのでエラー");
                        input
                    },
                }
            },
            _ => {
                self.log(&format!("レシーバは今のところLpiq以外にも構造体とかが増えるはずだが、これから{:?}", *p_reciever));
                input
            },
        }
    }

    fn eval_condition(&self, input: Node<Rc<Epiq>>, input_index: usize,
                             o: &str, p: usize, q: usize,
                             nest_level: u32) -> Node<Rc<Epiq>> {
        let lvl = (nest_level * 2) as usize;

        // p: ^T or ^F(他の値の評価はひとまず考えない)
        // q: Lpiq、^Tならpを返し、^Fならqを返す
        let p_condition = self.get_epiq(p);

        let Node(_, q_result) = self.get_epiq(q);

        // 条件節をwalk
        // println!("condition: {:?}", "条件節をwalk");
        let walked_condition_node = self.walk_internal(p_condition.clone(), nest_level + 1);

        // 値がwalk後に変化していたら付け替える
        if walked_condition_node.0 == p_condition.0 {
            let mut vm = self.vm.borrow_mut();
            let node_mut = vm.get_epiq_mut(input_index);
            node_mut.1 = Rc::new(Epiq::Tpiq{o:o.to_string(), p:walked_condition_node.0, q:q});
            println!("{:?} -> ({} {:?}){}condition eval後付け替え", *input.1, input_index, walked_condition_node.1, " ".repeat(lvl));
        }

        let walked_condition_piq = walked_condition_node.1;

        match *walked_condition_piq {
            ref v @ Epiq::Tval | ref v @ Epiq::Fval => {
                match *q_result {
                    Epiq::Lpiq(p, q) => {
                        match *v {
                            Epiq::Tval => {
                                let p_node = self.get_epiq(p);
                                self.walk_internal(p_node, nest_level + 1)
                            },

                            Epiq::Fval => {
                                let q_node = self.get_epiq(q);
                                self.walk_internal(q_node, nest_level + 1)
                            },
                            _ => {
                                // println!("condtion ^Tか^Fしか取れないが、事前に弾いているので、ここは通らないはず");
                                input
                            },
                        }
                    },

                    _ => {
                        // println!("result部分がLpiqじゃないのでエラー");
                        input
                    },
                }
            },

            _ => {
                // println!("condtion 評価結果は^Tか^Fだが{:?}なのでエラー", walked_condition_piq);
                input
            },
        }
    }

    fn eval_list(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        // println!("{}eval_list ＿開始＿ {:?}: ", " ".repeat(lvl), input);

        let Node(_, piq) = input.clone();

        match *piq {
            Epiq::Lpiq(p, q) => {
                let current_node = self.get_epiq(p);

                let evaled_current_node = self.eval_internal(current_node, nest_level + 1);

                self.log_piq(nest_level, "elst ＿完了前: ", p);
                self.log_piq(nest_level, "elst ＿完了後: ", evaled_current_node.0);

                let next = self.get_epiq(q);
                if let Epiq::Unit = *next.1 {
                    // リストの最後なので評価の結果を返す
                    evaled_current_node
                } else {
                    // 次の項目へ
                    let next_node = self.get_epiq(q);
                    self.eval_list(next_node, nest_level + 1)
                }
            },

            _ => input,
        }
    }

    fn assign_arguments(&self, parameters_node: Node<Rc<Epiq>>, arguments_node: Node<Rc<Epiq>>) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        // println!("assign_arguments: {:?}", "start!!");

        let next_params_node;
        let next_args_node;
        {

            let Node(_, params_piq) = parameters_node;
            let Node(_, args_piq) = arguments_node;

            let content;
            if let Some((cntt, next_args)) = match *args_piq {
                Epiq::Lpiq(p, q) => Some((p, q)),
                _ => { None },
            } {
                next_args_node = self.get_epiq(next_args);
                content = cntt;
            } else {
                /* 普通は通らない */
                // println!("assign arguments_piqがおかしい");
                return;
            }

            let Node(_, next_args_piq) = next_args_node.clone();
            let content_node = self.get_epiq(content);

            // println!("assign: {:?}", content_node);

            let next_params;
            let param;
            if let Epiq::Lpiq(p, q) = *params_piq {
                next_params = q;
                param = p;
            } else {
                /* 普通は通らない */
                // println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_node);
                return;
            }

            next_params_node = self.get_epiq(next_params);
            let Node(_, _next_params_piq) = next_params_node.clone();
            let Node(_, param_piq) = self.get_epiq(param);

            let mut symbol_string = "";
            if let Epiq::Name(ref s) = *param_piq {
                symbol_string = s;
            } else {
                // 文字列じゃない場合は初期値があるとか、
                // 他の可能性があるが今は実装しない
            }

            // println!("assign_arguments: {:?}", "define!!");
            // println!("borrow_mut: {:?}", 9);
            self.vm.borrow_mut().define(symbol_string, content_node.0);


            // paramsとargs、両方のリストを回していくが、
            // ループの基準となるのはargs。
            // paramsが途中でなくなっても知らん。
            if *next_args_piq == Epiq::Unit {
                // 最後なので終了
                // println!("assign終わりです");
                return;
            }
        }

        // 次にいく
        self.assign_arguments(next_params_node, next_args_node);
    }

    // walker内でEpiqを読み取り用で取得するためのヘルパー
    // cloneしているのは多分しかたない
    fn get_epiq(&self, i: usize) -> Node<Rc<Epiq>> {
        let vm = self.vm.borrow();
        let &Node(index, ref rc_epiq) = vm.get_epiq(i);
        Node(index, rc_epiq.clone())
    }

    fn log_piq(&self, lvl: u32, comment: &str, i: usize) {
        if DEGUGGING_NOW {
            println!("{}{} {:?}", " ".repeat((lvl * 2) as usize), comment, self.printer_printed(i));
        }
    }

    fn log(&self, s: &str) {
        if DEGUGGING_NOW {
            println!("{:?}", s);
        }
    }

    fn printer_printed(&self, i: NodeId) -> String {
        let printer = Printer::new(self.vm.clone());
        printer.print_aexp(i, 0)
    }
}
