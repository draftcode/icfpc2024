use crate::compiler::parser::parse;

use super::{expr::Expr, parser::parse_str};

pub fn compile_to_lambda(prog: String) -> anyhow::Result<Expr> {
    let mut define_z = parse_str(
        //         r#"(define (Z f) (
        //     (lambda (x) (f (lambda (y) ((x x) y))))
        //     (lambda (x) (f (lambda (y) ((x x) y))))
        // ))
        //     "#,
        r#"(define (Z f) (
        (lambda (x) (f (x x)))
        (lambda (x) (f (x x)))
))
"#,
    )?
    .exprs[0]
        .clone();
    define_z.recude_define_params();
    define_z.reduce_proc_params();

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

    Ok(expr)
}
