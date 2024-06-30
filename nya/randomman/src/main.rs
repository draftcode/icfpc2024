use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use common::expr::{BinOp, Expr};
use std::rc::Rc;
use util::{do_submit, search_main, Rng, MAX_MOVES};

use crate::assembler::ToExpr;

#[macro_use]
mod assembler;
pub mod util;

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

fn compile_main(
    problem_id: usize,
    seed: u64,
    stride: usize,
    moves: usize,
    rng_name: &str,
) -> Result<()> {
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;
    let expr = rng.compile_expr(problem_id, seed, stride, moves)?;
    println!("{}", expr.encoded());
    eprintln!("({} bytes)", expr.encoded().to_string().len());
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
    let expr = rng.compile_expr(problem_id, seed, stride, moves)?;
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
        let expr =
            known
                .rng
                .compile_expr(known.problem_id, known.seed, known.stride, known.moves)?;
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
