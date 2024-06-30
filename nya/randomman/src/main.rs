use std::{
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::{bail, ensure, Context, Result};
use clap::{Parser, Subcommand};
use common::expr::{BinOp, Expr};
use rayon::prelude::*;

use crate::assembler::ToExpr;

#[macro_use]
mod assembler;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rng {
    Default,
    Better,
}

impl Rng {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "default" => Some(Self::Default),
            "better" => Some(Self::Better),
            _ => None,
        }
    }

    pub fn next(&self, mut state: u64) -> (Direction, u64) {
        let dir = match state >> 62 {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => unreachable!(),
        };
        match self {
            Self::Default => {
                state = ((state as u128).wrapping_mul(48271) % 18446744073709551557) as u64;
            }
            Self::Better => {
                // https://arxiv.org/abs/2001.05304v3
                state = state.wrapping_mul(0xd1342543de82ef95).wrapping_add(1);
            }
        }
        (dir, state)
    }

    pub fn expr(&self) -> Expr {
        match self {
            Self::Default => icfp! {
                (fn s -> (% (* s 48271) 18446744073709551557))
            },
            Self::Better => icfp! {
                (fn s -> (% (+ (* s 0xd1342543de82ef95) 1) 18446744073709551616))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => panic!("unknown direction: {c:?}"),
        }
    }
}

impl Direction {
    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Pill,
    LambdaMan,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            ' ' => Cell::Empty,
            '.' => Cell::Pill,
            'L' => Cell::LambdaMan,
            _ => panic!("unknown cell type: {c:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    field: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    pills: usize,
    pos: (usize, usize),
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut field: Vec<Vec<Cell>> = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();

        // Ensure the field is rectangular.
        let height = field.len();
        let width = field[0].len();
        ensure!(
            field.iter().all(|row| row.len() == width),
            "inconsistent line length"
        );

        // Count pills.
        let pills = field
            .iter()
            .flatten()
            .filter(|&&cell| cell == Cell::Pill)
            .count();

        // Find the lambdaman.
        let candidates: Vec<(usize, usize)> = field
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &cell)| {
                    if cell == Cell::LambdaMan {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();
        ensure!(candidates.len() == 1, "lambdaman must be exactly one");
        let pos = candidates[0];
        field[pos.0][pos.1] = Cell::Empty;

        Ok(Self {
            field,
            height,
            width,
            pills,
            pos,
        })
    }
}

impl Game {
    pub fn pills(&self) -> usize {
        self.pills
    }

    pub fn step(&mut self, dir: Direction) {
        let (di, dj) = dir.delta();
        let (ci, cj) = self.pos;
        let (ni, nj) = (ci as isize + di, cj as isize + dj);
        // Reject moves that go out of bounds.
        if ni < 0 || ni >= self.height as isize || nj < 0 || nj >= self.width as isize {
            return;
        }

        let (ni, nj) = (ni as usize, nj as usize);
        match self.field[ni][nj] {
            Cell::Wall => return,
            Cell::Empty => {}
            Cell::Pill => {
                self.field[ni][nj] = Cell::Empty;
                self.pills -= 1;
            }
            Cell::LambdaMan => unreachable!(),
        }
        self.pos = (ni, nj);
    }
}

fn find_problems_dir() -> Result<PathBuf> {
    let current_dir = Path::new(".").canonicalize()?;
    for dir in current_dir.ancestors() {
        if dir.join(".git").exists() {
            return Ok(dir.join("problems/lambdaman"));
        }
    }
    bail!("Must be run under a git repository");
}

fn load_game(problem_id: usize) -> Result<Game> {
    let problems_dir = find_problems_dir()?;
    let problem_path = problems_dir.join(format!("{}.txt", problem_id));
    let content = std::fs::read_to_string(problem_path)?;
    content.parse()
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Search {
        #[arg(long, default_value_t = 2)]
        stride: usize,

        #[arg(long, default_value = "default")]
        rng: String,

        problem_id: usize,
    },
    Compile {
        #[arg(long, default_value_t = 2)]
        stride: usize,

        #[arg(long, default_value = "default")]
        rng: String,

        problem_id: usize,

        seed: u64,
    },
}

fn search(problem_id: usize, stride: usize, rng_name: &str) -> Result<()> {
    println!("Searching seed for problem {problem_id} with stride {stride}...");

    let game = load_game(problem_id)?;
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;

    let best_pills = AtomicUsize::new(1000000);

    const CHUNK_SIZE: usize = 1000;
    const SEED_MAX: u64 = 1000000000;
    let steps = 1000000 / stride;

    for start in (1..SEED_MAX).step_by(CHUNK_SIZE) {
        let end = start + CHUNK_SIZE as u64 - 1;
        eprint!("{}...\r", end);

        (start..=end).into_par_iter().for_each(|seed| {
            let mut game = game.clone();
            let mut state = seed;

            for step in 1..=steps {
                let (dir, new_state) = rng.next(state);
                for _ in 0..stride {
                    game.step(dir);
                }
                state = new_state;
                if game.pills() == 0 {
                    eprintln!("seed {seed}: all pills eaten in {} steps!", step * stride);
                    best_pills.store(0, Ordering::SeqCst);
                    return;
                }
            }

            let pills = game.pills();
            if pills < best_pills.fetch_min(pills, Ordering::SeqCst) {
                eprintln!("seed {seed}: {pills} pills");
            }
        });
    }

    Ok(())
}

fn compile(problem_id: usize, seed: u64, stride: usize, rng_name: &str) -> Result<()> {
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;
    let rng_expr = rng.expr();

    let header = format!("solve lambdaman{problem_id} ");
    let seed = seed as u128;

    let step_expr = match stride {
        1 => icfp! { (d s) },
        2 => icfp! { (concat (d s) (d s)) },
        _ => bail!("unsupported stride: {stride}"),
    };

    // ***HELP ME***: Optimize this code.
    let expr = icfp! {
        let R = (#rng_expr) in
        let d = (fn s -> (take 1 (drop (/ s 4611686018427387904) "LUDR"))) in
        (concat (#header) (fix (fn f c s r ->
            (if (== c 0) {
                r
            } else {
                (f (- c 1) (R s) (concat r (#step_expr)))
            })
        ) 500000 (#seed) ""))
    };

    println!("{}", expr.encoded());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    match args.command {
        Command::Search {
            rng,
            stride,
            problem_id,
        } => search(problem_id, stride, &rng),
        Command::Compile {
            rng,
            stride,
            problem_id,
            seed,
        } => compile(problem_id, seed, stride, &rng),
    }
}
