use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value_t = 1)]
    times: usize,

    #[clap(required = true)]
    world: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 0..args.times {
        println!("Hello, {}!", &args.world);
    }
    Ok(())
}
