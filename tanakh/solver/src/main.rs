use std::io::BufRead as _;

use expr::{tokenize, Expr};
use solver::*;

// fn reduce(&self) -> Self {
//     match self {
//         Expr::Bool(_) => self.clone(),
//         Expr::Int(_) => self.clone(),
//         Expr::String(_) => self.clone(),
//         Expr::Var(_) => self.clone(),
//         Expr::Un(op, e) => {
//             let e = e.reduce();
//             match (op, &e) {
//                 (UnOp::Neg, Expr::Int(n)) => Expr::Int(-n),
//                 (UnOp::Not, Expr::Bool(b)) => Expr::Bool(!b),
//                 (UnOp::StrToInt, Expr::String(s)) => {
//                     let n = s.chars().fold(0, |acc, c| acc * 94 + base94(c).unwrap());
//                     Expr::Int(n)
//                 }
//                 (UnOp::IntToStr, Expr::Int(n)) => {
//                     let mut s = String::new();
//                     let mut n = *n;
//                     while n > 0 {
//                         s.push(base94_char((n % 94) as u8 as char).unwrap());
//                         n /= 94;
//                     }
//                     s.chars().rev().collect::<String>().into()
//                 }
//                 _ => Expr::Un(*op, e),
//             }
//         }
//         Expr::Bin(op, e1, e2) => {
//             let e1 = e1.reduce();
//             let e2 = e2.reduce();
//             match (op, &e1, &e2) {
//                 (BinOp::Add, Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 + n2),
//                 (BinOp::Sub, Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 - n2),
//                 (BinOp::Mul, Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 * n2),
//                 (BinOp::Div, Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 / n2),
//                 (BinOp::Mod, Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 % n2),
//                 (BinOp::Lt, Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 < n2),
//                 (BinOp::Gt, Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 > n2),
//                 (BinOp::Eq, Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 == n2),
//                 (BinOp::Or, Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 || *b2),
//                 (BinOp::And, Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 && *b2),
//                 (BinOp::Concat, Expr::String(s1), Expr::String(s2)) => {
//                     Expr::String(s1.clone() + s2)
//                 }
//                 (BinOp::Take, Expr::String(s), Expr::Int(n)) => {
//                     Expr::String(s.chars().take(*n as usize).collect())
//                 }
//             }
//         }
//     }
//     todo!()
// }

fn main() -> anyhow::Result<()> {
    env_logger::init();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens = tokenize(&line)?;
        // println!("{:?}", tokens);
        let expr = Expr::parse(&tokens)?;
        println!("{}", expr);

        let result = eval::eval(&expr)?;
        println!("{:?}", result);
    }
    Ok(())
}
