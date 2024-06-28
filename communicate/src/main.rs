use std::io::{BufRead, BufReader, Write as _};
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

mod tanakh_copipe;

fn get_api_token_from_env() -> String {
    std::env::var("API_TOKEN").unwrap_or_default()
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value_t = get_api_token_from_env())]
    api_token: String,
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

    loop {
        write!(&mut stdout, "> ")?;
        stdout.flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }

        let input_expr = tanakh_copipe::Expr::String(line.trim().to_string());

        let response = client
            .post("https://boundvariable.space/communicate")
            .header("Authorization", format!("Bearer {}", args.api_token))
            .body(input_expr.to_string())
            .send()?;
        let text = response.text()?;
        writeln!(stdout, "{}", text)?;

        let tokens = tanakh_copipe::tokenize(&text)?;
        if tokens.len() == 1 {
            if let tanakh_copipe::Token::String(s) = &tokens[0] {
                writeln!(&mut stdout, "{}", s)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
