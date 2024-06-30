use anyhow::{bail, ensure};
use common::{
    compiler::{compile::compile_to_lambda, parser::parse},
    eval::eval,
    expr::Expr,
    lambdaman::map::LMap,
    optimize::optimize,
};
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

#[argopt::subcmd]
fn submit(
    #[opt(long, default_value = "")] mut api_token: String,
    #[opt(long)] nolambdaman: bool,
) -> anyhow::Result<()> {
    if api_token.is_empty() {
        api_token = get_api_token_from_env();
    }
    eprint!("> ");

    let mut input = "".to_string();

    std::io::stdin().read_to_string(&mut input)?;

    let expr = compile_to_lambda(input)?;

    let icfp = expr.icfp();

    let icfp_prog = icfp.join(" ");
    eprintln!("compiled ({} bytes): {}", icfp_prog.len(), icfp_prog);

    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://icfp-api.badalloc.com/communicate")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "text/plain")
        .body(icfp_prog)
        .send()?;

    ensure!(
        response.status().is_success(),
        "request failed: {}",
        response.status()
    );

    let text = response.text()?;
    eprintln!("{}", text);

    if let Ok(expr) = text.parse::<Expr>() {
        eprintln!("{}", expr);
        let expr = eval(&expr)?;
        if let Expr::String(s) = expr {
            println!("{}", s);
        } else {
            eprintln!("*** Failed to evaluate the response! ***");
        }
    }

    Ok(())
}

fn get_api_token_from_env() -> String {
    std::env::var("API_TOKEN").unwrap_or_default()
}

#[argopt::cmd_group(commands = [oneline, compile, submit])]
fn main() -> anyhow::Result<()> {}
