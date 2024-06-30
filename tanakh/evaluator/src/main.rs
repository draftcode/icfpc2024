use std::{io::Write as _, path::PathBuf, rc::Rc};

use common::{
    eval,
    expr::{tokenize, BinOp, Expr, UnOp},
};

fn is_fix(e: &Expr) -> bool {
    let check = |e: &Expr, f: usize| -> bool {
        match e {
            Expr::Lambda(x, e) => match e.as_ref() {
                Expr::Bin(BinOp::App, l, r) => {
                    matches!(l.as_ref(), Expr::Var(v) if *v == f)
                        && matches!(r.as_ref(), Expr::Bin(BinOp::App, v1, v2) if matches!(v1.as_ref(), Expr::Var(v1) if v1 == x) && matches!(v2.as_ref(), Expr::Var(v2) if v2 == x))
                }
                _ => false,
            },
            _ => false,
        }
    };

    match e {
        Expr::Lambda(f, e) => match e.as_ref() {
            Expr::Bin(BinOp::App, l, r) => {
                let l = check(&l, *f);
                let r = check(&r, *f);
                l && r
            }
            _ => false,
        },
        _ => false,
    }
}

fn pp(e: &Expr) -> String {
    if is_fix(e) {
        return "fix".to_string();
    }

    match e {
        Expr::Bool(b) => if *b { "T" } else { "F" }.to_string(),
        Expr::Int(n) => format!("{n}"),
        Expr::String(s) => format!("{s:?}"),
        Expr::Var(v) => format!("v{v}"),
        Expr::Un(op, e) => {
            let e = pp(e.as_ref());
            match op {
                UnOp::Neg => format!("-{e}"),
                UnOp::Not => format!("!{e}"),
                UnOp::StrToInt => format!("str-to-int {e}"),
                UnOp::IntToStr => format!("int-to-str {e}"),
            }
        }
        Expr::Bin(op, l, r) => {
            let l = pp(l.as_ref());
            let r = pp(r.as_ref());

            #[allow(unreachable_patterns)]
            match op {
                BinOp::Add => format!("({l} + {r})"),
                BinOp::Sub => format!("({l} - {r})"),
                BinOp::Mul => format!("({l} * {r})"),
                BinOp::Div => format!("({l} / {r})"),
                BinOp::Mod => format!("({l} % {r})"),
                BinOp::Lt => format!("({l} < {r})"),
                BinOp::Gt => format!("({l} > {r})"),
                BinOp::Eq => format!("({l} == {r})"),
                BinOp::Or => format!("({l} || {r})"),
                BinOp::And => format!("({l} && {r})"),
                BinOp::Concat => format!("({l} ++ {r})"),
                BinOp::Take => format!("take ({l}) ({r})"),
                BinOp::Drop => format!("drop ({l}) ({r})"),
                BinOp::App => format!("({l} {r})"),
                _ => unreachable!(),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = pp(cond);
            let th = pp(th);
            let el = pp(el);
            format!("(if {cond} then {th} else {el})")
        }
        Expr::Lambda(v, e) => {
            let mut vs = vec![v];
            let mut e = e.as_ref();

            loop {
                match e {
                    Expr::Lambda(v, e1) => {
                        vs.push(v);
                        e = e1.as_ref();
                    }
                    _ => break,
                }
            }

            let e = pp(e);
            format!(
                "(\\{} -> {e})",
                vs.into_iter()
                    .map(|v| format!("v{v}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }
}

fn simplify_comb(e: &Expr) -> Expr {
    if is_fix(e) {
        return Expr::Var(usize::MAX);
    }

    match e {
        Expr::Un(op, e) => Expr::Un(*op, Rc::new(simplify_comb(e))),
        Expr::Bin(op, l, r) => Expr::Bin(*op, Rc::new(simplify_comb(l)), Rc::new(simplify_comb(r))),
        Expr::If(cond, th, el) => Expr::If(
            Rc::new(simplify_comb(cond)),
            Rc::new(simplify_comb(th)),
            Rc::new(simplify_comb(el)),
        ),
        Expr::Lambda(v, e) => Expr::Lambda(*v, Rc::new(simplify_comb(e))),
        _ => e.clone(),
    }
}

fn to_haskell(e: &Expr) -> String {
    match e {
        Expr::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Expr::Int(n) => format!("{:#x}", n.as_ref()),
        Expr::String(s) => format!("{s:?}"),
        Expr::Var(n) => {
            if *n == usize::MAX {
                return "fix".to_string();
            } else {
                format!("v{n}")
            }
        }
        Expr::Un(op, e) => {
            match e.as_ref() {
                Expr::Bin(BinOp::Eq, l, r) => {
                    let l = to_haskell(l);
                    let r = to_haskell(r);
                    return format!("({l} /= {r})");
                }
                _ => {}
            }

            let e = to_haskell(e);
            match op {
                UnOp::Neg => format!("(- {})", e),
                UnOp::Not => format!("(not {})", e),
                UnOp::StrToInt => format!("(strToInt {e})"),
                UnOp::IntToStr => format!("(intToStr {e})"),
            }
        }
        Expr::Bin(op, l, r) => {
            let r = to_haskell(r);
            match (op, l.as_ref()) {
                (BinOp::App, Expr::Lambda(v, e)) => {
                    return format!("(let v{v} = {r} in {})", to_haskell(e));
                }
                _ => {}
            }

            if matches!(op, BinOp::Or) {
                let mut l = l;
                let mut rs = vec![r];

                loop {
                    match l.as_ref() {
                        Expr::Bin(BinOp::Or, l1, r1) => {
                            l = l1;
                            rs.push(to_haskell(r1));
                        }
                        _ => break,
                    }
                }

                let l = to_haskell(l);
                rs.reverse();
                return format!("({l} || {})", rs.join(" || "));
            }

            if matches!(op, BinOp::And) {
                let mut l = l;
                let mut rs = vec![r];

                loop {
                    match l.as_ref() {
                        Expr::Bin(BinOp::And, l1, r1) => {
                            l = l1;
                            rs.push(to_haskell(r1));
                        }
                        _ => break,
                    }
                }

                let l = to_haskell(l);
                rs.reverse();
                return format!("({l} && {})", rs.join(" && "));
            }

            let l = to_haskell(l);

            #[allow(unreachable_patterns)]
            match op {
                BinOp::Add => format!("({l} + {r})"),
                BinOp::Sub => format!("({l} - {r})"),
                BinOp::Mul => format!("({l} * {r})"),
                BinOp::Div => format!("({l} `div` {r})"),
                BinOp::Mod => format!("({l} `mod`{r})"),
                BinOp::Lt => format!("({l} < {r})"),
                BinOp::Gt => format!("({l} > {r})"),
                BinOp::Eq => format!("({l} == {r})"),
                BinOp::Or => format!("({l} || {r})"),
                BinOp::And => format!("({l} && {r})"),
                BinOp::Concat => format!("({l} ++ {r})"),
                BinOp::Take => format!("(take {l} {r})"),
                BinOp::Drop => format!("(drop {l} {r})"),
                BinOp::App => format!("({l} {r})"),
                _ => unreachable!(),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = to_haskell(cond);
            let th = to_haskell(th);
            let el = to_haskell(el);
            format!("(if {cond} then {th} else {el})")
        }
        Expr::Lambda(v, e) => {
            let mut vs = vec![v];
            let mut e = e.as_ref();

            loop {
                match e {
                    Expr::Lambda(v1, e1) => {
                        vs.push(v1);
                        e = e1.as_ref();
                    }
                    _ => break,
                }
            }

            let e = to_haskell(e);
            format!(
                "(\\{} -> {e})",
                vs.into_iter()
                    .map(|v| format!("v{v}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
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

            #[allow(unreachable_patterns)]
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
                _ => format!("Not support op {:?}", op),
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
fn pretty(path: PathBuf) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(&path)?;
    let tokens = tokenize(&s)?;
    let expr = Expr::parse_tokens(&tokens)?;
    println!("{}", pp(&expr));
    Ok(())
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
    let expr = simplify_comb(&expr);
    println!("import Control.Monad.Fix");
    println!("main = print $ {}", to_haskell(&expr));
    Ok(())
}

#[argopt::subcmd]
fn repl() -> anyhow::Result<()> {
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

#[argopt::cmd_group(commands = [scheme, haskell, repl, pretty])]
fn main() -> anyhow::Result<()> {
    env_logger::init();
}
