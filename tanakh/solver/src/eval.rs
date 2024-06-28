use anyhow::bail;

use crate::expr::{base94, base94enc, BinOp, Expr, UnOp};

#[derive(Default)]
struct Env {
    count: usize,
}

impl Env {
    // fn push(&mut self, v: usize, e: Expr) {
    //     eprintln!("push: v{v} {e:?}");
    //     self.vars.push((v, e));
    // }

    // fn pop(&mut self) {
    //     eprintln!("pop");
    //     self.vars.pop();
    // }

    // fn lookup(&self, v: usize) -> Option<&Expr> {
    //     eprintln!("lookup: v{v} {:?}", self.vars);
    //     self.vars.iter().rev().find(|(w, _)| *w == v).map(|r| &r.1)
    // }
}

pub fn eval(e: &Expr) -> anyhow::Result<Expr> {
    let mut e = e.clone();
    loop {
        let ne = reduce(&e, &mut Env::default())?;
        if e == ne {
            return Ok(e);
        }
        e = ne;
    }
}

fn reduce(e: &Expr, env: &mut Env) -> anyhow::Result<Expr> {
    log::info!("eval: {e}");

    Ok(match e {
        Expr::Un(op, e) => {
            let e = reduce(e.as_ref(), env)?;
            match op {
                UnOp::Neg => match e {
                    Expr::Int(n) => Expr::Int(-n),
                    _ => bail!("Invalid operator for neg: {e:?}"),
                },
                UnOp::Not => match e {
                    Expr::Bool(b) => Expr::Bool(!b),
                    _ => bail!("Invalid operator for not: {e:?}"),
                },
                UnOp::StrToInt => match e {
                    Expr::String(s) => Expr::Int(str_to_int(&s)),
                    _ => bail!("Invalid operator for str_to_int: {e:?}"),
                },
                UnOp::IntToStr => match e {
                    Expr::Int(n) => Expr::String(int_to_str(n)),
                    _ => bail!("Invalid operator for int_to_str: {e:?}"),
                },
            }
        }
        Expr::Bin(op, l, r) => {
            if matches!(op, BinOp::App) {
                dbg!(l, r);
                let f = reduce(l.as_ref(), env)?;
                match f {
                    Expr::Lambda(v, e) => {
                        return beta_reduction(e.as_ref(), v, r.as_ref(), &mut vec![]);
                    }
                    _ => bail!("Invalid operator for app: {f:?}"),
                }
            }

            let l = reduce(l.as_ref(), env)?;
            let r = reduce(r.as_ref(), env)?;
            match (op, &l, &r) {
                (BinOp::Add, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 + n2),
                    _ => bail!("Invalid operator for add: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Sub, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 - n2),
                    _ => bail!("Invalid operator for sub: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Mul, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 * n2),
                    _ => bail!("Invalid operator for mul: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Div, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 / n2),
                    _ => bail!("Invalid operator for div: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Mod, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 % n2),
                    _ => bail!("Invalid operator for mod: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Lt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 < n2),
                    _ => bail!("Invalid operator for lt: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Gt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 > n2),
                    _ => bail!("Invalid operator for gt: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Eq, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 == n2),
                    _ => bail!("Invalid operator for eq: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Or, l, r) => match (l, r) {
                    (Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 || *b2),
                    _ => bail!("Invalid operator for or: {op:?} {l:?} {r:?}"),
                },
                (BinOp::And, l, r) => match (l, r) {
                    (Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 && *b2),
                    _ => bail!("Invalid operator for and: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Concat, l, r) => match (l, r) {
                    (Expr::String(s1), Expr::String(s2)) => Expr::String(s1.clone() + s2),
                    _ => bail!("Invalid operator for concat: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Take, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => {
                        Expr::String(s.chars().take(*n as usize).collect())
                    }
                    _ => bail!("Invalid operator for take: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Drop, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => {
                        Expr::String(s.chars().skip(*n as usize).collect())
                    }
                    _ => bail!("Invalid operator for drop: {op:?} {l:?} {r:?}"),
                },
                _ => unreachable!(),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = reduce(cond.as_ref(), env)?;
            match cond {
                Expr::Bool(true) => th.as_ref().clone(),
                Expr::Bool(false) => el.as_ref().clone(),
                _ => bail!("Invalid condition: {cond:?}"),
            }
        }
        // Expr::Lambda(v, e) => Expr::Lambda(*v, reduce(e.as_ref(), env)?.into()),
        _ => e.clone(),
    })
}

fn str_to_int(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret = ret * 94 + base94(c).unwrap();
    }
    ret
}

fn int_to_str(n: i64) -> String {
    let mut s = String::new();
    let mut n = n;
    while n > 0 {
        s.push(base94enc(n % 94).unwrap());
        n /= 94;
    }
    s.chars().rev().collect::<String>()
}

fn beta_reduction(e: &Expr, v: usize, arg: &Expr, shadow: &mut Vec<usize>) -> anyhow::Result<Expr> {
    Ok(match e {
        Expr::Var(w) if v == *w && !shadow.contains(w) => arg.clone(),
        Expr::Un(op, e) => Expr::Un(*op, beta_reduction(e.as_ref(), v, arg, shadow)?.into()),
        Expr::Bin(op, l, r) => Expr::Bin(
            *op,
            beta_reduction(l.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(r.as_ref(), v, arg, shadow)?.into(),
        ),
        Expr::If(cond, th, el) => Expr::If(
            beta_reduction(cond.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(th.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(el.as_ref(), v, arg, shadow)?.into(),
        ),
        Expr::Lambda(w, e) => {
            shadow.push(*w);
            let e = beta_reduction(e.as_ref(), v, arg, shadow)?;
            shadow.pop();
            Expr::Lambda(*w, e.into())
        }
        _ => e.clone(),
    })
}
