use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {}

fn main() -> Result<()> {
    let _args = Args::parse();

    println!("Hello, world!");
    Ok(())
}
