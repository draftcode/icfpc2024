#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Proc { name: String, args: Vec<Expr> },
    Str(String),
    Num(i32),
    Var(String),
}
