use std::rc::Rc;
use std::cell::RefCell;
use core::*;

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
        // println!("\n");

        if let Some((entry, eee)) = {
            let borrowed_vm = self.vm.borrow();
            if let Some(entry) = borrowed_vm.entry() {
                let &Node(ref e0, ref e1) = borrowed_vm.get_epiq(entry);
                let eee = (entry, Node(e0.clone(), e1.clone()));
                Some(eee)
            } else {
                None
            }
        } {
            let walked = self.walk_internal(&eee, 0);
            let result = walked.0;
            if result != entry {
                // なんらかの変化があったので反映する必要がある
                // ここだと、entrypointを変更する
                // // println!("borrow_mut: {:?}", 1);
                self.vm.borrow_mut().set_entry(result);
            }
        }
    }

    fn walk_internal<'a>(&self, input: &'a Node<Epiq>, nest_level: u32) -> Box<Node<Epiq>> {

        // let lvl = (nest_level * 2) as usize;
        // println!("{:?}{}walk ＿開始＿: ", input, " ".repeat(lvl));

        let &Node(input_index, ref piq) = input;

        match piq {
            &Epiq::Eval(p, q) => {
                // ひとまずpは無視

                // そのまま返すとNG
                let q_node = self.get_epiq(q);

                let result = self.eval_internal(&q_node, nest_level + 1);
                // println!("{:?} => {:?}{}walk: eval完了", q_node, result, " ".repeat(lvl));

                result
            },

            &Epiq::Tpiq{ref o, p, q} => {
                match o.as_ref() as &str {
                    _ => {
                        // その他のTpiqの場合は、pとq両方をwalkしてみて、
                        // 結果が両方とも変わらなければそのまま返す、
                        // そうでなければ新しくTpiqを作って返す

                        let p_node = self.get_epiq(p);
                        let q_node = self.get_epiq(q);

                        let p_result = self.walk_internal(&p_node, nest_level + 1);
                        let new_p = p_result.0;

                        let q_result = self.walk_internal(&q_node, nest_level + 1);
                        let new_q = q_result.0;

                        if new_p == p && new_q == q {
                            Box::new(input.clone())
                        } else {
                            let new_epiq_index = {
                                let new_epiq = Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q};
                                let mut borrow_mut_vm = self.vm.borrow_mut();
                                borrow_mut_vm.alloc(new_epiq)
                            };
                            let new_epiq_node = self.vm.borrow().get_epiq(new_epiq_index).clone();
                            Box::new(new_epiq_node)
                        }
                    },
                }
            },

            _ => Box::new(input.clone()),
        }
    }

    fn eval_internal<'a>(&self, input: &'a Node<Epiq>, nest_level: u32) -> Box<Node<Epiq>> {
        /*
        if nest_level == 30 {
            println!("{:?}", "evalがたくさん回ったのでstack overflow");
            return Box::new(Node(0, Epiq::Unit));
        }*/

        let lvl = (nest_level * 2) as usize;
        // println!("{:?}{}eval ＿開始＿: ", input, " ".repeat(lvl));

        let &Node(input_index, ref piq) = input;

        match piq {
            &Epiq::Unit | &Epiq::Tval | &Epiq::Fval |
            &Epiq::Uit8(_) | &Epiq::Name(_) => Box::new(input.clone()),

            // eval
            // もしかしてこっちはあまり通らないかもしれない
            &Epiq::Eval(p, q) => {
                // ひとまずpは無視

                // そのまま返すとNG
                let q_node = self.get_epiq(q);

                let result = self.eval_internal(&q_node, nest_level + 1);
                // println!("{}eval: origin: {:?} result: {:?}", " ".repeat(lvl), q, result);

                result
            },

            // consは何もしない
            &Epiq::Lpiq(p, q) => Box::new(input.clone()),

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

            &Epiq::Tpiq{ref o, p, q} => {

                match o.as_ref() {
                    // bind
                    "#" => {
                        let result;
                        if let Some((n, walked_q_val)) = {

                            let p_val = self.get_epiq(p);

                            let walked_p_val = self.walk_internal(&p_val, nest_level + 1);
                            if let Epiq::Name(ref n) = walked_p_val.1 {

                                let q_val = self.get_epiq(q);

                                let result = self.walk_internal(&q_val, nest_level + 1);
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
                                let new_index = self.vm.borrow_mut().alloc(Epiq::Unit);
                                self.vm.borrow().get_epiq(new_index).clone()
                            };

                            Box::new(result)
                        } else {

                            // println!("#.p is not Name");
                            Box::new(input.clone())
                        }
                    },

                    // environment
                    "%" => {
                        // ひとまずNoneを返しておく
                        // 本来は中身もwalkしてから返すべき？
                        Box::new(input.clone())
                    },

                    // block
                    r"\" => {
                        // TODO: 一つ目の環境の中身はひとまず無視する
                        // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
                        // ただ、そもそも、blockをevalしても、何も変化はないはず。
                        Box::new(input.clone())
                    },

                    // apply
                    "!" => {
                        // p: lambda q:arguments
                        // println!("apply: {:?}", "start!!");

                        let lambda_node = self.get_epiq(p);

                        // println!("apply: lambda_node: {:?}", lambda_node);


                        let walked_lambda_box = self.walk_internal(&lambda_node, nest_level + 1);
                        let ref walked_lambda_piq = walked_lambda_box.1;

                        let args_node = self.get_epiq(q);

                        let args = self.walk_internal(&args_node, nest_level + 1);

                        match walked_lambda_piq {
                            &Epiq::Tpiq{o:_, p:lambda_env, q:lambda_body} => {
                                self.eval_lambda(input, lambda_env, lambda_body, args, nest_level)
                            },

                            &Epiq::Prim(ref n) => self.eval_primitive(input, args, n),

                            _ => {
                                // println!("関数部分がlambdaでもprimでもないのでエラー");
                                Box::new(input.clone())
                            },
                        }
                    },

                    // resolve
                    "@" => {
                        // p: 用途未定。ひとまず無視
                        // q: シンボルというか名前
                        let node = self.get_epiq(q);

                        let result = self.walk_internal(&node, nest_level + 1);
                        let ref q_name = result.1;

                        if let &Epiq::Name(ref n) = q_name {
                            let borrowed_vm = self.vm.borrow();
                            match borrowed_vm.resolve(n) {
                                Some(Some(ref res)) => Box::new(res.clone().clone()),
                                _ => {
                                    // println!("resolve時に指定されたキーが見つからない: {:?}", n);
                                    Box::new(input.clone())
                                },
                            }
                        } else {
                            // println!("resolve時のキーがNameじゃないのでエラー");
                            Box::new(input.clone())
                        }
                    },

                    // access
                    "." => self.eval_access(input, p, q, nest_level),

                    // condition
                    "?" => self.eval_condition(input, input_index, o, p, q, nest_level),

                    _ => Box::new(input.clone()),
                }
            },

            &Epiq::Mpiq{ref o, p: _p, q} => {
                match o.as_ref() {
                    ">" => {
                        // ^> リストのeval
                        // リストの要素それぞれをevalする
                        // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)

                        let eval_list_node = self.get_epiq(q);
                        let result = self.eval_list(&eval_list_node, nest_level + 1);
                        // println!("eval_list result: {:?}", result);
                        result
                    },

                    // true
                    "T" => {
                        // println!("true {:?}", "start");
                        // // println!("borrow_mut: {:?}", 7);
                        let new_index = self.vm.borrow_mut().alloc(Epiq::Tval);
                        Box::new(self.get_epiq(new_index))
                    },

                    // false
                    "F" => {
                        // println!("false {:?}", "start");
                        // println!("borrow_mut: {:?}", 8);
                        let new_index = self.vm.borrow_mut().alloc(Epiq::Fval);
                        Box::new(self.get_epiq(new_index))
                    },
                    _ => Box::new(input.clone()),
                }
            },

            _ => Box::new(input.clone()),
        }
    }

    fn eval_lambda(&self, input: &Node<Epiq>,
                          lambda_env: usize, lambda_body: usize, args: Box<Node<Epiq>>,
                          nest_level: u32) -> Box<Node<Epiq>> {
        // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
        // まず環境を取得
        // println!("apply: {:?}", "get env!!");
        let env_node = self.get_epiq(lambda_env);
        let walked_env_box = self.walk_internal(&env_node, nest_level + 1);
        let ref walked_env_piq = walked_env_box.1;

        if let &Epiq::Tpiq{o:ref otag, p:_, q:symbol_table} = walked_env_piq {
            if otag == "%" {
                // pは無視
                // qはシンボルのリストになる
                let params = self.get_epiq(symbol_table);

                // 新しい環境フレームを作る
                // println!("borrow_mut: {:?}", 5);
                self.vm.borrow_mut().extend();

                // 束縛を追加する
                self.assign_arguments(&params, &args);

                // 2. p.q(関数本体)をそのまま返却する
                let lambda_body_node = self.get_epiq(lambda_body);

                // walkを挟んでから返す
                // TODO: walkにするとLambdaをそのまま返してしまうので、マクロのような扱いになる
                // 実行したければevalしてから返す、
                // しかしできればマクロ展開・関数適用を両方ともこの中でやってしまいたい。。。
                // 今のところはひとまず関数適用しておく（普通にevalを通す）
                // println!("apply: {:?}", "Lambdaの評価開始");
                let walked_lambda_body_box = self.eval_internal(&lambda_body_node, nest_level + 1);

                // 環境フレームを削除する
                // println!("borrow_mut: {:?}", 6);
                self.vm.borrow_mut().pop();

                // println!("apply 正常終了, {:?}", walked_lambda_body_box);
                walked_lambda_body_box
            } else {
                // println!("apply env_piqが環境じゃないのでエラー");
                Box::new(input.clone())
            }
        } else {
            // println!("apply env_piqがTpiqじゃないのでエラー");
            Box::new(input.clone())
        }
    }

    fn eval_primitive(&self, input: &Node<Epiq>, args: Box<Node<Epiq>>, n: &str) -> Box<Node<Epiq>> {
        // println!("{:?}", "primitive");

        match n.as_ref() {
            "decr" => {
                // 面倒なので 1- を実装
                // 引数取得
                if let Some(Node(_, Epiq::Uit8(n))) = {
                    let piq = args.1;
                    if let Epiq::Tpiq{o,p,q} = piq {
                        Some(self.get_epiq(p))
                    } else {
                        None
                    }
                } {
                    // 1を引く
                    let new_index = self.vm.borrow_mut().alloc(Epiq::Uit8(n - 1));
                    Box::new(self.get_epiq(new_index))
                } else {
                    // 引数がリストじゃなかった
                    // 中身が数値じゃなかった
                    Box::new(input.clone())
                }
            },

            prim_name @ "ltoreq" |
            prim_name @ "eq" |
            prim_name @ "plus" |
            prim_name @ "minus" => {
                // <=を実装
                // 一つ目の引数
                if let Some( (Node(_, Epiq::Uit8(n1)), q) ) = {
                    let piq = args.1;
                    if let Epiq::Tpiq{o,p,q} = piq {
                        Some( (self.get_epiq(p), q) )
                    } else {
                        // println!("primitive ltoreq 1つ目の引数がリストじゃなかった");
                        None
                    }
                } {
                    // 二つ目の引数
                    if let Some(Node(_, Epiq::Uit8(n2))) = {
                        let node = self.get_epiq(q);
                        let piq = node.1;
                        if let Epiq::Tpiq{o:o2,p:p2,q:q2} = piq {
                            Some(self.get_epiq(p2))
                        } else {
                            // println!("primitive ltoreq 2つ目の引数がリストじゃなかった");
                            None
                        }
                    } {
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

                        new_index = self.vm.borrow_mut().alloc(new_epiq);
                        Box::new(self.vm.borrow().get_epiq(new_index).clone())
                    } else {
                        // println!("primitive ltoreq 2つ目の引数がリストじゃなかった or 中身が数値じゃなかった");
                        Box::new(input.clone())
                    }
                } else {
                    // println!("primitive ltoreq 1つ目の引数がリストじゃなかった or 中身が数値じゃなかった");
                    Box::new(input.clone())
                }
            },

            _ => {
                // println!("Primitive関数名が想定外なのでエラー");
                Box::new(input.clone())
            },
        }
    }

    fn eval_access(&self, input: &Node<Epiq>,
                          p: usize, q: usize,
                          nest_level: u32) -> Box<Node<Epiq>> {
        // println!("access");
        // p: レシーバ
        // q: アクセッサ
        let Node(_, ref p_reciever) = self.get_epiq(p);
        let Node(_, ref q_accessor) = self.get_epiq(q);

        // レシーバの種類によってできることが変わる
        match p_reciever {
            &Epiq::Lpiq(p, q) => {
                // Lpiqならば、pとqが使える、それ以外は無理
                match q_accessor {
                    &Epiq::Name(ref n) => {
                        match n.as_ref() {
                            "p" => {
                                let p_node = self.get_epiq(p);
                                self.walk_internal(&p_node, nest_level + 1)
                            },
                            "q" => {
                                let q_node = self.get_epiq(q);
                                self.walk_internal(&q_node, nest_level + 1)
                            },
                            _ => {
                                // println!("Lpiqならばpとq以外はエラー");
                                Box::new(input.clone())
                            },
                        }
                    },

                    _ => {
                        // println!("アクセッサがNameではないのでエラー");
                        Box::new(input.clone())
                    },
                }
            },
            _ => {
                // println!("レシーバは今のところLpiq以外にも構造体とかが増えるはずだが、これから");
                Box::new(input.clone())
            },
        }
    }

    fn eval_condition(&self, input: &Node<Epiq>, input_index: usize,
                             o: &str, p: usize, q: usize,
                             nest_level: u32) -> Box<Node<Epiq>> {
        // println!("condition");
        // p: ^T or ^F(他の値の評価はひとまず考えない)
        // q: Lpiq、^Tならpを返し、^Fならqを返す
        let p_condition = self.get_epiq(p);

        let Node(_, ref q_result) = self.get_epiq(q);

        // 条件節をwalk
        // println!("condition: {:?}", "条件節をwalk");
        let walked_condition_node = self.walk_internal(&p_condition, nest_level + 1);

        // 値がwalk後に変化していたら付け替える
        if walked_condition_node.0 == p_condition.0 {
            let mut vm = self.vm.borrow_mut();
            let node_mut = vm.get_epiq_mut(input_index);
            node_mut.1 = Epiq::Tpiq{o:o.to_string(), p:walked_condition_node.0, q:q};
            // println!("{:?} -> ({} {:?}){}condition eval後付け替え", input, input_index, walked_condition_node.1, " ".repeat(lvl));
        }

        let ref walked_condition_piq = walked_condition_node.1;

        match walked_condition_piq {
            v @ &Epiq::Tval | v @ &Epiq::Fval => {
                match q_result {
                    &Epiq::Lpiq(p, q) => {
                        match v {
                            &Epiq::Tval => {
                                let p_node = self.get_epiq(p);
                                self.walk_internal(&p_node, nest_level + 1)
                            },
                            &Epiq::Fval => {
                                let q_node = self.get_epiq(q);
                                self.walk_internal(&q_node, nest_level + 1)
                            },
                            _ => {
                                // println!("condtion ^Tか^Fしか取れないが、事前に弾いているので、ここは通らないはず");
                                Box::new(input.clone())
                            },
                        }
                    },

                    _ => {
                        // println!("result部分がLpiqじゃないのでエラー");
                        Box::new(input.clone())
                    },
                }
            },

            _ => {
                // println!("condtion 評価結果は^Tか^Fだが{:?}なのでエラー", walked_condition_piq);
                Box::new(input.clone())
            },
        }
    }

    fn eval_list(&self, input: &Node<Epiq>, nest_level: u32) -> Box<Node<Epiq>> {
        let lvl = (nest_level * 2) as usize;
        // println!("{:?}{}eval_list ＿開始＿: ", input, " ".repeat(lvl));

        let &Node(input_index, ref piq) = input;

        match piq {
            &Epiq::Tpiq{ref o, p, q} => {
                match o.as_ref() {
                    ":" => {
                        let current_node = self.get_epiq(p);

                        let evaled_current_node = self.eval_internal(&current_node, nest_level + 1);
                        let next = self.get_epiq(q);
                        if let Node(_, Epiq::Unit) = next {
                            // リストの最後なので評価の結果を返す
                            evaled_current_node
                        } else {
                            // 次の項目へ
                            let next_node = self.get_epiq(q);
                            self.eval_list(&next_node, nest_level + 1)
                        }
                    },
                    _ => Box::new(input.clone()),
                }
            },
            _ => Box::new(input.clone()),
        }
    }

    fn assign_arguments(&self, parameters_node: &Node<Epiq>, arguments_node: &Node<Epiq>) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        // println!("assign_arguments: {:?}", "start!!");


        let next_params_node;
        let next_args_node;
        {

            let &Node(_, ref params_piq) = parameters_node;
            let &Node(_, ref args_piq) = arguments_node;

            let content;
            if let Some((colon, cntt, next_args)) =
            match args_piq {
                &Epiq::Tpiq{o: ref colon, p: content, q: next_args} => {
                    Some((colon, content, next_args))
                },
                _ => { None },
            } {
                if colon != ":" { return; /* 普通は通らない */ }

                next_args_node = self.get_epiq(next_args);
                content = cntt;
            } else {
                /* 普通は通らない */
                // println!("assign arguments_piqがおかしい");
                return;
            }

            let Node(_, ref next_args_piq) = next_args_node;
            let content_node = self.get_epiq(content);

            // println!("assign: {:?}", content_node);

            let next_params;
            let param;
            if let Some((params_colon, p, q)) = match params_piq {
                &Epiq::Tpiq{o: ref colon, p: param, q: next_params} => {
                    Some((colon, param, next_params))
                },
                _ => { None },
            } {
                if params_colon != ":" {
                    /* 普通は通らない */
                    // println!("assign parameters_piqがおかしい :じゃないTpiq");
                    return;
                }

                next_params = q;
                param = p;
            } else {
                /* 普通は通らない */
                // println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_node);
                return;
            }

            next_params_node = self.get_epiq(next_params);
            let Node(_, ref _next_params_piq) = next_params_node;
            let Node(_, ref param_piq) = self.get_epiq(param);

            let mut symbol_string = "";
            if let &Epiq::Name(ref s) = param_piq {
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
            if next_args_piq == &Epiq::Unit {
                // 最後なので終了
                // println!("assign終わりです");
                return;
            }
        }

        // 次にいく
        self.assign_arguments(&next_params_node, &next_args_node);
    }

    // walker内でEpiqを読み取り用で取得するためのヘルパー
    // cloneしているのは多分しかたない
    fn get_epiq(&self, i: usize) -> Node<Epiq> {
        let borrowed_vm = self.vm.borrow();
        borrowed_vm.get_epiq(i).clone()
    }
}
