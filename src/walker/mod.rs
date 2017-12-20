use std::rc::Rc;
use std::cell::RefCell;

use std::error::Error;

use core::*;

pub struct Walker {
    vm: Rc<RefCell<Heliqs>>,
    /*
    ast: &'a NodeArena<Epiq>,
    symbol_table: SymbolTable<'a>,
    */
}

enum Result {
    MakeEpiq(Option<Epiq>),
    NewIndex(NodeId),
}

impl Walker {
    pub fn new(vm: Rc<RefCell<Heliqs>>) -> Walker {
        // let _new_index = ast.alloc(Epiq::Prim("decr".to_string()));

        Walker {
            vm: vm,
            /*
            ast: ast,
            symbol_table: SymbolTable::new(),
            */
        }
    }

    pub fn walk(&self)/* -> Option<&'a NodeArena<Epiq>>*/ {
        println!("\n");
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
            // match self.vm.try_borrow_mut() {
            //     Err(e) => {
            //         println!("NG: {:?}", e.description());
            //     },
            //     Ok(n) => { println!("OK:");}
            // }

            let walked = self.walk_internal(&eee, 0);
            let result = walked.0;
            // let &Node(result, _) = self.eval_internal(&eee, 0);


            if result != entry {
                // なんらかの変化があったので反映する必要がある
                // ここだと、entrypointを変更する
                self.vm.borrow_mut().set_entry(result);
            }
        }

        // Some(self.ast)
    }

    fn walk_internal<'a>(&self, input: &'a Node<Epiq>, nest_level: u32) -> Box<Node<Epiq>> {

        let lvl = (nest_level * 2) as usize;
        println!("{}walk ＿開始＿: {:?}", " ".repeat(lvl), input);


        /*let res = */{
            let &Node(input_index, ref piq) = input;
            match piq {
                &Epiq::Tpiq{ref o, p, q} => {
                    match o.as_ref() {
                        ">" => {
                            // ひとまずpは無視

                            // そのまま返すとNG
                            let q_node = { let borrowed_vm = self.vm.borrow();
                            borrowed_vm.get_epiq(q).clone()};
                            let result = self.eval_internal(&q_node, nest_level + 1);

                            // TODO: これ以下はeval_internalと重複しているのでまとめたい
                            // let &Node(new_q, ref new_q_piq) = result;
                            let new_q = result.0;
                            let ref new_q_piq = result.1;
                            if new_q != q {
                                let &Node(input_index, _) = input;
                                let mut borrow_mut_vm = self.vm.borrow_mut();
                                let mut node_mut = borrow_mut_vm.get_epiq_mut(input_index);
                                node_mut.1 = new_q_piq.clone();
                            }

                            Box::new(input.clone())
                        },

                        _ => {
                            // その他のTpiqの場合は、pとq両方をwalkしてみて、
                            // 結果が両方とも変わらなければそのまま返す、
                            // そうでなければ新しくTpiqを作ってそのindexを返す
                            // println!("{}walk >以外 pに入ります", " ".repeat(lvl));
                            let borrowed_vm = self.vm.borrow();
                            let p_node = borrowed_vm.get_epiq(p);
                            let q_node = borrowed_vm.get_epiq(q);
                            let p_result = self.walk_internal(p_node, nest_level + 1);
                            let new_p = p_result.0;
                            // println!("{}walk >以外 qに入ります", " ".repeat(lvl));
                            let q_result = self.walk_internal(q_node, nest_level + 1);
                            let new_q = q_result.0;

                            if new_p != p || new_q != q {
                                let &Node(input_index, _) = input;
                                let mut borrow_mut_vm = self.vm.borrow_mut();
                                let mut node_mut = borrow_mut_vm.get_epiq_mut(input_index);
                                node_mut.1 = Epiq::Tpiq{o: o.to_string(), p: new_p, q: new_q};
                            }

                            Box::new(input.clone())
                        },
                    }
                },

                _ => Box::new(input.clone()),
            }
        }//;

        // res
        /*
        match res {
            Result::MakeEpiq(Some(new_epiq)) => {
                // まずpushだけ
                println!("{}walk 生み出す: {:?}", " ".repeat(lvl), new_epiq);
                self.vm.borrow_mut().alloc(new_epiq)
            },
            Result::MakeEpiq(None) => {
                // 変化なし
                let &Node(_, ref piq) = self.vm.borrow().get_epiq(index);
                println!("{}walk 変化なし: {:?}", " ".repeat(lvl), piq);
                index
            },
            Result::NewIndex(i) => {
                // let borrowed_ast = self.ast.borrow();
                let &Node(_, ref piq1) = self.vm.borrow().get_epiq(index);
                let &Node(_, ref piq2) = self.vm.borrow().get_epiq(i);
                println!("{}walk 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
                i
            },
        }
        */
    }

    fn eval_internal<'a>(&self, input: &'a Node<Epiq>, nest_level: u32) -> Box<Node<Epiq>> {

        let lvl = (nest_level * 2) as usize;
        println!("{}eval ＿開始＿: {:?}", " ".repeat(lvl), input);

        /*
        let res = */{
            let &Node(input_index, ref piq) = input;

            match piq {
                &Epiq::Unit | &Epiq::Tval | &Epiq::Fval |
                &Epiq::Uit8(_) | &Epiq::Name(_) => Box::new(input.clone()),

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

                                let p_val = {let borrowed_vm = self.vm.borrow();
                                borrowed_vm.get_epiq(p).clone()};

                                let walked_p_val = self.walk_internal(&p_val, nest_level + 1);
                                if let Epiq::Name(ref n) = walked_p_val.1 {

                                    let q_val = {let borrowed_vm = self.vm.borrow();
                                        borrowed_vm.get_epiq(q).clone()};
                                    let result = self.walk_internal(&q_val, nest_level + 1);
                                    let walked_q_val = result.0;
                                    Some((n.clone(), walked_q_val.clone()))

                                    // Some(("wowow".to_string(), 24))
                                } else {
                                    None
                                }

                                // Some(("wowow".to_string(), 24))
                            } {
                                match self.vm.try_borrow_mut() {
                                    Err(e) => {
                                        println!("NG: {:?}", e.description());
                                    },
                                    Ok(n) => { println!("OK:");}
                                }
                                println!("#.p is Name");
                                result = {
                                    self.vm.borrow_mut().define(n.as_ref(), walked_q_val);
                                    let new_index = self.vm.borrow_mut().alloc(Epiq::Unit);
                                    self.vm.borrow().get_epiq(new_index).clone()
                                };

                                Box::new(result)
                            } else {

                                println!("#.p is not Name");
                                Box::new(input.clone())
                            }
                        },

                        // // environment
                        // "%" => {
                        //     // ひとまずNoneを返しておく
                        //     // 本来は中身もwalkしてから返すべき？
                        //     input
                        // },

                        // // block
                        // r"\" => {
                        //     // TODO: 一つ目の環境の中身はひとまず無視する
                        //     // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
                        //     // ただ、そもそも、blockをevalしても、何も変化はないはず。
                        //     input
                        // },

                        // // apply
                        // "!" => {
                        //     // p: lambda q:arguments
                        //
                        //     let lambda_node = borrowed_vm.get_epiq(p);
                        //     let &Node(_, ref walked_lambda_piq) = self.walk_internal(lambda_node, nest_level + 1);
                        //
                        //     let args_node = borrowed_vm.get_epiq(q);
                        //     let args = self.walk_internal(args_node, nest_level + 1);
                        //
                        //     match walked_lambda_piq {
                        //         &Epiq::Tpiq{o:_, p:lambda_env, q:lambda_body} => {
                        //             // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
                        //             // まず環境を取得
                        //             let env_node = borrowed_vm.get_epiq(lambda_env);
                        //             let &Node(_, ref walked_env_piq) = self.walk_internal(env_node, nest_level + 1);
                        //
                        //             if let &Epiq::Tpiq{o:ref otag, p:_, q:symbol_table} = walked_env_piq {
                        //                 if otag == "%" {
                        //                     // pは無視
                        //                     // qはシンボルのリストになる
                        //                     let params = borrowed_vm.get_epiq(symbol_table);
                        //
                        //                     // 新しい環境フレームを作る
                        //                     self.vm.borrow_mut().extend();
                        //
                        //                     // 束縛を追加する
                        //                     self.assign_arguments(params, args);
                        //
                        //                     // 2. p.q(関数本体)をそのまま返却する
                        //                     // walkを挟んでから返す
                        //                     let lambda_body_node = borrowed_vm.get_epiq(lambda_body);
                        //                     let walked_lambda_body_node = self.walk_internal(lambda_body_node, nest_level + 1);
                        //
                        //                     // 環境フレームを削除する
                        //                     self.vm.borrow_mut().pop();
                        //
                        //                     // Result::NewIndex(new_lambda_body)
                        //                     walked_lambda_body_node
                        //                 } else {
                        //                     println!("env_piqが環境じゃないのでエラー");
                        //                     input
                        //                 }
                        //             } else {
                        //                 println!("env_piqがTpiqじゃないのでエラー");
                        //                 input
                        //             }
                        //         },
                        //
                        //         &Epiq::Prim(ref n) => {
                        //             match n.as_ref() {
                        //                 "decr" => {
                        //                     // 面倒なので 1- を実装
                        //                     // Result::MakeEpiq(Some(Epiq::Uit8(3)))
                        //                     let new_index = self.vm.borrow_mut().alloc(Epiq::Uit8(3));
                        //                     borrowed_vm.get_epiq(new_index)
                        //                 },
                        //                 "ltoreq" => {
                        //                     // <=を実装
                        //                     // Result::MakeEpiq(None)
                        //                     input
                        //                 },
                        //                 _ => {
                        //                     println!("Primitive関数名が想定外なのでエラー");
                        //                     // Result::MakeEpiq(None)
                        //                     input
                        //                 }
                        //             }
                        //         },
                        //
                        //         _ => {
                        //             println!("関数部分がlambdaでもprimでもないのでエラー");
                        //             // Result::MakeEpiq(None)
                        //             self.vm.borrow().get_epiq(input_index)
                        //         },
                        //     }
                        // },

                        // // eval
                        // ">" => {
                        //     println!("eval");
                        //     // p: 用途未定。新しく何かを限定したりとかかなあ。
                        //     //    アイディアは思いつくけど、
                        //     //    そもそもパーサを変えたりとか？
                        //     //    環境を変更したりとか？
                        //     //    継続っぽく使うとか？
                        //     // q: evalされる本体。
                        //
                        //     let borrowed_vm = self.vm.borrow();
                        //     let eval_target_node = borrowed_vm.get_epiq(q);
                        //     let result = self.eval_internal(eval_target_node, nest_level + 1);
                        //     // TODO: これ以下はwalk_internalと重複しているのでまとめたい
                        //     let &Node(new_q, ref new_q_piq) = result;
                        //     if new_q != q {
                        //         let &Node(input_index, _) = input;
                        //         let mut borrow_mut_vm = self.vm.borrow_mut();
                        //         let mut node_mut = borrow_mut_vm.get_epiq_mut(input_index);
                        //         node_mut.1 = new_q_piq.clone();
                        //     }
                        //     input
                        // },

                        // // resolve
                        // "@" => {
                        //     // p: 用途未定。ひとまず無視
                        //     // q: シンボルというか名前
                        //     let node = borrowed_vm.get_epiq(q);
                        //     let &Node(_, ref q_name) = self.walk_internal(node, nest_level + 1);
                        //
                        //     if let &Epiq::Name(ref n) = q_name {
                        //         match borrowed_vm.resolve(n) {
                        //             Some(Some(ref res)) => /*Result::MakeEpiq(Some(res))*/res,
                        //             _ => {
                        //                 println!("resolve時に指定されたキーが見つからない: {:?}", n);
                        //                 // Result::MakeEpiq(None)
                        //                 input
                        //             },
                        //         }
                        //     } else {
                        //         println!("resolve時のキーがNameじゃないのでエラー");
                        //         // Result::MakeEpiq(None)
                        //         input
                        //     }
                        // },

                        // // access
                        // "." => {
                        //     println!("access");
                        //     // p: レシーバ
                        //     // q: アクセッサ
                        //     let &Node(_, ref p_reciever) = self.vm.borrow().get_epiq(p);
                        //     let &Node(_, ref q_accessor) = self.vm.borrow().get_epiq(q);
                        //
                        //     // レシーバの種類によってできることが変わる
                        //     match p_reciever {
                        //         &Epiq::Tpiq{ref o, p, q} => {
                        //             match o.as_ref() {
                        //                 ":" => {
                        //                     // Lpiqならば、pとqが使える、それ以外は無理
                        //                     match q_accessor {
                        //                         &Epiq::Name(ref n) => {
                        //                             match n.as_ref() {
                        //                                 "p" => self.walk_internal(self.vm.borrow().get_epiq(p), nest_level + 1)
                        //                                     /*Result::NewIndex(p)*/,
                        //                                 "q" => self.walk_internal(self.vm.borrow().get_epiq(q), nest_level + 1),
                        //                                 _ => {
                        //                                     /* Lpiqならばpとq以外はエラー */
                        //                                     println!("Lpiqならばpとq以外はエラー");
                        //                                     // Result::MakeEpiq(None)
                        //                                     input
                        //                                 },
                        //                             }
                        //                         },
                        //
                        //                         _ => {
                        //                             /* アクセッサがNameではないのでエラー */
                        //                             println!("アクセッサがNameではないのでエラー");
                        //                             // Result::MakeEpiq(None)
                        //                             input
                        //                         },
                        //                     }
                        //                 },
                        //                 _ => {
                        //                     /* Lpiq以外はまだ定義されていないが、これから増える */
                        //                     println!("Lpiq以外はまだ定義されていないが、これから増える");
                        //                     // Result::MakeEpiq(None)
                        //                     input
                        //                 },
                        //             }
                        //         },
                        //         _ => {
                        //             /* レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから */
                        //             println!("レシーバは今のところTpiq以外にも構造体とかが増えるはずだが、これから");
                        //             // Result::MakeEpiq(None)
                        //             input
                        //         },
                        //     }
                        // },

                        // // condition
                        // "?" => {
                        //     println!("condition");
                        //     // p: ^T or ^F(他の値の評価はひとまず考えない)
                        //     // q: Lpiq、^Tならpを返し、^Fならqを返す
                        //     let &Node(_, ref p_condition) = borrowed_vm.get_epiq(p);
                        //     let &Node(_, ref q_result) = borrowed_vm.get_epiq(q);
                        //
                        //     match p_condition {
                        //         &Epiq::Tval | &Epiq::Fval => {
                        //             match q_result {
                        //                 &Epiq::Tpiq{ref o, p, q} => {
                        //                     if o == ":" {
                        //                         match p_condition {
                        //                             &Epiq::Tval => {
                        //                                 self.walk_internal(self.vm.borrow().get_epiq(p), nest_level + 1)
                        //                             },
                        //                             &Epiq::Fval => {
                        //                                 self.walk_internal(self.vm.borrow().get_epiq(q), nest_level + 1)
                        //                             },
                        //                             _ => {
                        //                                 println!("condtion部分は^Tか^Fしか取れないが、事前に弾いているので、ここは通らないはず");
                        //                                 // Result::MakeEpiq(None)
                        //                                 input
                        //                             },
                        //                         }
                        //                     } else {
                        //                         println!("result部分がLpiqじゃないのでエラー");
                        //                         // Result::MakeEpiq(None)
                        //                         input
                        //                     }
                        //                 },
                        //
                        //                 _ => {
                        //                     println!("result部分がTpiqじゃないのでエラー");
                        //                     // Result::MakeEpiq(None)
                        //                     input
                        //                 },
                        //             }
                        //         },
                        //
                        //         _ => {
                        //             println!("condtion部分は^Tか^Fしか取れないようにしたいのでエラー");
                        //             // Result::MakeEpiq(None)
                        //             input
                        //         },
                        //     }
                        // },

                        _ => Box::new(input.clone()),
                    }
                },

                // &Epiq::Mpiq{ref o, p: _p, q} => {
                //     match o.as_ref() {
                //         ">" => {
                //             // ^> リストのeval
                //             // リストの要素それぞれをevalする
                //             // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)
                //             // Result::MakeEpiq(None)
                //             let eval_list_node = borrowed_vm.get_epiq(q);
                //             self.eval_list(eval_list_node, nest_level + 1)
                //
                //             /*
                //             // 戻り値のindexがすでに存在するなら何もしない
                //             if res <= self.ast.max_id().unwrap() {
                //                 // Result::NewIndex(res)
                //                 result_node
                //             } else {
                //                 let &Node(_, res_piq) = self.vm.borrow().get_epiq(res);
                //                 Result::MakeEpiq(Some(res_piq))
                //             }
                //             */
                //         },
                //
                //         // true
                //         "T" => /*Result::MakeEpiq(Some(Epiq::Tval))*/{
                //             let new_index = self.vm.borrow_mut().alloc(Epiq::Tval);
                //             borrowed_vm.get_epiq(new_index)
                //         },
                //         // false
                //         "F" => {
                //             let new_index = self.vm.borrow_mut().alloc(Epiq::Fval);
                //             borrowed_vm.get_epiq(new_index)
                //         },
                //
                //         _ => input,
                //     }
                // },
                _ => Box::new(input.clone()),
            }
        };/*;
        res

        // match res {
        //     Result::MakeEpiq(Some(new_epiq)) => {
        //         // まずpushだけ
        //         println!("{}eval 生み出す: {:?}", " ".repeat(lvl), new_epiq);
        //         self.vm.borrow_mut().alloc(new_epiq)
        //     },
        //     Result::MakeEpiq(None) => {
        //         // 変化なし
        //         let piq = self.vm.borrow().get_epiq(index);
        //         println!("{}eval 変化なし: {:?}", " ".repeat(lvl), piq);
        //         index
        //     },
        //     Result::NewIndex(i) => {
        //         let &Node(_, piq1) = self.vm.borrow().get_epiq(index);
        //         let &Node(_, piq2) = self.vm.borrow().get_epiq(i);
        //         println!("{}eval 付け替え: {:?} から {:?}", " ".repeat(lvl), piq1, piq2);
        //         i
        //     },
        // }
        */
        Box::new(input.clone())
    }
/*
    fn eval_list(&self, input: &Node<Epiq>, nest_level: u32) -> &Node<Epiq> {
        let lvl = (nest_level * 2) as usize;
        println!("{}eval_list ＿開始＿: {:?}", " ".repeat(lvl), input);

        let &Node(input_index, ref piq) = input;

        let borrowed_vm = self.vm.borrow();

        match piq {
            &Epiq::Tpiq{ref o, p, q} => {
                match o.as_ref() {
                    ":" => {
                        let current_node = borrowed_vm.get_epiq(p);
                        let evaled_current_node = self.eval_internal(current_node, nest_level + 1);
                        if let &Node(_, Epiq::Unit) = borrowed_vm.get_epiq(q) {
                            // リストの最後なので評価の結果を返す
                            evaled_current_node
                        } else {
                            // 次の項目へ
                            let next_node = borrowed_vm.get_epiq(q);
                            self.eval_list(next_node, nest_level + 1)
                        }
                    },
                    _ => borrowed_vm.get_epiq(input_index),
                }
            },
            _ => borrowed_vm.get_epiq(input_index),
        }
        // if let Some(res_index) = {
        //
        // } {
        //     res_index
        // } else {
        //     index
        // }
    }

    fn assign_arguments(&self, parameters_node: &Node<Epiq>, arguments_node: &Node<Epiq>) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        let borrowed_vm = self.vm.borrow();

        let next_params_node;
        let next_args_node;
        {

            let &Node(_, ref params_piq) = parameters_node;
            let &Node(_, ref args_piq) = arguments_node;

            let content;
            if let Some((colon, cntt, next_args)) =
            match args_piq {
                &Epiq::Tpiq{o: ref colon, p: content, q: next_args} => { Some((colon, content, next_args)) },
                _ => { None },
            } {
                if colon != ":" { return; /* 普通は通らない */ }

                next_args_node = borrowed_vm.get_epiq(next_args.clone());
                content = cntt;
            } else {
                /* 普通は通らない */
                println!("assign arguments_piqがおかしい");
                return;
            }

            let &Node(_, ref next_args_piq) = next_args_node;
            let content_node = borrowed_vm.get_epiq(content);

            println!("assign: {:?}", content_node);

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
                    println!("assign parameters_piqがおかしい :じゃないTpiq");
                    return;
                }

                next_params = q;
                param = p;
            } else {
                /* 普通は通らない */
                println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_node);
                return;
            }

            next_params_node = borrowed_vm.get_epiq(next_params);
            let &Node(_, ref _next_params_piq) = next_params_node;
            let &Node(_, ref param_piq) = borrowed_vm.get_epiq(param);

            let mut symbol_string = "";
            if let &Epiq::Name(ref s) = param_piq {
                symbol_string = s;
            } else {
                // 文字列じゃない場合は初期値があるとか、
                // 他の可能性があるが今は実装しない
            }

            self.vm.borrow_mut().define(symbol_string, content_node);


            // paramsとargs、両方のリストを回していくが、
            // ループの基準となるのはargs。
            // paramsが途中でなくなっても知らん。
            if next_args_piq == &Epiq::Unit {
                // 最後なので終了
                println!("assign終わりです");
                return;
            }
        }

        // 次にいく
        self.assign_arguments(next_params_node, next_args_node);


        // match args_piq {
        //     &Epiq::Tpiq{o: ref colon, p: content, q: next_args} => {
        //         if colon == ":" {
        //             let next_args_node = self.vm.borrow().get_epiq(next_args.clone());
        //             let &Node(_, ref next_args_piq) = next_args_node;
        //             let content_node = self.vm.borrow().get_epiq(content);
        //
        //             println!("assign: {:?}", content_node);
        //
        //             match params_piq {
        //                 &Epiq::Tpiq{o: ref colon, p: param, q: next_params} => {
        //                     if colon == ":" {
        //                         let next_params_node = self.vm.borrow().get_epiq(next_params);
        //                         let &Node(_, ref _next_params_piq) = next_params_node;
        //                         let &Node(_, ref param_piq) = self.vm.borrow().get_epiq(param);
        //
        //                         if let &Epiq::Name(ref s) = param_piq {
        //                             self.vm.borrow_mut().define(s, content_node);
        //
        //                             // paramsとargs、両方のリストを回していくが、
        //                             // ループの基準となるのはargs。
        //                             // paramsが途中でなくなっても知らん。
        //                             if next_args_piq == &Epiq::Unit {
        //                                 // 最後なので終了
        //                                 println!("assign終わりです");
        //                             } else {
        //                                 // 次にいく
        //                                 self.assign_arguments(next_params_node, next_args_node);
        //                             }
        //                         } else {
        //                             // 文字列じゃない場合は初期値があるとか、
        //                             // 他の可能性があるが今は実装しない
        //                         }
        //                     } else {
        //                         println!("assign parameters_piqがおかしい :じゃないTpiq");
        //                     }
        //                 },
        //                 _ => {
        //                     /* 普通は通らない */
        //                     println!("assign parameters_piqがおかしい Tpiqじゃない: {:?}", parameters_node);
        //                 },
        //             }
        //         }
        //     },
        //     _ => {
        //         /* 普通は通らない */
        //         println!("assign arguments_piqがおかしい");
        //     },
        // }

    }
    */
}

/*
#[test]
#[ignore]
fn new() {
    let ast = &RefCell::new(AbstractSyntaxTree::new());
    let mut walker = Walker::new(ast);
    walker.walk();
}
*/
