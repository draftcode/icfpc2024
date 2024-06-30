use std::{
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use anyhow::{bail, ensure, Context, Result};
use clap::{Parser, Subcommand};
use common::expr::{BinOp, Expr, Token};
use rayon::prelude::*;

use crate::assembler::ToExpr;

#[macro_use]
mod assembler;

const MAX_MOVES: usize = 1_000_000;

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
        // RNG expression takes `s` as an argument.
        match self {
            Self::Default => icfp! {
                (% (* s 48271) 18446744073709551557)
            },
            Self::Better => icfp! {
                (% (+ (* s 0xd1342543de82ef95) 1) 18446744073709551616)
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
        #[arg(long, required = true)]
        stride: usize,

        #[arg(long, default_value = "default")]
        rng: String,

        #[arg(long, default_value_t = 1)]
        start_seed: u64,

        problem_id: usize,
    },
    Compile {
        #[arg(long, required = true)]
        stride: usize,

        #[arg(long, default_value_t = MAX_MOVES)]
        moves: usize,

        #[arg(long, default_value = "default")]
        rng: String,

        problem_id: usize,

        seed: u64,
    },
    Submit {
        #[arg(long, required = true)]
        stride: usize,

        #[arg(long, default_value_t = MAX_MOVES)]
        moves: usize,

        #[arg(long, default_value = "default")]
        rng: String,

        problem_id: usize,

        seed: u64,
    },
    SubmitAll,
}

fn search_main(problem_id: usize, stride: usize, rng_name: &str, start_seed: u64) -> Result<()> {
    println!("Searching seed for problem {problem_id} with stride {stride}...");

    let game = load_game(problem_id)?;
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;

    let best_pills = AtomicUsize::new(1000000);

    const CHUNK_SIZE: usize = 1000;
    const SEED_MAX: u64 = 1000000000;
    let steps = MAX_MOVES / stride;

    for start in (start_seed..SEED_MAX).step_by(CHUNK_SIZE) {
        let end = start + CHUNK_SIZE as u64 - 1;
        eprint!("{}...\r", end);

        let solved = AtomicBool::new(false);

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
                    solved.store(true, Ordering::SeqCst);
                    return;
                }
            }

            let pills = game.pills();
            if pills < best_pills.fetch_min(pills, Ordering::SeqCst) {
                eprintln!("seed {seed}: {pills} pills");
            }
        });

        if solved.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}

fn compile_expr(
    problem_id: usize,
    seed: u64,
    stride: usize,
    moves: usize,
    rng: &Rng,
) -> Result<Expr> {
    let rng_expr = rng.expr();

    let header = format!("solve lambdaman{problem_id} ");
    let seed = seed as u128;

    let steps = (moves / stride) as u128;

    let mut seeds = vec![seed as u64];
    for _ in 1..=steps {
        let (_, new_seed) = rng.next(*seeds.last().unwrap());
        seeds.push(new_seed);
    }
    let last_seed = seeds.pop().unwrap();
    if seeds.contains(&last_seed) {
        bail!("seed cycle detected");
    }
    let last_seed = last_seed as u128;

    let step_expr = match stride {
        1 => icfp! { (take 1 (drop (/ s 4611686018427387904) "LUDR")) },
        2 => icfp! { (take 2 (drop (* (/ s 4611686018427387904) 2) "LLUUDDRR")) },
        _ => bail!("unsupported stride: {stride}"),
    };

    // ***HELP ME***: Optimize this code.
    let expr = icfp! {
        (concat (#header) (fix (fn f s ->
            (if (== s (#last_seed)) {
                ""
            } else {
                (concat (#step_expr) (f (#rng_expr)))
            })
        ) (#seed)))
    };
    Ok(expr)
}

fn compile_main(
    problem_id: usize,
    seed: u64,
    stride: usize,
    moves: usize,
    rng_name: &str,
) -> Result<()> {
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;
    let expr = compile_expr(problem_id, seed, stride, moves, &rng)?;
    println!("{}", expr.encoded());
    eprintln!("({} bytes)", expr.encoded().to_string().len());
    Ok(())
}

fn do_submit(problem_id: usize, expr: &Expr) -> Result<()> {
    let api_token = std::env::var("API_TOKEN").context("API_TOKEN is not set")?;

    eprintln!(
        "lambdaman{problem_id}: submitting {}B solution...",
        expr.encoded().to_string().len()
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://icfp-api.badalloc.com/communicate")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "text/plain")
        .body(expr.encoded().to_string())
        .send()?;
    ensure!(
        response.status().is_success(),
        "request failed: {}",
        response.status()
    );

    let raw = response.text()?;
    let Ok(Token::String(text)) = raw.parse() else {
        bail!("Failed to parse response: {raw}");
    };

    eprintln!("lambdaman{problem_id}: {text}");
    Ok(())
}

fn submit_main(
    problem_id: usize,
    seed: u64,
    stride: usize,
    moves: usize,
    rng_name: &str,
) -> Result<()> {
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;
    let expr = compile_expr(problem_id, seed, stride, moves, &rng)?;
    do_submit(problem_id, &expr)?;
    Ok(())
}

struct KnownSolution {
    problem_id: usize,
    rng: Rng,
    seed: u64,
    stride: usize,
    moves: usize,
}

// ***HELP ME***: Please add known solutions here.
const KNOWN_SOLUTIONS: &[KnownSolution] = &[
    // 1-3 have shorter solutions
    KnownSolution {
        problem_id: 4,
        rng: Rng::Default,
        seed: 29,
        stride: 1,
        moves: 17042,
    },
    // TODO: 5 can be revisited if code gets shorter
    // 6 has a shorter solution
    KnownSolution {
        problem_id: 7,
        rng: Rng::Default,
        seed: 298,
        stride: 1,
        moves: 17860,
    },
    // 8-9 have shorter solutions
    KnownSolution {
        problem_id: 10,
        rng: Rng::Default,
        seed: 1,
        stride: 1,
        moves: 53394,
    },
    KnownSolution {
        problem_id: 11,
        rng: Rng::Default,
        seed: 4610551,
        stride: 2,
        moves: MAX_MOVES,
    },
    KnownSolution {
        problem_id: 12,
        rng: Rng::Default,
        seed: 663880,
        stride: 2,
        moves: MAX_MOVES,
    },
    KnownSolution {
        problem_id: 13,
        rng: Rng::Default,
        seed: 217404,
        stride: 2,
        moves: MAX_MOVES,
    },
    KnownSolution {
        problem_id: 14,
        rng: Rng::Default,
        seed: 35975,
        stride: 2,
        moves: MAX_MOVES,
    },
    KnownSolution {
        problem_id: 15,
        rng: Rng::Default,
        seed: 1663183,
        stride: 2,
        moves: MAX_MOVES,
    },
    // 16 is hard
    KnownSolution {
        problem_id: 17,
        rng: Rng::Default,
        seed: 9,
        stride: 1,
        moves: MAX_MOVES,
    },
    KnownSolution {
        problem_id: 18,
        rng: Rng::Default,
        seed: 288180,
        stride: 1,
        moves: MAX_MOVES,
    },
    // 19 is hard
    // 20 is hard
    // TODO: 21 looks solvable
];

fn submit_all_main() -> Result<()> {
    for known in KNOWN_SOLUTIONS {
        let expr = compile_expr(
            known.problem_id,
            known.seed,
            known.stride,
            known.moves,
            &known.rng,
        )?;
        do_submit(known.problem_id, &expr)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    match args.command {
        Command::Search {
            rng,
            stride,
            start_seed,
            problem_id,
        } => search_main(problem_id, stride, &rng, start_seed),
        Command::Compile {
            rng,
            stride,
            moves,
            problem_id,
            seed,
        } => compile_main(problem_id, seed, stride, moves, &rng),
        Command::Submit {
            rng,
            stride,
            moves,
            problem_id,
            seed,
        } => submit_main(problem_id, seed, stride, moves, &rng),
        Command::SubmitAll => submit_all_main(),
    }
}
