use std::str::Chars;

use super::{
    expr::Expr,
    program::Program,
    tokenize::{Token, Tokenizer},
};

pub fn parse<'a>(cs: Chars<'a>) -> anyhow::Result<Program> {
    let toks = Tokenizer::new(cs);

    let mut parser = Parser::new(toks);
    let mut exprs = vec![];
    while let Some(expr) = parser.next_expr() {
        exprs.push(expr);
    }

    Ok(Program { exprs })
}

pub struct Parser<'a> {
    toks: Tokenizer<'a>,
    tok: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(toks: Tokenizer<'a>) -> Self {
        Parser { toks, tok: None }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(tok) = self.tok.take() {
            return Some(tok);
        }
        self.toks.next()
    }

    fn push_back(&mut self, tok: Token) {
        self.tok = tok.into();
    }

    pub fn next_expr(&mut self) -> Option<Expr> {
        let fst = self.next_token()?;
        Some(match fst {
            Token::OpenParen => {
                let name = if let Token::Var(name) = self.next_token()? {
                    name
                } else {
                    panic!("no name after paren");
                };
                let mut args = Vec::new();
                while let Some(tok) = self.next_token() {
                    match tok {
                        Token::CloseParen => return Some(Expr::Proc { name, args }),
                        _ => {
                            self.push_back(tok);
                            args.push(self.next_expr()?);
                        }
                    }
                }
                panic!("no close paren")
            }
            Token::Str(s) => Expr::Str(s),
            Token::Num(n) => Expr::Num(n),
            Token::Var(v) => Expr::Var(v),
            _ => panic!("unexpected token {:?}", fst),
        })
    }
}
