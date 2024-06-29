use std::fmt::Display;

use super::expr::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
    pub exprs: Vec<Expr>,
}
impl Program {
    pub fn get_res_as_single_lambda(&self) -> Expr {
        self.clone().get_res_as_single_lambda_inner()
    }

    pub(crate) fn get_res_as_single_lambda_inner(&mut self) -> Expr {
        let lst = self.exprs.pop().expect("(define (res) ... ) not found");
        let Some((args, expr)) = lst.get_define() else {
            return self.get_res_as_single_lambda_inner();
        };
        if !(args.len() == 1 && args[0] == "res") {
            return self.get_res_as_single_lambda_inner();
        }

        self.make_single_lambda(expr)
    }

    fn make_single_lambda(&mut self, expr: Expr) -> Expr {
        let Some(lst) = self.exprs.pop() else {
            return expr;
        };

        let Some((args, def)) = lst.get_define() else {
            return self.make_single_lambda(expr);
        };

        // (define (name) def) expr = ((lambda (name) expr) def)
        if let [name] = args.as_slice() {
            let nxt_expr = Expr::proc2(Expr::lambda(name.to_string(), expr), def);
            return self.make_single_lambda(nxt_expr);
        }

        let [name, var] = args.as_slice() else {
            return self.make_single_lambda(expr);
        };
        let nxt_expr = Expr::proc2(
            Expr::lambda(name.to_string(), expr),
            Expr::lambda(var.to_string(), def),
        );
        self.make_single_lambda(nxt_expr)
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, expr) in self.exprs.iter().enumerate() {
            if i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", expr)?;
        }
        Ok(())
    }
}
