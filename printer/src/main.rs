use anyhow::Result;
use std::io;

use common::cps::cps_conversion;
use common::eval::eval;
use common::expr::{BinOp, Expr, UnOp};

pub fn pretty_print_scheme(expr: &Expr) -> String {
    match expr {
        Expr::Bool(b) => {
            if *b {
                "#t".to_string()
            } else {
                "#f".to_string()
            }
        }
        Expr::Int(i) => i.to_string(),
        Expr::String(s) => {
            format!("\"{}\"", s.replace("\n", "\\n"))
        }
        Expr::Var(idx) => {
            format!("v{}", idx)
        }
        Expr::Un(op, expr) => {
            let operand = pretty_print_scheme(expr.as_ref());
            match op {
                UnOp::Neg => {
                    format!("-{}", operand)
                }
                UnOp::Not => {
                    format!("!{}", operand)
                }
                UnOp::StrToInt => {
                    format!("(stoi {})", operand)
                }
                UnOp::IntToStr => {
                    format!("(itos {})", operand)
                }
            }
        }
        Expr::Bin(op, lhs, rhs) => {
            let l = pretty_print_scheme(lhs.as_ref());
            let r = pretty_print_scheme(rhs.as_ref());
            match op {
                BinOp::Add => {
                    format!("(+ {} {})", l, r)
                }
                BinOp::Sub => {
                    format!("(- {} {})", l, r)
                }
                BinOp::Mul => {
                    format!("(* {} {})", l, r)
                }
                BinOp::Div => {
                    format!("(/ {} {})", l, r)
                }
                BinOp::Mod => {
                    format!("(% {} {})", l, r)
                }
                BinOp::Lt => {
                    format!("(< {} {})", l, r)
                }
                BinOp::Gt => {
                    format!("(> {} {})", l, r)
                }
                BinOp::Eq => {
                    format!("(equal? {} {})", l, r)
                }
                BinOp::Or => {
                    format!("(and {} {})", l, r)
                }
                BinOp::And => {
                    format!("(or {} {})", l, r)
                }
                BinOp::Concat => {
                    format!("(string-append {} {})", l, r)
                }
                BinOp::Take => {
                    format!("(take {} {})", l, r)
                }
                BinOp::Drop => {
                    format!("(drop {} {})", l, r)
                }
                BinOp::App => {
                    format!("({} {})", l, r)
                }
                BinOp::AppL => {
                    format!("(delay {} {})", l, r)
                }
                BinOp::AppV => {
                    format!("(AppV {} {})", l, r)
                }
            }
        }
        Expr::If(cond, th, el) => {
            let cond = pretty_print_scheme(cond.as_ref());
            let th = pretty_print_scheme(th.as_ref());
            let el = pretty_print_scheme(el.as_ref());
            format!("(if {} {} {})", cond, th, el)
        }
        Expr::Lambda(idx, body) => {
            let b = pretty_print_scheme(body.as_ref());
            format!("(lambda (v{}) {})", idx, b)
        }
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let exp = buffer.parse::<Expr>()?;
    println!("{}", pretty_print_scheme(&exp));
    Ok(())
}
