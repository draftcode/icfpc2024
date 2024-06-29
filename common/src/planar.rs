use std::str::FromStr;

use anyhow::{self, bail};

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    InputA,
    InputB,
    Number(i32),
    Up,
    Down,
    Left,
    Right,
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
    Warp,
    Eq,
    Neq,
    Submit,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Cell> {
        let c = match s {
            "." => Cell::Empty,
            "A" => Cell::InputA,
            "B" => Cell::InputB,
            "^" => Cell::Up,
            "v" => Cell::Down,
            ">" => Cell::Right,
            "<" => Cell::Left,
            "+" => Cell::Plus,
            "-" => Cell::Minus,
            "*" => Cell::Mul,
            "/" => Cell::Div,
            "%" => Cell::Rem,
            "@" => Cell::Warp,
            "=" => Cell::Eq,
            "#" => Cell::Neq,
            "S" => Cell::Submit,
            _ => {
                match s.parse::<i32>() {
                    Ok(i) => {
                        if i <= -100 || i >= 100 {
                            bail!("Invalid number: {}", s);
                        }
                        Cell::Number(i)
                    }
                    _ => bail!("Invalid cell: {}", s)
                }
            }
        };
        Ok(c)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::InputA => write!(f, "A"),
            Cell::InputB => write!(f, "B"),
            Cell::Up => write!(f, "^"),
            Cell::Down => write!(f, "v"),
            Cell::Right => write!(f, ">"),
            Cell::Left => write!(f, "<"),
            Cell::Plus => write!(f, "+"),
            Cell::Minus => write!(f, "-"),
            Cell::Mul => write!(f, "*"),
            Cell::Div => write!(f, "/"),
            Cell::Rem => write!(f, "%"),
            Cell::Warp => write!(f, "@"),
            Cell::Eq => write!(f, "="),
            Cell::Neq => write!(f, "#"),
            Cell::Submit => write!(f, "S"),
            Cell::Number(i) => write!(f, "{}", i.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Board(pub Vec<Vec<Cell>>);

impl std::fmt::Display for Board {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter() {
            write!(f, "{}", l.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join(" "))?;
            write!(f, "\n")?;
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    pub board: Board,
    pub history: Vec<Board>,
}

impl Default for State {
    fn default() -> Self {
        State {
            board: Board(vec![]),
            history: vec![],
        }
    }
}

impl State {

}