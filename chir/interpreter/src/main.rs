use std::{str::FromStr, io};

use anyhow::{anyhow, bail, Result};

use common::planar::{Cell, Board, State};


fn main() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;

    let mut state: State = Default::default();
    for l in s.lines() {
        let mut row = vec![];
        for c in l.split_whitespace() {
            row.push(Cell::from_str(c)?);
        }
        state.board.0.push(row);
    }

    state.input_a = 5;
    state.input_b = 5;

    println!("{:?}", state);

    println!("{}", state.board);

    for _ in 0..3 {
        state.onestep()?;
        println!("{}", state.board);
    }
    Ok(())
}
