use std::io::{BufRead, BufReader, Write as _};
use std::process::ExitCode;

use anyhow::{ensure, Result};
use clap::Parser;
use expr::{Expr, Token};

mod expr;

fn get_api_token_from_env() -> String {
    std::env::var("API_TOKEN").unwrap_or_default()
}

fn get_api_url_from_env() -> String {
    std::env::var("API_URL").unwrap_or("https://icfp-api.badalloc.com/communicate".to_string())
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

    let stdin = std::io::stdin().lock();
    let mut stdin = BufReader::new(stdin);
    let mut stdout = std::io::stdout().lock();
    let client = reqwest::blocking::Client::new();

    if let Some(request) = args.request {
        let input_expr = Expr::String(request);

        let response = client
            .post(args.api_url)
            .header("Authorization", format!("Bearer {}", args.api_token))
            .header("Content-Type", "text/plain")
            .body(input_expr.to_string())
            .send()?;
        ensure!(
            response.status().is_success(),
            "request failed: {}",
            response.status()
        );
        let text = response.text()?;

        let tokens = expr::tokenize(&text)?;
        if tokens.len() == 1 {
            if let Token::String(s) = &tokens[0] {
                writeln!(&mut stdout, "{}", s)?;
                return Ok(ExitCode::SUCCESS);
            }
        }

        eprintln!("*** Failed to evaluate the response! Printing the raw response. ***");
        writeln!(stdout, "{}", text)?;
        return Ok(ExitCode::FAILURE);
    }

    loop {
        write!(&mut stdout, "> ")?;
        stdout.flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }

        let input_expr = Expr::String(line.trim().to_string());

        let response = client
            .post("https://boundvariable.space/communicate")
            .header("Authorization", format!("Bearer {}", args.api_token))
            .body(input_expr.to_string())
            .send()?;
        ensure!(
            response.status().is_success(),
            "request failed: {}",
            response.status()
        );
        let text = response.text()?;
        writeln!(stdout, "{}", text)?;

        let tokens = expr::tokenize(&text)?;
        if tokens.len() == 1 {
            if let Token::String(s) = &tokens[0] {
                writeln!(&mut stdout, "{}", s)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
