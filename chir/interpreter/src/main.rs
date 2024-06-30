use std::{
    collections::VecDeque,
    fs,
    io::{self, Write},
};

use anyhow::{anyhow, bail, Result};
use num_bigint::BigInt;

use common::planar::{Cell, State};

#[argopt::subcmd]
fn resolve_label() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;
    let mut state = State::new_with_input_port(s.as_str(), 0.into(), 0.into())?;
    state.resolve_label()?;
    println!("{}", common::planar::print_for_submit(&state));
    Ok(())
}

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, -1, 0, 1];

fn inside(board: &Vec<Vec<Cell>>, x: i32, y: i32) -> bool {
    x >= 0 && (x < board[0].len() as i32) && y >= 0 && y < board.len() as i32
}

fn is_operator(c: &Cell) -> bool {
    match c {
        Cell::Plus | Cell::Minus | Cell::Mul | Cell::Div | Cell::Eq | Cell::Neq | Cell::Warp(_) => {
            true
        }
        _ => false,
    }
}

fn bfs(
    board: &Vec<Vec<Cell>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> (i32, i32, i32, i32) {
    println!("bfs {},{}", x, y);
    let mut q = VecDeque::new();
    visited[y as usize][x as usize] = true;
    q.push_back((x as i32, y as i32));

    let mut min_x = x as i32;
    let mut min_y = y as i32;
    let mut max_x = x as i32;
    let mut max_y = y as i32;

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);

        match board[y as usize][x as usize] {
            Cell::Plus
            | Cell::Minus
            | Cell::Mul
            | Cell::Div
            | Cell::Rem
            | Cell::Eq
            | Cell::Neq
            | Cell::Warp(_) => {
                for i in 0..4 {
                    let nx = x + DX[i];
                    let ny = y + DY[i];
                    if !inside(board, nx, ny) {
                        continue;
                    }
                    if visited[ny as usize][nx as usize] {
                        continue;
                    }
                    visited[ny as usize][nx as usize] = true;
                    q.push_back((nx, ny));
                }
            }
            Cell::Number(_)
            | Cell::InputA
            | Cell::InputB
            | Cell::Submit
            | Cell::Empty
            | Cell::Label(_, _) => {
                let mut nx = x + 1;
                let mut ny = y + 1;
                // Check right
                if inside(board, nx, y)
                    && is_operator(&board[y as usize][nx as usize])
                    && !visited[y as usize][nx as usize]
                {
                    visited[y as usize][nx as usize] = true;
                    q.push_back((nx, y))
                }

                // Check down
                if inside(board, x, ny)
                    && is_operator(&board[ny as usize][x as usize])
                    && !visited[ny as usize][x as usize]
                {
                    visited[ny as usize][x as usize] = true;
                    q.push_back((x, ny));
                }
            }
            Cell::Up | Cell::Down => {
                if inside(board, x, y - 1) && !visited[(y - 1) as usize][x as usize] {
                    visited[(y - 1) as usize][x as usize] = true;
                    q.push_back((x, y - 1));
                }
                if inside(board, x, y + 1) && !visited[(y + 1) as usize][x as usize] {
                    visited[(y + 1) as usize][x as usize] = true;
                    q.push_back((x, y + 1));
                }
            }
            Cell::Left | Cell::Right => {
                if inside(board, x - 1, y) && !visited[y as usize][(x - 1) as usize] {
                    visited[y as usize][(x - 1) as usize] = true;
                    q.push_back((x - 1, y));
                }
                if inside(board, x + 1, y) && !visited[y as usize][(x + 1) as usize] {
                    visited[y as usize][(x + 1) as usize] = true;
                    q.push_back((x + 1, y));
                }
            }
        };
    }

    (min_x, min_y, max_x, max_y)
}

fn find_boxes(state: &State) -> Vec<(i32, i32, i32, i32)> {
    println!("find_boxes");
    let h = state.board.0.len();
    let w = state.board.0[0].len();

    let mut res = vec![];

    let mut visited = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            if visited[y][x] {
                continue;
            }
            res.push(bfs(&state.board.0, &mut visited, x, y));
        }
    }

    vec![(0, 0, 0, 0)]
}

#[argopt::subcmd]
fn placement() -> Result<()> {
    let s = io::read_to_string(io::stdin())?;
    let state = State::new_with_input_port(s.as_str(), 0.into(), 0.into())?;

    let boxes = find_boxes(&state);

    println!("boxes = {:?}", boxes);

    Ok(())
}

#[argopt::subcmd]
fn run(
    #[opt(short = 'p', long = "program")] program: std::path::PathBuf,
    #[opt(short = 't', long = "turn")] turn: Option<u32>,
    #[opt(short = 'd', long = "debug")] debug: bool,
) -> Result<()> {
    let s = fs::read_to_string(program)?;

    print!("Input A and B >>> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut a_and_b = vec![];
    for num in input.split_whitespace() {
        a_and_b.push(num.parse::<num_bigint::BigInt>()?);
    }

    let mut state = State::new(&s, a_and_b[0].clone(), a_and_b[1].clone())?;

    if debug {
        println!("before label processing");
        println!("{}", state.board);
    }
    state.resolve_label()?;
    if debug {
        println!("after label processing");
        println!("{}", state.board);
    }

    let max_turn = if let Some(t) = turn { t } else { 1000000 };
    let mut turn = 0;
    while state.output.is_none() && turn < max_turn {
        state.onestep()?;
        if debug {
            println!(
                "[time={},tick={},x={},y={}]",
                state.monotonic_tick,
                state.tick,
                state.used_x(),
                state.used_y()
            );
            println!("{}", state.board);
        }
        turn += 1;
    }

    let score = state.score();
    println!(
        "finished {}, score = {}, time = {}",
        state.output.unwrap(),
        score,
        state.monotonic_tick
    );
    Ok(())
}

#[argopt::cmd_group(commands = [resolve_label, run, placement])]
fn main() -> Result<()> {}
