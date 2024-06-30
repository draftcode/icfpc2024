use std::rc::Rc;

use common::expr::{BinOp, Expr};
use num_bigint::BigInt;

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
            e = Expr::Lambda(varid(arg), Rc::new(e));
        }
        e
    }};
    (let $var:ident = $val:tt in $($body:tt)+) => {
        Expr::Bin(BinOp::App,
            Rc::new(Expr::Lambda(varid(stringify!($var)), Rc::new(icfp!{ $($body)* }))),
            Rc::new(icfp!{ $val }))
    };
    (if $cond:tt { $($th:tt)+ } else { $($el:tt)+ }) => {
        Expr::If(Rc::new(icfp!{ $cond }), Rc::new(icfp!{ $($th)* }), Rc::new(icfp!{ $($el)* }))
    };
    (concat $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Concat, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (take $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Take, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (drop $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Drop, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (== $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Eq, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (< $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Lt, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (+ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Add, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (- $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Sub, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (* $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mul, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (/ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Div, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (% $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mod, Rc::new(icfp!{ $a1 }), Rc::new(icfp!{ $a2 }))
    };
    (# $var:ident) => {
        ToExpr::to_expr(&$var)
    };
    ($f:tt $($args:tt)+) => {{
        let mut e = icfp!{ $f };
        $(
            e = Expr::Bin(BinOp::App, Rc::new(e), Rc::new(icfp!{ $args }));
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

fn problem19() -> Expr {
    let header = "solve lambdaman19 ";
    icfp! {
        (concat (#header)
            (
                (fn r ->
                    (
                        fix (fn f n ->
                            (if (< n 0) {
                                ""
                            } else {
                                (concat (concat (concat (concat (concat (concat (concat (concat (concat
                                    (r "D" n)
                                    (f (- n 1)))
                                    (r "U" (+ n 1)))
                                    (f (- n 1)))
                                    (r "D" n))
                                    (r "L" n))
                                    (f (- n 1)))
                                    (r "R" (+ n 1)))
                                    (f (- n 1)))
                                    (r "L" n))
                            })
                        )
                        6
                    )
                )
                (fix (fn f s n -> (if (== n 0) { s } else { (concat (f s (- n 1)) (f s (- n 1))) })))
            )
        )
    }
}

fn main() {
    println!("{}", problem19().encoded());
}
