use std::io::Write as _;

use common::{eval, expr::Expr};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    loop {
        write!(&mut stdout, "> ")?;
        stdout.flush()?;

        let mut s = String::new();
        stdin.read_line(&mut s)?;
        let expr: Expr = s.parse()?;
        log::info!("{}", expr);
        let result = eval::eval(&expr)?;
        println!("{}", result);

        match result {
            Expr::String(s) => {
                println!("=== string ===");
                println!("{}", s);
            }
            _ => {}
        }
    }
}
