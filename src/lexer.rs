#[derive(Debug, Clone, PartialEq)]
pub enum Tokn {
    Nmbr(String), // Number
    Text(String), // Text
    Name(String),

    Dbqt, // double quartation
    Lbkt, // left bracket
    Rbkt, // right bracket
    /*
    Colon,
    Lparen,
    Rparen,
    Hash,
    BigP,
    BigQ,
    Carret,
    Dollar,
    BackSlash,
    Asterisk,
    Lbrace,
    Rbrace,
    Question,
    Bang,
    SemiColon,
    Dot,
    */
}

#[derive(Debug)]
pub struct LexerError(pub String);

impl LexerError {
    pub fn new(s: &str) -> LexerError {
        LexerError(s.to_string())
    }
}

pub struct Lexer {}

impl Lexer {
    pub fn lex(s: &str) -> Result<Vec<Tokn>, LexerError> {
        let mut tokens = Vec::new();
        let mut token = String::from("");
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                Lexer::finish(&mut token, &mut tokens);
                continue;
            }

            match c {
                // Number(not 0)
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    Lexer::finish(&mut token, &mut tokens);
                    token.push(c);
                    while let Some(c) = chars.next() {
                        match c {
                            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                                token.push(c);
                            }
                            _ => {
                                tokens.push(Tokn::Nmbr(token.to_string()));
                                token.clear();
                                break;
                            }
                        }
                    }
                },

                // Number can not begin with 0 expect "0"
                '0' => {
                    Lexer::finish(&mut token, &mut tokens);
                    token.push(c);

                    if let Some(c) = chars.next() {
                        if c.is_whitespace() {
                            tokens.push(Tokn::Nmbr("0".to_string()))
                        } else {
                            return Err(LexerError::new("INVALID NUMBER"));
                        }
                    }
                }

                // Text
                '"' => {
                    Lexer::finish(&mut token, &mut tokens);
                    tokens.push(Tokn::Dbqt);
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            tokens.push(Tokn::Text(token.to_string()));
                            token.clear();
                            tokens.push(Tokn::Dbqt);
                            break;
                        } else {
                            token.push(c);
                        }
                    }
                },

                // Left bracket
                '[' => {
                    Lexer::finish(&mut token, &mut tokens);
                    tokens.push(Tokn::Lbkt);
                },

                // Right bracket
                ']' => {
                    Lexer::finish(&mut token, &mut tokens);
                    tokens.push(Tokn::Rbkt);
                },

                // Others (almost symbol)
                _ => token.push(c),
            }
        }

        Lexer::finish(&mut token, &mut tokens);

        Ok(tokens)
    }

    fn finish(t: &mut String, ts: &mut Vec<Tokn>) {
        if !t.is_empty() {
            ts.push(Tokn::Name(t.to_string()));
            t.clear();
        }
    }
}
