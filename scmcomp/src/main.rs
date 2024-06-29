use common::compiler::{compile::compile_to_lambda, parser::parse};
use std::io::Read;

#[argopt::subcmd]
fn oneline() -> anyhow::Result<()> {
    eprint!("> ");

    let mut input = "".to_string();
    std::io::stdin().read_to_string(&mut input)?;

    let expr = compile_to_lambda(input)?;

    println!(
        "(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(print {})",
        expr
    );

    Ok(())
}

#[argopt::subcmd]
fn compile() -> anyhow::Result<()> {
    eprint!("> ");

    let mut input = "".to_string();
    std::io::stdin().read_to_string(&mut input)?;

    let expr = compile_to_lambda(input)?;

    let icfp = expr.icfp();

    println!("{}", icfp.join(" "));

    Ok(())
}

#[argopt::cmd_group(commands = [oneline, compile])]
fn main() -> anyhow::Result<()> {}
