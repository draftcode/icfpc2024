#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::needless_range_loop
)]

use anyhow::{anyhow, bail, Result};

#[argopt::subcmd]
fn hello() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[argopt::cmd_group(commands = [hello])]
fn main() -> Result<()> {}
