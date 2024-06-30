use std::{io, str::FromStr};

use anyhow::{anyhow, bail, Result};

use common::planar::{Cell, State};

fn parse_input(s: &str) -> Result<State> {
    if let Some((first, board)) = s.split_once("\n") {
        let first = first.split_whitespace().collect::<Vec<&str>>();
        if first.len() != 2 {
            bail!("Please put A and B in the first line");
        }
        let a = first[0].parse::<i32>()?;
        let b = first[1].parse::<i32>()?;
        State::new(board, a, b)
    } else {
        Err(anyhow!("Failed to parse the input"))
    }
}

fn main() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;

    let mut state = parse_input(s.as_str())?;

    println!("before label processing");
    println!("{}", state.board);
    state.resolve_label()?;
    println!("after label processing");
    println!("{}", state.board);

    let mut turn = 0;
    while state.output.is_none() {
        state.onestep()?;
        println!("{}", state.board);
        turn += 1;
    }

    println!("finished {}", state.output.unwrap());
    Ok(())
}
