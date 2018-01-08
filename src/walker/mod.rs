macro_rules! alloc_node {
    ($s:ident, $n:expr) => {{
        let new_index = $s.vm.borrow_mut().alloc($n);
        $s.get_epiq(new_index)
    }}
}

macro_rules! unwrap_name {
    ($s:ident, $e:expr) => {{
        match *$e.1 {
            Epiq::Name(ref t) => t,
            _ => {
                let from = $s.printer_printed($e.0);
                panic!("{:?}からnameは取り出せません", from);
            },
        }
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
            Epiq::Eval(p, q) => self.eval(p, q, nest_level),
            Epiq::Quot(_, q) => self.get_epiq(q),

            Epiq::Lpiq(p, q) => self.walk_piq(input, ":", p, q, nest_level),
            Epiq::Appl(p, q) => self.walk_piq(input, "!", p, q, nest_level),
            Epiq::Rslv(p, q) => self.walk_piq(input, "@", p, q, nest_level),
            Epiq::Cond(p, q) => self.walk_piq(input, "?", p, q, nest_level),
            Epiq::Envn(p, q) => self.walk_piq(input, "%", p, q, nest_level),
            Epiq::Bind(p, q) => self.walk_piq(input, "#", p, q, nest_level),
            Epiq::Lmbd(p, q) => self.walk_piq(input, r"\", p, q, nest_level),
            Epiq::Tpiq{ref o, p, q} => self.walk_piq(input, o, p, q, nest_level),

            _ => input,
        }
    }

    fn walk_piq(&self, input: Node<Rc<Epiq>>, o: &str, p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        // pとq両方をwalkして結果が両方とも変わらなければそのまま返す
        // そうでなければ新しくpiqを作って返す
        let p_result = self.walked_node(p, nest_level);
        let q_result = self.walked_node(q, nest_level);

        if p_result.0 == p && q_result.0 == q {
            input
        } else {
            self.walk_piq_internal(o, p_result.0, q_result.0)
        }
    }

    fn walk_piq_internal(&self, o: &str, p: NodeId, q: NodeId) -> Node<Rc<Epiq>> {
        let new_epiq = match o {
            ":" => Epiq::Lpiq(p, q),
            "!" => Epiq::Appl(p, q),
            "@" => Epiq::Rslv(p, q),
            "?" => Epiq::Cond(p, q),
            "%" => Epiq::Envn(p, q),
            "#" => Epiq::Bind(p, q),
            r"\" => Epiq::Lmbd(p, q),
            _   => Epiq::Tpiq{o: o.to_string(), p, q},
        };
        let new_epiq_index = {
            let mut borrow_mut_vm = self.vm.borrow_mut();
            borrow_mut_vm.alloc(new_epiq)
        };

        self.get_epiq(new_epiq_index)
    }

    fn eval_internal<'a>(&self, input: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_internal", input.0);

        let Node(_, piq) = input.clone();

        match *piq {
            Epiq::Unit |
            Epiq::Tval |
            Epiq::Fval |
            Epiq::Uit8(_) |
            Epiq::Name(_) |
            Epiq::Text(_) |
            Epiq::Lpiq(..) => input,

            Epiq::Eval(p, q) => self.eval_internal(self.eval(p, q, nest_level), nest_level + 1), // こっちはあまり通らないかもしれない
            Epiq::Quot(_, q) => self.get_epiq(q),
            Epiq::Appl(p, q) => self.eval_apply(input, p, q, nest_level),
            Epiq::Rslv(p, q) => self.eval_resolve(p, q, nest_level),
            Epiq::Cond(p, q) => self.eval_condition(input, "?", p, q, nest_level),
            Epiq::Envn(..) => self.eval_environment(input),
            Epiq::Bind(p, q) => self.eval_bind(p, q, nest_level),
            Epiq::Accs(p, q) => self.eval_access(input, p, q, nest_level),
            Epiq::Lmbd(..) => self.eval_lambda_direct(input),
            Epiq::Prim(_) => self.eval_primitive_direct(input),

            Epiq::Tpiq{ref o, p, q} => self.eval_tpiq(o, p, q, nest_level),
            Epiq::Mpiq{ref o, p, q} => self.eval_mpiq(o, p, q, nest_level),

            // _ => panic!("eval_internal: 無効なEpiq"),
        }
    }

    /// ひとまずpは無視
    fn eval(&self, _p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        let q_node = self.get_epiq(q);
        let result = self.eval_internal(q_node, nest_level + 1);

        self.log_piq(nest_level, "eval: eval完前", q);
        self.log_piq(nest_level, "eval: eval完後", result.0);

        result
    }

    /// p: lambda
    /// q: arguments
    fn eval_apply(&self, input: Node<Rc<Epiq>>, p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        let lambda = self.walked_node(p, nest_level);
        let args = self.walked_node(q, nest_level);

        self.log_piq(nest_level, "args: ", args.0);

        match *lambda.1 {
            Epiq::Lmbd(env, body) => self.eval_lambda(input, env, body, args, nest_level),
            Epiq::Prim(ref n) => self.eval_primitive(input, n, args, nest_level),
            _ => panic!("関数部分がlambdaでもprimでもないのでエラー: {:?}", lambda.1),
        }
    }

    /// p: 用途未定。ひとまず無視
    /// q: シンボルというか名前
    fn eval_resolve(&self, _p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        let result = self.walked_node(q, nest_level);
        let n = unwrap_name!(self, result);
        match self.vm.borrow().resolve(n) {
            Some(Some(res)) => res.clone().clone(),
            _ => panic!("resolve時に指定されたキーが見つからない: {:?}", n),
        }
    }

    /// ひとまずNoneを返しておく
    /// 本来は中身もwalkしてから返すべき？
    fn eval_environment(&self, input: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        input
    }

    fn eval_bind(&self, p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        let walked_p = self.walked_node(p, nest_level);
        let name = unwrap_name!(self, walked_p);

        let target = self.walked_node(q, nest_level);

        self.vm.borrow_mut().define(name.as_ref(), target.0);
        self.get_epiq(UNIT_INDX)
    }

    /// 直接lambdaをevalした時('> |\)に通る(現在、基本的には何もしない)
    /// 一方、eval_lambda()はapplyを通して呼ばれる
    fn eval_lambda_direct(&self, input: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        // TODO: 一つ目の環境の中身はひとまず無視する
        // qのリストだけを逐次実行して、勝手に最後の値をwalkしてから返却するようにする
        // ただ、そもそも、blockをevalしても、何も変化はないはず。
        input
    }

    /// applyを通して呼ばれる
    /// 一方、eval_lambda_direct()は直接lambdaをevalした時('> |\)に通る
    fn eval_lambda(&self, input: Node<Rc<Epiq>>,
                          env: usize, body: usize, args: Node<Rc<Epiq>>,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_lambda", input.0);

        // 1. 環境を作成する
        let walked_env_box = self.walked_node(env, nest_level);

        if let Epiq::Envn(_, symbols) = *walked_env_box.1 {
            let params = self.get_epiq(symbols);
            self.vm.borrow_mut().extend();
            self.assign_arguments(params, args, nest_level);
        } else {
            panic!("apply envがTpiqではありません");
        }

        // 2. 関数本体をwalkを挟んでから返す
        // TODO: walkにするとLambdaをそのまま返してしまうので、マクロのような扱いになる
        // 実行したければevalしてから返す、
        // しかしできればマクロ展開・関数適用を両方ともこの中でやってしまいたい。。。
        // 今のところはひとまず関数適用しておく（普通にevalを通す）
        let body_node = self.get_epiq(body);
        let evaled_body = self.eval_internal(body_node, nest_level + 1);

        // 3. 環境フレームを削除する
        self.vm.borrow_mut().pop();

        self.log_piq(nest_level, "apply 正常終了: ", evaled_body.0);
        evaled_body
    }

    /// 直接Primをevalした時に通る(現在、基本的には何もしない)
    /// 一方、eval_primitive()はapplyを通して呼ばれる
    fn eval_primitive_direct(&self, input: Node<Rc<Epiq>>) -> Node<Rc<Epiq>> {
        // println!("primitive");
        input
    }

    /// applyを通して呼ばれる
    /// 一方、eval_primitive_direct()は直接Primをevalした時に通る
    fn eval_primitive(&self, input: Node<Rc<Epiq>>, n: &str, args: Node<Rc<Epiq>>, nest_level: u32) -> Node<Rc<Epiq>> {
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
                    Epiq::Uit8(_) => self.eq_nmbr(args),
                    Epiq::Text(_) => self.eq_text(args),
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


    fn eval_access(&self, input: Node<Rc<Epiq>>, p: usize, q: usize,
                          nest_level: u32) -> Node<Rc<Epiq>> {
        self.log_piq(nest_level, "eval_access", input.0);

        // p: レシーバ q: アクセッサ
        let p_reciever = self.walked_node(p, nest_level);
        let q_accessor = self.walked_node(q, nest_level);

        // レシーバの種類によってできることが変わる
        match *p_reciever.1 {
            Epiq::Lpiq(p, q) => {
                // Lpiqならば、pとqのみが使える
                let n = unwrap_name!(self, q_accessor);
                match n.as_ref() {
                    "p" => self.walked_node(p, nest_level),
                    "q" => self.walked_node(q, nest_level),
                    _ => panic!("Lpiqならばpとq以外はエラー"),
                }
            },
            Epiq::Tpiq{ref o,p,q} => {
                // Tpiqならば、pとqが使える
                let n = unwrap_name!(self, q_accessor);
                match n.as_ref() {
                    "o" => alloc_node!(self, Epiq::Text(o.to_string())),
                    "p" => self.walked_node(p, nest_level),
                    "q" => self.walked_node(q, nest_level),
                    _ => panic!("Tpiqならばoとpとq以外はエラー"),
                }
            },
            _ => panic!("{:?}に{:?}というアクセッサはありません", *p_reciever.1, *q_accessor.1),
        }
    }

    fn eval_condition(&self, input: Node<Rc<Epiq>>, o: &str, p: usize, q: usize,
                             nest_level: u32) -> Node<Rc<Epiq>> {
        /// p: ^T or ^F(他の値の評価はひとまず考えない)
        /// q: Lpiq、^Tならpを返し、^Fならqを返す
        self.log_piq(nest_level, "eval_condition", input.0);

        // 条件節をwalk
        let cond = self.get_epiq(p);
        let walked_cond = self.walk_internal(cond.clone(), nest_level + 1);

        // 値がwalk後に変化していたら付け替える
        if cond.0 == walked_cond.0 {
            let mut vm = self.vm.borrow_mut();
            let node_mut = vm.get_epiq_mut(input.0);
            node_mut.1 = Rc::new(Epiq::Tpiq{o:o.to_string(), p:walked_cond.0, q});
        }

        let result = self.get_epiq(q);
        match *walked_cond.1 {
            Epiq::Tval => self.eval_internal(self.pval(result), nest_level + 1),
            Epiq::Fval => self.eval_internal(self.qval(result), nest_level + 1),
            _ => panic!("condtion 評価結果は^Tか^Fだが{:?}なのでエラー", self.printer_printed(walked_cond.0)),
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

    fn assign_arguments(&self, params: Node<Rc<Epiq>>, args: Node<Rc<Epiq>>, nest_level: u32) {
        // arguments_piqはリストのはずなので、一つ一つ回して定義していく
        self.log_piq(nest_level, "assign_arguments", params.0);
        
        // 引数が必要ない場合は即終了
        if *params.1 == Epiq::Unit {
            return;
        }

        let param = self.pval(params.clone());
        let next_params = self.qval(params);

        let mut symbol_name = "";
        if let Epiq::Name(ref s) = *param.1 {
            symbol_name = s;
        } else {
            // 文字列じゃない場合は初期値があるとか、他の可能性があるが今は実装しない
        }

        let content = self.pval(args.clone());
        let next_args = self.qval(args);

        self.vm.borrow_mut().define(symbol_name, content.0);

        // paramsとargs、両方のリストを回していく
        // ループの基準となるのはargs。paramsが途中でなくなっても知らん。
        if *next_args.1 != Epiq::Unit {
            // 次にいく
            self.assign_arguments(next_params, next_args, nest_level);
        }
    }

    fn eval_tpiq(&self, o: &str, p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
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
        let eval = self.vm.borrow_mut().alloc(Epiq::Eval(UNIT_INDX, appl));
        let appl_node = self.get_epiq(eval);

        let macro_result = self.eval_internal(appl_node, nest_level + 1);
        macro_result
    }

    fn eval_mpiq(&self, o: &str, _p: NodeId, q: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        match o.as_ref() {
            ">" => {
                // ^> リストの要素それぞれをevalする
                // pは-1だとして処理する(最後の項目の評価結果が最終的な結果となる)

                let eval_list_node = self.get_epiq(q);
                let result = self.eval_list(eval_list_node, nest_level + 1);
                result
            },

            _ => panic!("Epiq::Mpiqは>のみ"),
        }
    }

    fn walked_node(&self, i: NodeId, nest_level: u32) -> Node<Rc<Epiq>> {
        let node = self.get_epiq(i);
        self.walk_internal(node, nest_level + 1)
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
