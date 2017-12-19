// 書き換えたがLexerと同様の構造にするのが難しそうだったので一旦保留
/*
#[derive(Eq, PartialEq, Clone, Debug)]
enum State {
    Aexp,
    WaitOtag,
    Error(String),
}
*/

/*
fn parse_flat() {
    loop {
        let s = self.state.clone();
        let t = self.current_token.borrow().clone();

        println!("parser state: {:?} token: {:?}", s, t);
        match s {
            State::Aexp => {
                match t {
                    CurrentToken::SOT => {
                        // 初期値状態 最初のトークンを取得する
                        self.consume_token();
                    },

                    CurrentToken::Has(Tokn::Pipe) => {
                        self.state = State::WaitOtag;

                        self.consume_token();

                        let len = self.aexp_tokens.len();
                        self.aexp_tokens[len - 1].push(Tokn::Pipe);
                    },

                    CurrentToken::Has(Tokn::Chvc(ref s)) => {
                        self.consume_token();

                        let name = Epiq::Name(s.clone());
                        self.ast.borrow_mut().push(name);

                        let len = self.aexp_tokens.len();
                        self.aexp_tokens[len - 1].push(Tokn::Chvc(s.to_string()));
                    },

                    CurrentToken::Has(Tokn::Nmbr(ref s)) => {
                        self.consume_token();
                        self.ast.borrow_mut().push(Epiq::Uit8(s.parse::<u64>().unwrap()));

                        let len = self.aexp_tokens.len();
                        self.aexp_tokens[len - 1].push(Tokn::Nmbr(ref s));
                    },

                    CurrentToken::EOT => {
                        break;
                    },
                    _ => {},
                }
            },

            State::WaitOtag => {
                match t {
                    CurrentToken::Has(Tokn::Otag(tag_name)) => {
                        match tag_name.as_str() {
                            ":" => {
                                self.state = State::Aexp;
                                self.consume_token();
                                let len = self.aexp_tokens.len();
                                self.aexp_tokens[len - 1].push(Tokn::Otag(tag_name));
                                self.aexp_tokens.push(vec![]);
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },

            State::Error(_e) => {
                break;
            },
        }
    }
}
*/
