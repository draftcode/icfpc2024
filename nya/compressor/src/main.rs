use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use common::expr::Expr;

#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}

fn compress(text: &str) -> Expr {
    todo!()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let text = std::fs::read_to_string(args.file)?.trim().to_string();
    let expr = compress(&text);
    println!("{}", expr.encoded());
    Ok(())
}
