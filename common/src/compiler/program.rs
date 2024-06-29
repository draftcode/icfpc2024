use super::expr::Expr;

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub exprs: Vec<Expr>,
}
