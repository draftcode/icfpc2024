use crate::expr::{BinOp, Expr, UnOp};

pub fn cps_conversion(e: &Expr) -> Expr {
    cps_conv_internal(e)
}

fn cps_conv_internal(e: &Expr) -> Expr {
    log::trace!("cps_conv: {e}");

    match e {
        Expr::Bool(_) => e.clone(),
        Expr::Int(_) => e.clone(),
        Expr::String(_) => e.clone(),
        Expr::Var(v) => Expr::Lambda(
            200,
            Box::new(Expr::Bin(
                BinOp::AppV,
                Box::new(Expr::Var(200)),
                Box::new(Expr::Var(*v)),
            )),
        ),
        Expr::Un(_, _) => e.clone(),
        Expr::Bin(op, l, r) => {
            if matches!(op, BinOp::App) {
                let l = cps_conv_internal(l);
                let r = cps_conv_internal(r);
                Expr::Lambda(
                    100,
                    Box::new(Expr::Bin(
                        BinOp::AppV,
                        Box::new(l),
                        Box::new(Expr::Lambda(
                            101,
                            Box::new(Expr::Bin(
                                BinOp::AppV,
                                Box::new(r),
                                Box::new(Expr::Lambda(
                                    102,
                                    Box::new(Expr::Bin(
                                        BinOp::AppV,
                                        Box::new(Expr::Bin(
                                            BinOp::AppV,
                                            Box::new(Expr::Var(101)),
                                            Box::new(Expr::Var(102)),
                                        )),
                                        Box::new(Expr::Var(100)),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )
            } else {
                e.clone()
            }
        }
        Expr::If(_, _, _) => e.clone(),
        Expr::Lambda(v, e) => {
            let e = cps_conv_internal(e);
            let lin = Box::new(Expr::Lambda(*v, Box::new(e)));
            Expr::Lambda(
                100,
                Box::new(Expr::Bin(BinOp::AppV, Box::new(Expr::Var(100)), lin)),
            )
        }
    }
}
