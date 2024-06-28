use std::io::Write as _;

use expr::{tokenize, Expr};
use solver::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    loop {
        write!(&mut stdout, "> ")?;
        stdout.flush()?;

        let mut s = String::new();
        stdin.read_line(&mut s)?;
        let tokens = tokenize(&s)?;
        let expr = Expr::parse(&tokens)?;
        log::info!("{}", expr);
        let result = eval::eval(&expr)?;
        println!("{}", result);
    }
}
