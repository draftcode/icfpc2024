use std::{
    fs,
    io::{self, Write},
};

use anyhow::{anyhow, bail, Result};

use common::planar::State;

#[argopt::subcmd]
fn resolve_label() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;
    let mut state = State::new(s.as_str(), 0, 0)?;
    state.resolve_label()?;
    println!("{}", common::planar::print_for_submit(&state));
    Ok(())
}

#[argopt::subcmd]
fn run(
    #[opt(short = 'p', long = "program")] program: std::path::PathBuf,
    #[opt(short = 't', long = "turn")] turn: Option<u32>,
) -> Result<()> {
    let s = fs::read_to_string(program)?;

    print!("Input A and B >>> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut a_and_b = vec![];
    for num in input.split_whitespace() {
        a_and_b.push(num.parse::<i32>()?);
    }

    let mut state = State::new(&s, a_and_b[0], a_and_b[1])?;

    println!("before label processing");
    println!("{}", state.board);
    state.resolve_label()?;
    println!("after label processing");
    println!("{}", state.board);

    let max_turn = if let Some(t) = turn { t } else { 1000000 };
    let mut turn = 0;
    while state.output.is_none() && turn < max_turn {
        state.onestep()?;
        println!("{}", state.board);
        turn += 1;
    }

    println!("finished {}", state.output.unwrap());
    Ok(())
}

#[argopt::cmd_group(commands = [resolve_label, run])]
fn main() -> Result<()> {}
