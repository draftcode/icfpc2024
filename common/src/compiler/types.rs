#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Proc { name: String, args: Vec<Expr> },
    Str(String),
    Num(i32),
    Var(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Str(String),
    Num(i32),
    Var(String),
}
