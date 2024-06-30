use std::rc::Rc;

use common::{
    eval::eval,
    expr::{BinOp, Expr},
};
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

fn problem6() -> Expr {
    let header = "solve lambdaman6 ";

    // 90 bytes
    // B$ L8 B$ L8 B. S3/,6%},!-"$!-!.[} B. B. v8 v8 B. v8 v8 B. B. v8 v8 B. v8 v8 SLLLLLLLLLLLLL
    // B$
    //   L8
    //     B$
    //       L8
    //         B.
    //           S3/,6%},!-"$!-!.[}
    //           B.
    //             B.
    //               v8
    //               v8
    //             B.
    //               v8
    //               v8
    //       B.
    //         B.
    //           v8
    //           v8
    //         B.
    //           v8
    //           v8
    //   SLLLLLLLLLLLLL
    //
    icfp! {
        let x = "RRRRRRRRRRRRR" in
        let x = (concat (concat x x) (concat x x)) in
        (concat (#header) (concat (concat x x) (concat x x)))
    };

    // 87 bytes
    // B$ L0 B$ L8 B. S3/,6%},!-"$!-!.[} B$ v0 B$ v0 v8 SLLLLLLLLLLLLL L8 B. B. v8 v8 B. v8 v8
    icfp! {
        let p = (fn x -> (concat (concat x x) (concat x x))) in
        let x = "RRRRRRRRRRRRR" in
        (concat (#header) (p (p x)))
    };

    // 75 bytes
    // B$ L! B. S3/,6%},!-"$!-!.[} B$ v! B$ v! B$ v! SLLLL L! B. B. v! v! B. v! v!
    // B$
    //   L!
    //     B.
    //       S3/,6%},!-"$!-!.[}
    //       B$
    //         v!
    //         B$
    //           v!
    //           B$
    //             v!
    //             SLLLL
    //   L!
    //     B.
    //       B.
    //         v!
    //         v!
    //       B.
    //         v!
    //         v!
    icfp! {
        let f = (fn x -> (concat (concat x x) (concat x x))) in
        (concat
            (#header)
            (f (f (f "RRRR")))
        )
    };

    // 74 bytes
    // B$ L& B. S3/,6%},!-"$!-!.[} B$ v& B$ v& B$ v& B$ v& SLLL L8 B. v8 B. v8 v8
    icfp! {
        let f = (fn x -> (concat x (concat x x))) in
        (concat
            (#header)
            (f (f (f (f "RRR"))))
        )
    };

    // 73 bytes
    // B$ L& B. S3/,6%},!-"$!-!.[} B$ v& B$ v& B$ v& SLLLLLLLL L8 B. v8 B. v8 v8
    icfp! {
        let f = (fn x -> (concat x (concat x x))) in
        (concat
            (#header)
            (f (f (f "RRRRRRRR")))
        )
    }
}

fn problem19() -> Expr {
    let header = "solve lambdaman19 ";
    icfp! {
        let y = (fn f -> ((fn x -> (f (x x))) (fn x -> (f (x x))))) in
        (concat (#header)
            (
                (fn r ->
                    (
                        y (fn f n ->
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
                (y (fn f s n -> (if (== n 0) { s } else { (concat (f s (- n 1)) (f s (- n 1))) })))
            )
        )
    }
}

fn problem20() -> Expr {
    let header = "solve lambdaman20 ";
    icfp! {
        (concat (#header)
            (
                (fn r ->
                    (
                        fix (fn f n o ->
                            (if (== n 0) {
                                ""
                            } else {
                                (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat (concat
                                    (r "U" n)
                                    "R")
                                    (f (- n 1)))
                                    "L")
                                    (r "D" n))
                                    "L")
                                    (r "D" n))
                                    (f (- n 1)))
                                    (r "U" n))
                                    (r "L" n))
                                    "UR")
                                    (f (- n 1)))
                                    "D")
                                    (r "R" n))
                                    "D")
                                    (r "R" n))
                                    (f (- n 1)))
                                    (r "L" n))
                                    "U")
                            })
                        )
                        6
                        "x"
                    )
                )
                (fix (fn f s n -> (if (== n 0) { s } else { (concat (f s (- n 1)) (f s (- n 1))) })))
            )
        )
    }
}

fn main() {
    let expr = problem19();
    println!("{}", expr.encoded());
    eprintln!("{}", expr);
    eprintln!("{} bytes", expr.encoded().to_string().len());
    let Expr::String(s) = eval(&expr).unwrap() else {
        panic!("not a string")
    };
    //eprintln!("{}", s);
}