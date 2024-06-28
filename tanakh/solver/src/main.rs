use std::io::BufRead as _;

use expr::{tokenize, Expr};
use solver::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens = tokenize(&line)?;
        // println!("{:?}", tokens);
        let expr = Expr::parse(&tokens)?;
        println!("{}", expr);

        let result = eval::eval(&expr)?;
        println!("{}", result);
    }
    Ok(())
}
