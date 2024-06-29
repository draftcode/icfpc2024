use std::{str::FromStr, io};

use anyhow::{anyhow, bail, Result};

use common::planar::{Cell, Board, State};


fn main() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;

    let mut it = s.lines();
    let first = it.next().unwrap().split_whitespace().collect::<Vec<&str>>();

    let mut state: State = Default::default();
    state.input_a = first[0].parse::<i32>()?;
    state.input_b = first[1].parse::<i32>()?;

    for l in it {
        let mut row = vec![];
        for c in l.split_whitespace() {
            row.push(Cell::from_str(c)?);
        }
        state.board.0.push(row);
    }

    println!("{}", state.board);

    let mut turn = 0;
    while state.output.is_none() {
        state.onestep()?;
        // println!("{}", state.board);
        turn += 1;
    }

    println!("finished {}", state.output.unwrap());
    Ok(())
}
