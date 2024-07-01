use std::{path::PathBuf, rc::Rc};

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

impl ToExpr for i64 {
    fn to_expr(&self) -> Expr {
        Expr::Int(BigInt::from(*self).into())
    }
}

impl ToExpr for i128 {
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
    (+ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Add, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (- $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Sub, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (* $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mul, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
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
    let header = "solve lambdaman6 ";
    icfp! {
        let x = "RRRRRRRRRRRRR" in
        let x = (concat (concat x x) (concat x x)) in
        (concat (#header) (concat (concat x x) (concat x x)))
    }
}

fn problem8() -> Expr {
    let header = "solve lambdaman8 ";
    icfp! {
        fix (fn f c p q s ->
            (if (== c 50) {
              s
            } else {
              f (+ c 1)
                (concat "DD" (concat p "LL"))
                (concat "UU" (concat q "RR"))
                (concat s (if (== (% c 2) 1) { p } else { q }))
            })
        ) 0 "" "" (#header)
    }
}

fn problem9() -> Expr {
    let header = "solve lambdaman9 ";

    // icfp! {
    //     let r =
    //         (let s = "RRRRRRRRRRRR" in
    //             concat (concat (concat s s) (concat s s)) "RD") in
    //     let l =
    //         (let s = "LLLLLLLLLLLL" in
    //             concat (concat (concat s s) (concat s s)) "LD") in
    //     let p = (concat r l) in
    //     let q = (concat (concat p p) p) in
    //     let r = (concat q q) in
    //     let s = (concat r r) in
    //     (concat (#header) (concat (concat s s) p))
    // }

    icfp! {
        fix (fn f c p q s ->
            (if (== c 50) {
                s
            } else {
                f (+ c 1)
                    (concat "R" (concat p "U"))
                    (concat "D" (concat q "L"))
                    (concat s
                        (if (== (% c 2) 1) {
                            (concat p "R")
                        } else {
                            (concat q "D")
                        }))
            }))
            0 "" "" (#header)
    }
}

fn problem16() -> Expr {
    let header = "solve lambdaman16 ";
    icfp! {
        (concat (#header)
            (fix (fn f s d e g r ->
                (if (== s 0) {
                    ""
                } else {
                    let t = (/ (- s 1) 2) in
                    (concat
                        (concat (concat (f t e d r g) d)
                                (concat (f t d e g r) e))
                        (concat (concat (f t d e g r) g)
                                (f t r g e d)))
                })) 127 "DD" "RR" "UU" "LL"))
    }
}

fn problem11() -> Expr {
    icfp! {
        let R = (fn s -> (% (* s 48271) 18446744073709551557_i128)) in
        let d = (fn s -> (take 1 (drop (/ s 4611686018427387904_i128) "LUDR"))) in
        (concat "solve lambdaman11 " (fix (fn f c s r ->
            (if (== c 0) {
                r
            } else {
                (f (- c 1) (R s) (concat r (concat (d s) (d s))))
            })
        ) 500000 4610551 ""))
    }
}

// fn problem20() -> Expr {
//     icfp! {
//         let f = (fix (fn f n -> (
//             if (== n 0) {
//                 ""
//             } else {
//                 (* n (f (- n 1)))
//             }
//         )))
//         in (f(128))
//     }
// }

fn problem20(n: usize, u: char, r: char, d: char, l: char) -> String {
    if n == 0 {
        "".to_string()
    } else {
        let mut ret = String::new();

        for _ in 0..n {
            ret.push(u);
        }

        for _ in 0..3 {
            ret.push(l);
        }

        ret
    }
}

#[argopt::subcmd]
fn special(pid: usize) {
    let e = match pid {
        6 => problem6(),
        8 => problem8(),
        9 => problem9(),
        16 => problem16(),
        _ => unimplemented!(),
    };

    let tokens = e.to_tokens();
    let s = tokens
        .into_iter()
        .map(|token| token.encoded().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    eprintln!("{} bytes", s.len());
    println!("{}", s);
}

#[argopt::subcmd]
fn comp(pid: usize, path: PathBuf) {
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

#[argopt::cmd_group(commands = [special, comp])]
fn main() {}
