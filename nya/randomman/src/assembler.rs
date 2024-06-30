use common::expr::Expr;
use num_bigint::BigInt;

pub(crate) trait ToExpr {
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

impl ToExpr for u128 {
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
            e = Expr::Lambda(crate::assembler::varid(arg), Rc::new(e));
        }
        e
    }};
    (let $var:ident = $val:tt in $($body:tt)+) => {
        Expr::Bin(BinOp::AppL,
            Rc::new(Expr::Lambda(crate::assembler::varid(stringify!($var)), Rc::new(icfp!{ $($body)* }))),
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
            e = Expr::Bin(BinOp::AppL, Rc::new(e), Rc::new(icfp!{ $args }));
        )+
        e
    }};
    ($var:ident) => {
        Expr::Var(crate::assembler::varid(stringify!($var)))
    };
    ($val:literal) => {
        ToExpr::to_expr(&$val)
    };
    (($($tt:tt)+)) => {
        icfp! { $($tt)+ }
    };
}

pub(crate) fn varid(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'A' as usize
}
