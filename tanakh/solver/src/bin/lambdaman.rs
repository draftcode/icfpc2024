use std::{io::Read, path::PathBuf};

use common::expr::{BinOp, Expr};
use num_bigint::BigInt;

fn compress(sol: &str) -> BigInt {
    let mut ret = BigInt::from(0);

    if sol.ends_with("L") {
        ret = ret * 4 + 1;
    }

    for c in sol.chars().rev() {
        let c = match c {
            'L' => 0,
            'R' => 1,
            'U' => 2,
            'D' => 3,
            _ => unreachable!(),
        };
        ret = &ret * 4 + c;
    }

    ret
}

trait ToExpr {
    fn to_expr(&self) -> Expr;
}

impl ToExpr for BigInt {
    fn to_expr(&self) -> Expr {
        Expr::Int(self.clone().into())
    }
}

impl ToExpr for String {
    fn to_expr(&self) -> Expr {
        Expr::String(self.clone().into())
    }
}

impl ToExpr for &str {
    fn to_expr(&self) -> Expr {
        Expr::String(self.to_string().into())
    }
}

impl ToExpr for i32 {
    fn to_expr(&self) -> Expr {
        Expr::Int(BigInt::from(*self).into())
    }
}

impl ToExpr for Expr {
    fn to_expr(&self) -> Expr {
        self.clone()
    }
}

macro_rules! icfp {
    (fix $($args:tt)+) => {
        icfp! {
            (fn f -> ((fn x -> (f (x x))) (fn x -> (f (x x))))) $($args)*
        }
    };
    (fn $($args:ident)+ -> $body:tt) => {{
        let mut e = icfp!{ $body };
        for arg in [$(stringify!($args)), *].iter().rev() {
            e = Expr::Lambda(varid(arg), Box::new(e));
        }
        e
    }};
    (let $var:ident = $val:tt in $($body:tt)+) => {
        Expr::Bin(BinOp::App,
            Box::new(Expr::Lambda(varid(stringify!($var)), Box::new(icfp!{ $($body)* }))),
            Box::new(icfp!{ $val }))
    };
    (if $cond:tt { $($th:tt)+ } else { $($el:tt)+ }) => {
        Expr::If(Box::new(icfp!{ $cond }), Box::new(icfp!{ $($th)* }), Box::new(icfp!{ $($el)* }))
    };
    (concat $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Concat, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (take $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Take, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (drop $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Drop, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (== $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Eq, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (/ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Div, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (% $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mod, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (# $var:ident) => {
        ToExpr::to_expr(&$var)
    };
    ($f:tt $($args:tt)+) => {{
        let mut e = icfp!{ $f };
        $(
            e = Expr::Bin(BinOp::App, Box::new(e), Box::new(icfp!{ $args }));
        )+
        e
    }};
    ($var:ident) => {
        Expr::Var(varid(stringify!($var)))
    };
    ($val:literal) => {
        ToExpr::to_expr(&$val)
    };
    (($($tt:tt)+)) => {
        icfp! { $($tt)+ }
    };
}

fn varid(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

fn problem6() -> Expr {
    icfp! {
        let x = "RRRRRRRRRRRRR" in
        let x = (concat (concat x x) (concat x x)) in
        (concat (concat x x) (concat x x))
    }
}

#[argopt::cmd]
fn main(pid: usize, path: PathBuf) {
    if pid == 6 {
        let code = problem6();
        let code = icfp! {
            (concat "solve lambdaman6 " (#code))
        };

        let tokens = code.to_tokens();
        let s = tokens
            .into_iter()
            .map(|token| token.encoded().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", s);
        return;
    }

    let s = std::fs::read_to_string(path).unwrap();
    let c = compress(s.trim());
    let header = format!("solve lambdaman{pid} ");

    let code = icfp! {
        fix (fn f c s ->
            (if (== c 0) {
                s
            } else {
                f (/ c 4) (concat s (take 1 (drop (% c 4) "LRUD")))
            })
        ) (#c) (#header)
    };

    eprintln!("{code}");

    let tokens = code.to_tokens();
    eprintln!("{:?}", tokens);

    let s = tokens
        .into_iter()
        .map(|token| token.encoded().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", s);
}
