use std::{io::Write as _, path::PathBuf};

use common::{
    eval,
    expr::{tokenize, BinOp, Expr, UnOp},
};

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
                BinOp::Add => format!("({l}) + ({r})"),
                BinOp::Sub => format!("({l}) - ({r})"),
                BinOp::Mul => format!("({l}) * ({r})"),
                BinOp::Div => format!("({l}) / ({r})"),
                BinOp::Mod => format!("({l}) `mod` ({r})"),
                BinOp::Lt => format!("({l}) < ({r})"),
                BinOp::Gt => format!("({l}) > ({r})"),
                BinOp::Eq => format!("({l}) == ({r})"),
                BinOp::Or => format!("({l}) || ({r})"),
                BinOp::And => format!("({l}) && ({r})"),
                BinOp::Concat => format!("({l}) ++ ({r})"),
                BinOp::Take => format!("take ({l}) ({r})"),
                BinOp::Drop => format!("drop ({l}) ({r})"),
                BinOp::App => format!("({l}) ({r})"),
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
                BinOp::Add => format!("(+ (force {l}) (force {r}))"),
                BinOp::Sub => format!("(- (force {l}) (force {r}))"),
                BinOp::Mul => format!("(* (force {l}) (force {r}))"),
                BinOp::Div => format!("(div (force {l}) (force {r}))"),
                BinOp::Mod => format!("(mod (force {l}) (force {r}))"),
                BinOp::Lt => format!("(< (force {l}) (force {r}))"),
                BinOp::Gt => format!("(> (force {l}) (force {r}))"),
                BinOp::Eq => format!("(= (force {l}) (force {r}))"),
                BinOp::Or => format!("(or (force {l}) (force {r}))"),
                BinOp::And => format!("(and (force {l}) (force {r}))"),
                BinOp::Concat => format!("(string-append (force {l}) (force {r}))"),
                BinOp::Take => format!("(string-take (force {r}) (force {l}))"),
                BinOp::Drop => format!("(string-drop (force {r}) (force {l}))"),
                BinOp::App => format!("((force {l}) {r})"),
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
    let expr = Expr::parse_tokens(&tokens)?;
    println!("(use srfi-13)");
    println!("(print (force {}))", to_scheme(&expr));
    Ok(())
}

#[argopt::subcmd]
fn haskell(path: PathBuf) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(&path)?;
    let tokens = tokenize(&s)?;
    let expr = Expr::parse_tokens(&tokens)?;
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
        let expr: Expr = s.parse()?;
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
