use crate::compiler::{expr, parser::parse};

use super::{expr::Expr, parser::parse_str, program::Program};

pub fn compile_to_lambda(prog: String) -> anyhow::Result<Expr> {
    let define_z = parse_str(
        r#"(define (Z f) (
    (lambda (x) (f (lambda (y) ((x x) y))))
    (lambda (x) (f (lambda (y) ((x x) y))))
))
    "#,
    )?
    .exprs[0]
        .clone();

    let mut program = parse(prog.chars())?;

    let mut need_z = false;
    for expr in program.exprs.iter_mut() {
        expr.recude_define_params();
        expr.reduce_proc_params();
        need_z |= expr.update_recursive_define();
    }
    if need_z {
        program.exprs.insert(0, define_z);
    }
    let expr = program.get_res_as_single_lambda();

    // // while let Some(p) = update(program) {
    // //     program = p;
    // // }

    // println!("{:?}", expr);

    Ok(expr)
}

// struct Compiler {
//     program: Program,

//     fn trans_to_lambda(&mut self) {
//         self.program.exprs = self.program.exprs.into_iter().map(|expr| {
//             match expr {
//                 Expr::Proc { name, args } => {
//                     let mut args = args.into_iter().map(|arg| {
//                         match arg {
//                             Expr::Var(name) => name,
//                             _ => panic!("unexpected expr"),
//                         }
//                     }).collect();
//                     args.reverse();
//                     Expr::Lambda { name, args }
//                 },
//                 _ => panic!("unexpected expr"),
//             }
//         }).collect();
//     }
// }
