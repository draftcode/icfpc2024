use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{bail, ensure, Result};
use clap::Parser;
use common::eval::eval;
use common::expr::{Expr, Token};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn get_api_token_from_env() -> String {
    std::env::var("API_TOKEN").unwrap_or_default()
}

fn get_api_url_from_env() -> String {
    std::env::var("API_URL").unwrap_or("https://icfp-api.badalloc.com/communicate".to_string())
}

fn find_history_file() -> Result<PathBuf> {
    let mut current_dir = Path::new(".").canonicalize().unwrap();
    for dir in current_dir.ancestors() {
        if dir.join(".git").exists() {
            return Ok(dir.join(".communicate.history"));
        }
    }
    bail!("Must be run under a git repository");
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = get_api_token_from_env())]
    api_token: String,

    #[arg(long, default_value_t = get_api_url_from_env())]
    api_url: String,

    request: Option<String>,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    if args.api_token.is_empty() {
        eprintln!("API token it not set. Set $API_TOKEN or pass --api-token");
        return Ok(ExitCode::FAILURE);
    }

    let client = reqwest::blocking::Client::new();

    if let Some(request) = args.request {
        let input_expr = Token::String(request);

        let response = client
            .post(&args.api_url)
            .header("Authorization", format!("Bearer {}", args.api_token))
            .header("Content-Type", "text/plain")
            .body(input_expr.encoded().to_string())
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
                return Ok(ExitCode::FAILURE);
            }
        }

        return Ok(ExitCode::SUCCESS);
    }

    let history_path = find_history_file()?;

    let mut rl = DefaultEditor::new()?;
    rl.load_history(&history_path).ok();

    loop {
        let line = match rl.readline(">>> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Err(err.into()),
        };
        rl.add_history_entry(&line)?;

        let input_token = Token::String(line.trim().to_string());

        let response = client
            .post(&args.api_url)
            .header("Authorization", format!("Bearer {}", args.api_token))
            .header("Content-Type", "text/plain")
            .body(input_token.encoded().to_string())
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
    }

    rl.save_history(&history_path)?;

    Ok(ExitCode::SUCCESS)
}
