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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct State {
    pub board: Board,
    pub history: Vec<Board>,
    pub input_a: i32,
    pub input_b: i32,
    pub output: Option<i32>,
    written: Vec<Vec<bool>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            board: Board(vec![]),
            history: vec![],
            input_a: 0,
            input_b: 0,
            output: None,
            written: vec![],
        }
    }
}

fn inside(board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> bool {
    let x = pos.0;
    let y = pos.1;
    x >= 0 && x < board[0].len() as i32 && y >= 0 && y < board.len() as i32
}

fn get_cell(board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> Option<Cell> {
    if !inside(board, pos) {
        None
    } else {
        Some(board[pos.1 as usize][pos.0 as usize])
    }
}

fn readable(board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> bool {
    match get_cell(board, pos) {
        Some(Cell::Number(_)) => true,
        Some(Cell::InputA) => true,
        Some(Cell::InputB) => true,
        _ => false,
    }
}

impl State {
    pub fn onestep(&mut self) -> anyhow::Result<()> {
        self.history.push(self.board.clone());
        let mut new_board = self.board.0.clone();
        self.written = vec![vec![false; new_board[0].len()]; new_board.len()];
        for y in 0..new_board.len() {
            for x in 0..new_board[y].len() {
                match new_board[y][x] {
                    Cell::InputA => new_board[y][x] = Cell::Number(self.input_a),
                    Cell::InputB => new_board[y][x] = Cell::Number(self.input_b),
                    Cell::Up => {
                        self.move_v(&mut new_board, x, y, 0, -1)?;
                    },
                    Cell::Down => {
                        self.move_v(&mut new_board, x, y, 0, 1)?;
                    },
                    Cell::Right => {
                        self.move_v(&mut new_board, x, y, 1, 0)?;
                    },
                    Cell::Left => {
                        self.move_v(&mut new_board, x, y, -1, 0)?;
                    },
                    Cell::Number(_) => {},
                    Cell::Empty => {},
                    Cell::Submit => {},
                    Cell::Plus | Cell::Minus | Cell::Mul | Cell::Div | Cell::Rem => {
                        self.binop_arith(&mut new_board, x, y)?;
                    },
                    Cell::Eq | Cell::Neq => {
                        self.binop_comp(&mut new_board, x, y)?;
                    }
                    Cell::Warp => {
                        println!("not implemented");
                    }
                    _ => bail!("Not implemented {:?}", new_board[y][x]),
                }
            }
        }
        self.board.0 = new_board;
        Ok(())
    }

    fn move_v(&mut self, board: &mut Vec<Vec<Cell>>, x: usize, y: usize, dx: i32, dy: i32) -> anyhow::Result<()> {
        let from_x = x as i32 - dx;
        let from_y = y as i32 - dy;
        let to_x = x as i32 + dx;
        let to_y = y as i32 + dy;
        if !inside(&board, (from_x, from_y)) || !inside(&board, (to_x, to_y)) {
            bail!("Invalid move from {},{} to {},{}", from_x, from_y, to_x, to_y);
        }
        if !self.writable(board, (to_x, to_y)) {
            bail!("Trying to write the cell twice {},{}", to_x, to_y);
        }
        let to_x = to_x as usize;
        let to_y = to_y as usize;
        if let Some(i) = self.get_number((from_x, from_y)) {
            self.write_to(board, to_x, to_y, Cell::Number(i))?;
            // Not updating written
            board[from_y as usize][from_x as usize] = Cell::Empty;
        }

        Ok(())
    }

    fn write_to(&mut self, board: &mut Vec<Vec<Cell>>, x: usize, y: usize, v: Cell) -> anyhow::Result<()> {
        if !inside(&board, (x as i32, y as i32)) {
            bail!("Invalid write to {},{}", x, y);
        }
        if let Cell::Submit = board[y][x] {
            if let Cell::Number(i) = v {
                self.output = Some(i);
            }
        }
        board[y][x] = v;
        self.written[y][x] = true;
        Ok(())
    }

    fn binop_arith(&mut self, board: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> anyhow::Result<()> {
        let arg1 = (x as i32 - 1, y as i32);
        let arg2 = (x as i32, y as i32 - 1);
        let to1 = (x as i32 + 1, y as i32);
        let to2 = (x as i32, y as i32 + 1);
        if !readable(&self.board.0, arg1) || !readable(&self.board.0, arg2) {
            // Args are not ready yet.
            return Ok(());
        }
        if !self.writable(board, to1) || !self.writable(board, to2) {
            bail!("Cell is not writable {:?} or {:?}", to1, to2);
        }
        let op1 = if let Some(v) = self.get_number(arg1) {
            v
        } else {
            bail!("non number at {:?}", arg1);
        };
        let op2 = if let Some(v) = self.get_number(arg2) {
            v
        } else {
            bail!("non number at {:?}", arg2);
        };

        // Arith
        let result = match board[y][x] {
            Cell::Plus => op1 + op2,
            Cell::Minus => op1 - op2,
            Cell::Mul => op1 * op2,
            Cell::Div => op1 / op2,
            Cell::Rem => op1 % op2,
            _ => bail!("Non arith binop {:?}", board[y][x]),
        };
        self.write_to(board, to1.0 as usize, to1.1 as usize, Cell::Number(result))?;
        self.write_to(board, to2.0 as usize, to2.1 as usize, Cell::Number(result))?;
        self.write_to(board, arg1.0 as usize, arg1.1 as usize, Cell::Empty)?;
        self.write_to(board, arg2.0 as usize, arg2.1 as usize, Cell::Empty)?;

        Ok(())
    }

    fn binop_comp(&mut self, board: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> anyhow::Result<()> {
        let arg1 = (x as i32 - 1, y as i32);
        let arg2 = (x as i32, y as i32 - 1);
        let to1 = (x as i32 + 1, y as i32);
        let to2 = (x as i32, y as i32 + 1);
        if !readable(&self.board.0, arg1) || !readable(&self.board.0, arg2) {
            // Args are not ready yet.
            return Ok(());
        }
        if !self.writable(board, to1) || !self.writable(board, to2) {
            bail!("Cell is not writable {:?} or {:?}", to1, to2);
        }
        let op1 = if let Some(v) = self.get_number(arg1) {
            v
        } else {
            println!("found arg1 {:?} {}", self.board.0[arg1.1 as usize][arg1.0 as usize], readable(&self.board.0, arg1));
            bail!("non number at {:?}", arg1);
        };
        let op2 = if let Some(v) = self.get_number(arg2) {
            v
        } else {
            println!("found {:?}", self.board.0[arg1.1 as usize][arg1.0 as usize]);
            bail!("non number at {:?}", arg2);
        };

        let res = match board[y][x] {
            Cell::Eq => op1 == op2,
            Cell::Neq => op1 != op2,
            _ => bail!("Invalid comp op {}", board[y][x])
        };
        if res {
            self.write_to(board, to1.0 as usize, to1.1 as usize, Cell::Number(op2))?;
            self.write_to(board, to2.0 as usize, to2.1 as usize, Cell::Number(op1))?;
            self.write_to(board, arg1.0 as usize, arg1.1 as usize, Cell::Empty)?;
            self.write_to(board, arg2.0 as usize, arg2.1 as usize, Cell::Empty)?;   
        }
        Ok(())
    }

    fn writable(&self, board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> bool {
        if !inside(board, pos) {
            return false;
        }
        return !self.written[pos.1 as usize][pos.0 as usize];
    }

    fn get_number(&self, pos: (i32, i32)) -> Option<i32> {
        if !self.writable(&self.board.0, pos) {
            return None;
        }
        match self.board.0[pos.1 as usize][pos.0 as usize] {
            Cell::Number(i) => Some(i),
            Cell::InputA => Some(self.input_a),
            Cell::InputB => Some(self.input_b),
            _ => None
        }
    }
}