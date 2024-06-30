use std::io::Read;

use anyhow::{bail, Result};
use common::lambdaman::map::LMap;

#[argopt::cmd_group(commands = [run])]
fn main() -> anyhow::Result<()> {}

#[argopt::subcmd]
fn run(#[opt(short, long)] out_dir: Option<String>) -> Result<()> {
    // let out_dir = out_dir.unwrap_or_else(|| {
    //     eprintln!("Using /tmp/lm as the default output directory. (can change with -o)");
    //     "/tmp/lm".to_string()
    // });

    let mut input = "".to_string();
    std::io::stdin().read_to_string(&mut input)?;

    let input = input.trim().to_string();

    let re = r"^solve lambdaman(\d+) (.*)$";
    let cs = regex::Regex::new(re)
        .unwrap()
        .captures(&input)
        .expect(format!("invalid input: must match {re}").as_str());
    let id = cs[1].parse::<usize>()?;
    let instr = cs[2].trim().to_string();

    let mut map = LMap::from_id(id)?;
    map.do_move(&instr)?;

    let rem = map.remaining_pills();
    if rem == 0 {
        eprintln!("no remaining pills - congrats!");
        return Ok(());
    }
    bail!("{} pills remaining", rem);
}
