use anyhow::{bail, Result};
use common::compiler::parser::parse;
use std::{io::Read, str::Chars};

#[argopt::subcmd]
fn compile() -> anyhow::Result<()> {
    print!("> ");

    let mut input = "".to_string();
    std::io::stdin().read_to_string(&mut input)?;

    let program = parse(input.chars())?;

    println!("{:?}", program);

    Ok(())
}

#[argopt::cmd_group(commands = [compile])]
fn main() -> anyhow::Result<()> {}
