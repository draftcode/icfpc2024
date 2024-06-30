use std::rc::Rc;

use anyhow::bail;
use num_bigint::BigInt;

use crate::{
    base94::{decode_base94, decode_char, encode_base94, encode_str},
    expr::{BinOp, Expr, UnOp},
};

#[derive(Default, Clone, Debug)]
struct Stats {
    beta_reductions: usize,
}

pub fn eval(e: &Expr) -> anyhow::Result<Expr> {
    reduce_to_nf(e, &mut Stats::default())
}

fn reduce_to_nf(e: &Expr, stats: &mut Stats) -> anyhow::Result<Expr> {
    log::trace!("eval: {e}");

    Ok(match e {
        Expr::Un(op, e) => {
            let e = reduce_to_nf(e.as_ref(), stats)?;
            match op {
                UnOp::Neg => match e {
                    Expr::Int(n) => Expr::Int(Rc::new(-n.as_ref().clone())),
                    _ => bail!("Invalid operator for neg: {e:?}"),
                },
                UnOp::Not => match e {
                    Expr::Bool(b) => Expr::Bool(!b),
                    _ => bail!("Invalid operator for not: {e:?}"),
                },
                UnOp::StrToInt => match e {
                    Expr::String(s) => Expr::Int(str_to_int(&s).into()),
                    _ => bail!("Invalid operator for str_to_int: {e:?}"),
                },
                UnOp::IntToStr => match e {
                    Expr::Int(n) => Expr::String(int_to_str(&n).into()),
                    _ => bail!("Invalid operator for int_to_str: {e:?}"),
                },
            }
        }
        Expr::Bin(op, l, r) => {
            if matches!(op, BinOp::App) {
                log::trace!("app: {l}, {r}");
                let f = reduce_to_nf(l.as_ref(), stats)?;
                match f {
                    Expr::Lambda(v, e) => {
                        stats.beta_reductions += 1;
                        return reduce_to_nf(
                            &beta_reduction(e.as_ref(), v, r.as_ref(), &mut vec![])?,
                            stats,
                        );
                    }
                    _ => bail!("Invalid operator for app: {f}"),
                }
            }
            if matches!(op, BinOp::AppV) {
                log::trace!("app: {l}, {r}");
                let f = reduce_to_nf(l.as_ref(), stats)?;
                // It's okay to eval the rhs because it's call-by-value.
                let g = reduce_to_nf(r.as_ref(), stats)?;
                match f {
                    Expr::Lambda(v, e) => {
                        stats.beta_reductions += 1;
                        return reduce_to_nf(
                            &beta_reduction(e.as_ref(), v, &g, &mut vec![])?,
                            stats,
                        );
                    }
                    _ => bail!("Invalid operator for appv: {f}"),
                }
            }

            let l = reduce_to_nf(l.as_ref(), stats)?;
            let r = reduce_to_nf(r.as_ref(), stats)?;
            match (op, &l, &r) {
                (BinOp::Add, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() + n2.as_ref()).into()),
                    _ => bail!("Invalid operator for add:\nl = {l}\nr = {r}"),
                },
                (BinOp::Sub, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() - n2.as_ref()).into()),
                    _ => bail!("Invalid operator for sub: {op} {l} {r}"),
                },
                (BinOp::Mul, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() * n2.as_ref()).into()),
                    _ => bail!("Invalid operator for mul: {op} {l} {r}"),
                },
                (BinOp::Div, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() / n2.as_ref()).into()),
                    _ => bail!("Invalid operator for div: {op} {l} {r}"),
                },
                (BinOp::Mod, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() % n2.as_ref()).into()),
                    _ => bail!("Invalid operator for mod: {l} {r}"),
                },
                (BinOp::Lt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 < n2),
                    _ => bail!("Invalid operator for lt: \n{l} \n{r}"),
                },
                (BinOp::Gt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 > n2),
                    _ => bail!("Invalid operator for gt: {l} {r}"),
                },
                (BinOp::Eq, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 == n2),
                    (Expr::String(n1), Expr::String(n2)) => Expr::Bool(n1 == n2),
                    (Expr::Bool(n1), Expr::Bool(n2)) => Expr::Bool(n1 == n2),
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
                    (Expr::String(s1), Expr::String(s2)) => {
                        Expr::String((s1.as_ref().clone() + s2.as_ref()).into())
                    }
                    _ => bail!("Invalid operator for concat: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Take, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => Expr::String(
                        s.chars()
                            .take(n.as_ref().try_into().unwrap())
                            .collect::<String>()
                            .into(),
                    ),
                    _ => bail!("Invalid operator for take: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Drop, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => Expr::String(
                        s.chars()
                            .skip(n.as_ref().try_into().unwrap())
                            .collect::<String>()
                            .into(),
                    ),
                    _ => bail!("Invalid operator for drop: {op:?} {l:?} {r:?}"),
                },
                _ => unreachable!(),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = reduce_to_nf(cond.as_ref(), stats)?;
            match cond {
                Expr::Bool(true) => reduce_to_nf(th.as_ref(), stats)?,
                Expr::Bool(false) => reduce_to_nf(el.as_ref(), stats)?,
                _ => bail!("Invalid condition: {cond:?}"),
            }
        }
        // Expr::Lambda(v, e) => Expr::Lambda(*v, reduce(e.as_ref(), env)?.into()),
        _ => e.clone(),
    })
}

fn str_to_int(s: &str) -> BigInt {
    let s = encode_str(s).unwrap();
    let mut ret = BigInt::from(0);
    for c in s.chars() {
        ret = ret * 94 + decode_base94(c).unwrap();
    }
    ret
}

fn int_to_str(n: &BigInt) -> String {
    let zero = BigInt::from(0);

    let mut s = String::new();
    let mut n = n.clone();
    while n > zero {
        let n2 = &n / 94;
        let r: BigInt = &n - &n2 * 94;
        s.push(decode_char(encode_base94(r.try_into().unwrap()).unwrap()).unwrap());
        n = n2;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        assert_eq!(str_to_int("test"), 15818151.into());
        assert_eq!(int_to_str(&15818151.into()), "test");
    }

    #[test]
    fn language_test() {
        let expr: Expr = r#"? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I" B% U- I$ I# ? B= I" B% I( I$ ? B= U- I" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4"#.parse().unwrap();
        assert_eq!(
            eval(&expr).unwrap(),
            Expr::String(
                "Self-check OK, send `solve language_test 4w3s0m3` to claim points for it"
                    .to_string()
                    .into()
            )
        );
    }

    #[test]
    fn beta_reductions() {
        let expr: Expr = r#"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"#.parse().unwrap();
        let mut stats = Stats::default();
        assert_eq!(
            reduce_to_nf(&expr, &mut stats).unwrap(),
            Expr::Int(BigInt::from(16).into())
        );
        assert_eq!(stats.beta_reductions, 109);
    }

    //#[test]
    fn tail_call() {
        // (
        //     fix
        //     (fn f n r ->
        //         (if (== n 0) {
        //             r
        //         } else {
        //             (f (- n 1) (+ r 1))
        //         })
        //     )
        //     10000 0
        // )
        let expr: Expr = r#"B$ B$ B$ L& B$ L8 B$ v& B$ v8 v8 L8 B$ v& B$ v8 v8 L& L. L2 ? B= v. I! v2 B$ B$ v& B- v. I" B+ v2 I" I"-E I!"#.parse().unwrap();
        assert_eq!(eval(&expr).unwrap(), Expr::Int(BigInt::from(1000).into()));
    }
}
