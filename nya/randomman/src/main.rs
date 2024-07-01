use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::{bail, ensure, Context, Result};
use clap::{Parser, Subcommand};
use common::expr::{Expr, Token};
use rayon::prelude::*;
use rng::Rng;
use simulate::{load_game, MAX_MOVES};

#[macro_use]
mod assembler;
pub mod rng;
pub mod simulate;

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
    CompileAll,
    SubmitAll,
}

fn search_main(problem_id: usize, stride: usize, rng_name: &str, start_seed: u64) -> Result<()> {
    eprintln!("Searching seed for problem {problem_id} with stride {stride}...");

    let game = load_game(problem_id)?;
    let rng = Rng::from_name(rng_name).context("unknown RNG name")?;

    eprintln!("Initial pills: {}", game.pills());

    let best_pills = AtomicUsize::new(game.pills());

    const CHUNK_SIZE: usize = 1000;
    const SEED_MAX: u64 = 1000000000;
    let steps = MAX_MOVES / stride;

    for start in (start_seed..SEED_MAX).step_by(CHUNK_SIZE) {
        let end = start + CHUNK_SIZE as u64 - 1;
        eprint!("{}...\r", end);

        let solved = AtomicBool::new(false);

        (start..=end).into_par_iter().for_each(|seed| {
            let mut game = game.clone();
            let mut state = if rng.skip_first_seed() {
                rng.next(seed).1
            } else {
                seed
            };

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
    // KnownSolution {
    //     // 167B
    //     problem_id: 4,
    //     rng: Rng::Default,
    //     seed: 29,
    //     stride: 1,
    //     moves: 17042,
    // },
    // KnownSolution {
    //     // 163B
    //     problem_id: 4,
    //     rng: Rng::DefaultRev,
    //     seed: 2,
    //     stride: 1,
    //     moves: 54181,
    // },
    KnownSolution {
        // 142B
        problem_id: 4,
        rng: Rng::SmallModRev,
        seed: 46,
        stride: 1,
        moves: 22091,
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
    KnownSolution {
        problem_id: 21,
        rng: Rng::Default,
        seed: 13229262,
        stride: 1,
        moves: 932995,
    },
];

fn compile_all_main() -> Result<()> {
    for known in KNOWN_SOLUTIONS {
        let expr =
            known
                .rng
                .compile_expr(known.problem_id, known.seed, known.stride, known.moves)?;
        let code = expr.encoded().to_string();
        eprintln!("lambdaman{}: {} bytes", known.problem_id, code.len());
    }
    Ok(())
}

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
        Command::CompileAll => compile_all_main(),
        Command::SubmitAll => submit_all_main(),
    }
}
