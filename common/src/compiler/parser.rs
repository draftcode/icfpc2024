use std::str::Chars;

use super::{
    expr::Expr,
    program::Program,
    tokenize::{Token, Tokenizer},
};

pub fn parse<'a>(cs: Chars<'a>) -> anyhow::Result<Program> {
    let str = cs.collect::<String>();
    let toks = Tokenizer::new(&str);

    let mut parser = Parser::new(toks);
    let mut exprs = vec![];
    while let Some(expr) = parser.next_expr() {
        exprs.push(expr);
    }

    Ok(Program { exprs })
}

pub fn parse_str(prog: &str) -> anyhow::Result<Program> {
    parse(prog.chars())
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
                let mut args: Vec<Expr> = vec![];
                while let Some(tok) = self.next_token() {
                    match tok {
                        Token::CloseParen => {
                            if args[0] == Expr::Var("lambda".to_string()) {
                                let names = args[1].must_proc();
                                let expr = args[2].clone();
                                if names.len() != 1 {
                                    panic!("lambda should have 1 arg");
                                }
                                return Some(Expr::lambda(names[0].to_string(), expr));
                            }
                            return Some(Expr::Proc(args));
                        }
                        _ => {
                            self.push_back(tok);
                            args.push(self.next_expr()?);
                        }
                    }
                }
                panic!("no close paren");
            }
            Token::Str(s) => Expr::Str(s),
            Token::Num(n) => Expr::Num(n),
            Token::Var(v) => Expr::Var(v),
            _ => panic!("unexpected token {:?}", fst),
        })
    }
}
