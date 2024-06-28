use std::{io::Write as _, path::PathBuf};

use evaluator::*;
use expr::{tokenize, Expr, UnOp};

fn to_haskell(e: &Expr) -> String {
    match e {
        Expr::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Expr::Int(n) => n.to_string(),
        Expr::String(s) => format!("{s:?}"),
        Expr::Var(n) => format!("v{n}"),
        Expr::Un(op, e) => {
            let e = to_haskell(e);
            match op {
                UnOp::Neg => format!("-({})", e),
                UnOp::Not => format!("not ({})", e),
                UnOp::StrToInt => format!("strToInt ({e})"),
                UnOp::IntToStr => format!("intToStr ({e})"),
            }
        }
        Expr::Bin(op, l, r) => {
            let l = to_haskell(l);
            let r = to_haskell(r);
            match op {
                expr::BinOp::Add => format!("({l}) + ({r})"),
                expr::BinOp::Sub => format!("({l}) - ({r})"),
                expr::BinOp::Mul => format!("({l}) * ({r})"),
                expr::BinOp::Div => format!("({l}) / ({r})"),
                expr::BinOp::Mod => format!("({l}) `mod` ({r})"),
                expr::BinOp::Lt => format!("({l}) < ({r})"),
                expr::BinOp::Gt => format!("({l}) > ({r})"),
                expr::BinOp::Eq => format!("({l}) == ({r})"),
                expr::BinOp::Or => format!("({l}) || ({r})"),
                expr::BinOp::And => format!("({l}) && ({r})"),
                expr::BinOp::Concat => format!("({l}) ++ ({r})"),
                expr::BinOp::Take => format!("take ({l}) ({r})"),
                expr::BinOp::Drop => format!("drop ({l}) ({r})"),
                expr::BinOp::App => format!("({l}) ({r})"),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = to_haskell(cond);
            let th = to_haskell(th);
            let el = to_haskell(el);
            format!("if {cond} then {th} else {el}")
        }
        Expr::Lambda(v, e) => {
            let e = to_haskell(e);
            format!("\\v{v} -> {e}")
        }
    }
}

fn to_scheme(e: &Expr) -> String {
    match e {
        Expr::Bool(b) => format!("(lazy {})", if *b { "#t" } else { "#f" }),
        Expr::Int(n) => format!("(lazy {n})"),
        Expr::String(s) => format!("(lazy {s:?})"),
        Expr::Var(n) => format!("(lazy v{n})"),
        Expr::Un(op, e) => {
            let e = to_scheme(e);
            let ret = match op {
                UnOp::Neg => format!("(- (force {e}))"),
                UnOp::Not => format!("(not (force {e}))"),
                UnOp::StrToInt => format!("(str-to-int (force {e}))"),
                UnOp::IntToStr => format!("(int-to-str (force {e}))"),
            };
            format!("(lazy {ret})")
        }
        Expr::Bin(op, l, r) => {
            let l = to_scheme(l);
            let r = to_scheme(r);
            let ret = match op {
                expr::BinOp::Add => format!("(+ (force {l}) (force {r}))"),
                expr::BinOp::Sub => format!("(- (force {l}) (force {r}))"),
                expr::BinOp::Mul => format!("(* (force {l}) (force {r}))"),
                expr::BinOp::Div => format!("(div (force {l}) (force {r}))"),
                expr::BinOp::Mod => format!("(mod (force {l}) (force {r}))"),
                expr::BinOp::Lt => format!("(< (force {l}) (force {r}))"),
                expr::BinOp::Gt => format!("(> (force {l}) (force {r}))"),
                expr::BinOp::Eq => format!("(= (force {l}) (force {r}))"),
                expr::BinOp::Or => format!("(or (force {l}) (force {r}))"),
                expr::BinOp::And => format!("(and (force {l}) (force {r}))"),
                expr::BinOp::Concat => format!("(string-append (force {l}) (force {r}))"),
                expr::BinOp::Take => format!("(string-take (force {r}) (force {l}))"),
                expr::BinOp::Drop => format!("(string-drop (force {r}) (force {l}))"),
                expr::BinOp::App => format!("((force {l}) {r})"),
            };
            format!("(lazy {ret})")
        }
        Expr::If(cond, th, el) => {
            let cond = to_scheme(cond);
            let th = to_scheme(th);
            let el = to_scheme(el);
            format!("(lazy (if (force {cond}) (lazy {th}) (lazy {el})))")
        }
        Expr::Lambda(v, e) => {
            let e = to_scheme(e);
            format!("(lazy (lambda (v{v}) (lazy {e})))")
        }
    }
}

#[argopt::subcmd]
fn scheme(path: PathBuf) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(&path)?;
    let tokens = tokenize(&s)?;
    let expr = Expr::parse(&tokens)?;
    println!("(use srfi-13)");
    println!("(print (force {}))", to_scheme(&expr));
    Ok(())
}

#[argopt::subcmd]
fn haskell(path: PathBuf) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(&path)?;
    let tokens = tokenize(&s)?;
    let expr = Expr::parse(&tokens)?;
    println!("{}", to_haskell(&expr));
    Ok(())
}

#[argopt::subcmd]
fn repl() -> anyhow::Result<()> {
    env_logger::init();

    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    loop {
        write!(&mut stdout, "> ")?;
        stdout.flush()?;

        let mut s = String::new();
        stdin.read_line(&mut s)?;
        let tokens = tokenize(&s)?;
        log::info!("{:?}", tokens);
        let expr = Expr::parse(&tokens)?;
        log::info!("{}", expr);
        let result = eval::eval(&expr)?;
        println!("{}", result);

        match result {
            Expr::String(s) => {
                println!("=== string ===");
                println!("{}", s);
            }
            _ => {}
        }
    }
}

#[argopt::cmd_group(commands = [scheme, haskell, repl])]
fn main() -> anyhow::Result<()> {}
