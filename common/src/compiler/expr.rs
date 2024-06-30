use std::collections::BTreeMap;

use crate::base94::{encode_base94, encode_base94_int, encode_str};

use super::icfp::{binary_op, unary_op};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Lambda(String, Box<Expr>),
    Proc(Vec<Expr>),
    Str(String),
    Num(i32),
    Var(String),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Lambda(name, expr) => write!(f, "(lambda ({}) {})", name, expr),
            Expr::Proc(args) => {
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Str(s) => write!(f, "\"{}\"", s),
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Var(v) => write!(f, "{}", v),
        }
    }
}

impl Expr {
    pub fn lambda(name: String, expr: Expr) -> Expr {
        Expr::Lambda(name, Box::new(expr))
    }

    pub fn define(name: String, var: String, expr: Expr) -> Expr {
        Expr::Proc(vec![
            Expr::Var("define".to_string()),
            Expr::Proc(vec![Expr::Var(name), Expr::Var(var)]),
            expr,
        ])
    }

    pub fn proc2(x: Expr, y: Expr) -> Expr {
        Expr::Proc(vec![x, y])
    }

    pub fn proc3(x: Expr, y: Expr, z: Expr) -> Expr {
        Expr::Proc(vec![x, y, z])
    }

    fn is_variadic(&self) -> bool {
        let Expr::Var(v) = self else { return false };

        v == "string-append"
    }

    pub fn is_buildin_var(&self, check_arity: Option<usize>) -> bool {
        let Expr::Var(v) = self else { return false };

        if v == "define" {
            return true;
        }
        let mut a = vec![];
        if v == "if" {
            a.push(3usize);
        }
        if binary_op(v).is_some() {
            a.push(2usize);
        }
        if unary_op(v).is_some() {
            a.push(1usize);
        }
        if a.is_empty() {
            return false;
        }

        if let Some(ari) = check_arity {
            if !a.contains(&ari) {
                panic!("{} should not have arity {}", v, ari);
            }
        }

        return true;
    }

    pub fn must_var(&self) -> String {
        match self {
            Expr::Var(v) => v.to_owned(),
            _ => panic!("not a var {}", self),
        }
    }

    pub fn must_proc(&self) -> Vec<Expr> {
        match self {
            Expr::Proc(args) => args.clone(),
            _ => panic!("not a proc"),
        }
    }

    fn is_define(&self) -> bool {
        match self {
            Expr::Proc(args) => args[0] == Expr::Var("define".to_string()),
            _ => false,
        }
    }

    fn define_vars(&self) -> Vec<String> {
        match self {
            Expr::Proc(args) => args.iter().map(|arg| arg.must_var()).collect(),
            _ => vec![],
        }
    }

    pub fn get_define(&self) -> Option<(Vec<String>, Expr)> {
        if !self.is_define() {
            return None;
        }
        match self {
            Expr::Proc(args) => {
                let [_, first, second] = args.as_slice() else {
                    panic!("define should have 2 args: {:?}", args)
                };

                let vars = first.define_vars();
                return Some((vars, second.clone()));
            }
            _ => None,
        }
    }

    pub fn recude_define_params(&mut self) {
        let Some((mut vars, expr)) = self.get_define() else {
            return;
        };
        if vars.len() <= 2 || Expr::Var(vars[0].clone()).is_buildin_var(None) {
            return;
        }
        let last_var = vars.pop().unwrap();
        let lambda = Expr::lambda(last_var, expr);
        let new_proc = Expr::Proc(vec![
            Expr::Var("define".to_string()),
            Expr::Proc(vars.into_iter().map(Expr::Var).collect()),
            lambda,
        ]);
        *self = new_proc;

        self.recude_define_params();
    }

    pub fn reduce_proc_params(&mut self) {
        match self {
            Expr::Proc(args) => {
                args.iter_mut().for_each(Expr::reduce_proc_params);

                // Special-case string-append
                if args.len() > 3 && args[0].is_variadic() {
                    let lst = args.pop().unwrap();
                    let before_lst = args.pop().unwrap();
                    let new_lst = Expr::Proc(vec![args[0].clone(), before_lst, lst]);
                    args.push(new_lst);
                    self.reduce_proc_params();
                    return;
                } else if args.len() <= 2 || args[0].is_buildin_var(None) {
                    return;
                }
                let lst = args.pop().unwrap();
                let l = Expr::Proc(args.clone());
                *self = Expr::Proc(vec![l, lst]);
                self.reduce_proc_params();
            }
            Expr::Lambda(_, expr) => {
                expr.reduce_proc_params();
            }
            _ => {}
        }
    }

    // appears as a free variable?
    pub fn is_free(&self, name: &str) -> bool {
        match self {
            Expr::Var(v) => {
                if v == name {
                    true
                } else {
                    false
                }
            }
            Expr::Proc(args) => args.iter().any(|arg| arg.is_free(name)),
            Expr::Lambda(new_name, expr) => {
                if name == new_name {
                    false
                } else {
                    expr.is_free(name)
                }
            }
            _ => false,
        }
    }

    // returns need of Z
    pub(crate) fn update_recursive_define(&mut self) -> bool {
        let Some((args, expr)) = self.get_define() else {
            return false;
        };
        if args.len() != 2 {
            return false;
        }
        // assert_eq!(args.len(), 2);
        let f = &args[0];
        let x = &args[1];
        if !expr.is_free(f) {
            return false;
        }

        // (define (f x) expr)
        // => (define (f x) ((Z (lambda (f) ((lambda (x) expr)))) x))

        *self = Self::define(
            f.to_string(),
            x.to_string(),
            Expr::proc2(
                Expr::proc2(
                    Expr::Var("Z".to_string()),
                    Expr::lambda(f.to_string(), Expr::lambda(x.to_string(), expr.clone())),
                ),
                Expr::Var(x.to_string()),
            ),
        );

        true
    }

    pub fn icfp(&self) -> Vec<String> {
        let mut res = vec![];
        let mut env = BTreeMap::new();
        self.icfp_inner(&mut res, &mut env, false);
        res
    }

    fn icfp_inner(&self, res: &mut Vec<String>, env: &mut BTreeMap<String, i32>, unary: bool) {
        match self {
            Expr::Var(k) => {
                if unary {
                    if let Some(u) = unary_op(k) {
                        res.push(u);
                        return;
                    }
                }
                if let Some(b) = binary_op(k) {
                    res.push(b);
                    return;
                }
                if k == "if" {
                    res.push("?".to_string());
                    return;
                }
                assert_ne!(k, "lambda", "lambda should be reduced");

                let v = env.get(k).expect(format!("not found {}", k).as_str());
                res.push(format!(
                    "v{}",
                    encode_base94(*v as i64).expect("more than 94 vars")
                ));
                return;
            }
            Expr::Proc(args) => {
                if args.len() == 1 {
                    // (f) => f
                } else if !args[0].is_buildin_var(Some(args.len() - 1)) {
                    assert_eq!(args.len(), 2, "{}", self);
                    res.push("B$".to_string());
                }
                args.iter()
                    .for_each(|arg| arg.icfp_inner(res, env, args.len() == 2))
            }
            Expr::Lambda(name, expr) => {
                let new_num = (0..).find(|i| !env.values().any(|v| v == i)).unwrap();
                // let new_num = *ii;
                // *ii += 1;

                res.push(format!(
                    "L{}",
                    encode_base94(new_num as i64).expect("more than 94 vars")
                ));

                let orig = env.get(name).map(|v| *v).clone();

                env.insert(name.to_string(), new_num);

                expr.icfp_inner(res, env, false);

                if let Some(orig) = orig {
                    env.insert(name.to_string(), orig);
                } else {
                    env.remove(name);
                }
            }
            Expr::Str(s) => res.push("S".to_string() + &encode_str(s).unwrap()),
            Expr::Num(n) => {
                res.push(format!("I{}", encode_base94_int(*n as i64).unwrap()));
            }
        }
    }
}
