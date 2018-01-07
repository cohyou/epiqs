macro_rules! alloc_node {
    ($s:ident, $n:expr) => {{
        let new_index = $s.vm.borrow_mut().alloc($n);
        $s.get_epiq(new_index)
    }}
}

mod primitive;

use std::rc::Rc;
use std::cell::RefCell;
use core::*;
use printer::*;

const DEGUGGING_NOW: bool = false;
const UNIT_INDX: usize = 0;

pub struct Walker {
    vm: Rc<RefCell<Heliqs>>,
}

impl Walker {
    pub fn new(vm: Rc<RefCell<Heliqs>>) -> Walker {
        Walker { vm: vm, }
    }

    pub fn walk(&self) {
        println!("\n");

        let entry = self.get_entry_node();
        let result = self.walk_internal(entry.clone(), 0);

        // 変化があればentryを変更する
        if result.0 != entry.0 {
            self.vm.borrow_mut().set_entry(result.0);
        }
    }

    fn get_entry_node(&self) -> Node<Rc<Epiq>> {
        if let Some(node) = self.vm.borrow().entry().map(|entry| self.get_epiq(entry)) {
            node
        } else {
            panic!("entryが正しく取得できませんでした");
        }
    }

    fn walk_internal<'a>(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "walk_internal", input.0);

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
            new_epiq_node
        }
    }

    fn eval_internal<'a>(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_internal", input.0);

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

                result
            },

            // consは何もしない
            Epiq::Lpiq(_, _) => input,

            // apply
            Epiq::Appl(p, q) => {
                // p: lambda q:arguments

                let lambda_node = self.get_epiq(p);

                let walked_lambda_box = self.walk_internal(lambda_node, nest_level + 1);
                let walked_lambda_piq = walked_lambda_box.1;

                let args_node = self.get_epiq(q);

                let args = self.walk_internal(args_node, nest_level + 1);

                self.log_piq(nest_level, "args: ", args.0);

                match *walked_lambda_piq {
                    Epiq::Lmbd(lambda_env, lambda_body) => {
                        self.eval_lambda(input, lambda_env, lambda_body, args, nest_level)
                    },

                    Epiq::Prim(ref n) => self.eval_primitive(input, args, n, nest_level),

                    _ => {
                        panic!("関数部分がlambdaでもprimでもないのでエラー");
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
                            panic!("resolve時に指定されたキーが見つからない: {:?}", n);
                        },
                    }
                } else {
                    panic!("resolve時のキーがNameじゃないのでエラー");
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
                    panic!("#.p is not Name");
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

            Epiq::Tpiq{ref o, p, q} => {
                // call macro
                let macrocro = match self.vm.borrow().resolve("macro") {
                    Some(Some(res)) => res.clone().clone(),
                    _ => {
                        panic!("resolve時に指定されたキーが見つからない: {:?}", "macro");
                    },
                };
                let arg1 = self.vm.borrow_mut().alloc(Epiq::Text(o.to_string()));
                let arg2_node = self.get_epiq(p);
                let arg3_node = self.get_epiq(q);

                let args_last = self.vm.borrow_mut().alloc(Epiq::Lpiq(arg3_node.0, UNIT_INDX));
                let args_second = self.vm.borrow_mut().alloc(Epiq::Lpiq(arg2_node.0, args_last));
                let args_first = self.vm.borrow_mut().alloc(Epiq::Lpiq(arg1, args_second));
                let appl = self.vm.borrow_mut().alloc(Epiq::Appl(macrocro.0, args_first));
                let appl_node = self.get_epiq(appl);

                let macro_result = self.eval_internal(appl_node, nest_level + 1);
                macro_result
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

                    _ => panic!("Epiq::Mpiqは>のみ"),
                }
            },

            _ => panic!("eval_internal: 無効なEpiq"),
        }
    }

    fn eval_lambda(&self, input: Node<Rc<Epiq>>,
                          lambda_env: usize, lambda_body: usize, args: Node<Rc<Epiq>>,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_lambda", input.0);
        // 1. bind p.p(環境)の順番に沿って、q(引数リスト)を当てはめていく
        // まず環境を取得
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
            self.assign_arguments(params, args, nest_level);

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
            panic!("apply env_piqがTpiqじゃないのでエラー");
        }
    }

    fn eval_primitive(&self, input: Node<Rc<Epiq>>, args: Node<Rc<Epiq>>, n: &str, nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_primitive", input.0);

        match n.as_ref() {
            "decr" => self.decrement(args),
            "plus" => self.plus(args),
            "minus" => self.minus(args),
            "print" => self.print(args),
            "ltoreq" => self.le_or_eq_nmbr(args),
            "concat" => self.concat(args),
            "eq" => {
                let first = self.pval(args.clone());
                match *first.1 {
                    Epiq::Uit8(n1) => self.eq_nmbr(args),
                    Epiq::Text(ref text1) => self.eq_text(args),
                    Epiq::Unit => {
                        let rest = self.qval(args);
                        let second = self.pval(rest);
                        let new_epiq = if *second.1 == Epiq::Unit { Epiq::Tval } else { Epiq::Fval };

                        alloc_node!(self, new_epiq)
                    }
                    _ => {
                        panic!("primitive 1つ目の引数の型は数値/文字列のみだが違反している, {:?}", self.printer_printed(first.0));
                    },
                }
            },

            _ => {
                panic!("Primitive関数名が想定外なのでエラー");
            },
        }
    }


    fn eval_access(&self, input: Node<Rc<Epiq>>,
                          p: usize, q: usize,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_access", input.0);
        // p: レシーバ
        // q: アクセッサ
        let Node(_, p_reciever) = self.walk_internal(self.get_epiq(p), nest_level + 1);
        let Node(_, q_accessor) = self.walk_internal(self.get_epiq(q), nest_level + 1);

        // レシーバの種類によってできることが変わる
        match *p_reciever {
            Epiq::Lpiq(p, q) => {
                // Lpiqならば、pとqが使える、それ以外は無理
                if let Epiq::Name(ref n) = *q_accessor {
                    match n.as_ref() {
                        "o" => alloc_node!(self, Epiq::Text(":".to_string())),
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
                } else {
                    panic!("アクセッサがNameではないのでエラー");
                }
            },
            Epiq::Tpiq{ref o,p,q} => {
                // Lpiqならば、pとqが使える、それ以外は無理
                if let Epiq::Name(ref n) = *q_accessor {
                    match n.as_ref() {
                        "o" => alloc_node!(self, Epiq::Text(o.to_string())),
                        "p" => {
                            let p_node = self.get_epiq(p);
                            self.walk_internal(p_node, nest_level + 1)
                        },
                        "q" => {
                            let q_node = self.get_epiq(q);
                            self.walk_internal(q_node, nest_level + 1)
                        },
                        _ => {
                            self.log("Tpiqならばoとpとq以外はエラー");
                            input
                        },
                    }
                } else {
                    panic!("アクセッサがNameではないのでエラー");
                }
            },
            _ => {
                panic!("{:?}.{:?}は現在取得できません", *p_reciever, *q_accessor);
            },
        }
    }

    fn eval_condition(&self, input: Node<Rc<Epiq>>, input_index: usize,
                             o: &str, p: usize, q: usize,
                             nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_condition", input.0);

        // p: ^T or ^F(他の値の評価はひとまず考えない)
        // q: Lpiq、^Tならpを返し、^Fならqを返す
        let p_condition = self.get_epiq(p);

        // 条件節をwalk
        // println!("condition: {:?}", "条件節をwalk");
        let walked_condition = self.walk_internal(p_condition.clone(), nest_level + 1);

        // 値がwalk後に変化していたら付け替える
        if walked_condition.0 == p_condition.0 {
            let mut vm = self.vm.borrow_mut();
            let node_mut = vm.get_epiq_mut(input_index);
            node_mut.1 = Rc::new(Epiq::Tpiq{o:o.to_string(), p:walked_condition.0, q:q});
            // println!("{:?} -> ({} {:?}){}condition eval後付け替え", *input.1, input_index, walked_condition_node.1, " ".repeat(lvl));
        }

        let result_piq = self.get_epiq(q);
        match *walked_condition.1 {
            Epiq::Tval => {
                let p_node = self.pval(result_piq);
                self.walk_internal(p_node, nest_level + 1)
            },
            Epiq::Fval => {
                let q_node = self.qval(result_piq);
                self.walk_internal(q_node, nest_level + 1)
            },
            _ => {
                panic!("condtion 評価結果は^Tか^Fだが{:?}なのでエラー", walked_condition.1);
            },
        }
    }

    fn eval_list(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_list", input.0);

        let current_node = self.pval(input.clone());
        let evaled_current_node = self.eval_internal(current_node, nest_level + 1);

        let next = self.qval(input);
        if let Epiq::Unit = *next.1 {
            // リストの最後なので評価の結果を返す
            evaled_current_node
        } else {
            // 次の項目へ
            self.eval_list(next, nest_level + 1)
        }
    }

    fn assign_arguments(&self, parameters_node: Node<Rc<Epiq>>, arguments_node: Node<Rc<Epiq>>, nest_level: u32) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        self.log_piq(nest_level, "assign_arguments", parameters_node.0);

        let content_node = self.pval(arguments_node.clone());
        let next_args_node = self.qval(arguments_node);

        let param_node = self.pval(parameters_node.clone());
        let next_params_node = self.qval(parameters_node);

        let mut symbol_string = "";
        if let Epiq::Name(ref s) = *param_node.1 {
            symbol_string = s;
        } else {
            // 文字列じゃない場合は初期値があるとか、
            // 他の可能性があるが今は実装しない
        }

        self.vm.borrow_mut().define(symbol_string, content_node.0);


        // paramsとargs、両方のリストを回していくが、
        // ループの基準となるのはargs。
        // paramsが途中でなくなっても知らん。
        if *next_args_node.1 == Epiq::Unit {
            // 最後なので終了
            return;
        }


        // 次にいく
        self.assign_arguments(next_params_node, next_args_node, nest_level
        );
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
            panic!("{:?}", s)
            // println!("{:?}", s);
        }
    }

    fn printer_printed(&self, i: NodeId) -> String {
        let printer = Printer::new(self.vm.clone());
        printer.print_aexp(i, 0)
    }
}
