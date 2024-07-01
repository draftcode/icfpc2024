use std::str::FromStr;

use anyhow::{self, bail};
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    InputA,
    InputB,
    Number(BigInt),
    Up,
    Down,
    Left,
    Right,
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
    Warp(String),
    Eq,
    Neq,
    Submit,
    Label(String, Option<Box<Cell>>),
}

fn parse_label(s: &str) -> anyhow::Result<(String, Option<Cell>)> {
    if !s.contains("[") {
        return Ok((s.to_owned(), None));
    }
    let v = s.split("[").collect::<Vec<&str>>();
    let name = v[0].to_owned();
    let v = v[1].split("]").collect::<Vec<&str>>();
    if v[0] == "A" {
        return Ok((name, Some(Cell::InputA)));
    }
    if v[0] == "B" {
        return Ok((name, Some(Cell::InputB)));
    }
    let val = v[0].parse::<i32>();
    if val.is_err() {
        bail!("Failed to parse the label value");
    }
    Ok((name, Some(Cell::Number(val?.into()))))
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
            "@" => Cell::Warp("_".to_string()),
            "=" => Cell::Eq,
            "#" => Cell::Neq,
            "S" => Cell::Submit,
            _ => match s.parse::<i32>() {
                Ok(i) => {
                    if i <= -100 || i >= 100 {
                        bail!("Invalid number: {}", s);
                    }
                    Cell::Number(i.into())
                }
                _ => {
                    if s.len() >= 1 && s.chars().nth(0).unwrap() != '@' {
                        let (name, v) = parse_label(s)?;
                        if v.is_some() {
                            Cell::Label(name, Some(Box::new(v.unwrap())))
                        } else {
                            Cell::Label(name, None)
                        }
                    } else if s.len() >= 2 && s.chars().next().unwrap() == '@' {
                        let label = &s[1..];
                        Cell::Warp(label.to_owned())
                    } else {
                        bail!("Invalid cell: {}", s);
                    }
                }
            },
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
            Cell::Warp(l) => {
                if l == "_" {
                    write!(f, "@")
                } else {
                    write!(f, "@{}", l)
                }
            }
            Cell::Eq => write!(f, "="),
            Cell::Neq => write!(f, "#"),
            Cell::Submit => write!(f, "S"),
            Cell::Number(i) => write!(f, "{}", i.to_string()),
            Cell::Label(c, init) => {
                if init.is_some() {
                    write!(f, "{}[{}]", c, init.clone().unwrap())
                } else {
                    write!(f, "{}", c)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board(pub Vec<Vec<Cell>>);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col_len = vec![0; self.0[0].len()];
        for l in self.0.iter() {
            for x in 0..l.len() {
                col_len[x] = col_len[x].max(format!("{}", l[x]).len());
            }
        }
        for l in self.0.iter() {
            let mut cols = vec![];
            for (idx, c) in l.iter().enumerate() {
                let mut s = format!("{}", c);
                s = " ".repeat(0.max(col_len[idx] - s.len())) + &s;
                cols.push(s);
            }
            write!(f, "{}\n", cols.join(" "))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub board: Board,
    pub history: Vec<Board>,
    pub input_a: BigInt,
    pub input_b: BigInt,
    pub output: Option<BigInt>,
    pub tick: i32,
    max_tick: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    written: Vec<Vec<bool>>,
    pub monotonic_tick: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            board: Board(vec![]),
            history: vec![],
            input_a: 0.into(),
            input_b: 0.into(),
            output: None,
            tick: 1,
            max_tick: 1,
            min_x: i32::MAX / 3,
            max_x: i32::MIN / 3,
            min_y: i32::MAX / 3,
            max_y: i32::MIN / 3,
            written: vec![],
            monotonic_tick: 0,
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
        Some(board[pos.1 as usize][pos.0 as usize].clone())
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

fn is_cell_emtpy(board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> bool {
    if !inside(board, pos) {
        return false;
    }
    match get_cell(board, pos) {
        Some(Cell::Empty) => true,
        _ => false,
    }
}

pub fn print_for_submit(state: &State) -> String {
    let mut col_len = vec![0; state.board.0[0].len()];

    for l in state.board.0.iter() {
        for (idx, c) in l.iter().enumerate() {
            let len = if let Cell::Warp(_) = c {
                1
            } else {
                format!("{}", c).len()
            };
            col_len[idx] = col_len[idx].max(len);
        }
    }

    let mut s = String::new();
    for l in state.board.0.iter() {
        let mut cols = vec![];
        for (idx, c) in l.iter().enumerate() {
            let cs = if let Cell::Warp(_) = c {
                "@".to_owned()
            } else {
                format!("{}", c)
            };
            cols.push(" ".repeat(0.max(col_len[idx] - cs.len())) + &cs);
        }
        s += format!("{}\n", cols.join(" ")).as_str();
    }
    s
}

impl State {
    pub fn new(board: &str, a: BigInt, b: BigInt) -> anyhow::Result<Self> {
        let mut s: State = Default::default();

        let mut col_max = 0;
        for l in board.lines() {
            let mut col_len = 0;
            let _ = l.split_whitespace().for_each(|_| col_len += 1);
            col_max = col_max.max(col_len);
        }

        s.board.0.push(vec![Cell::Empty; col_max + 2]);
        for l in board.lines() {
            if l.is_empty() || l.chars().nth(0).unwrap() == '?' {
                continue;
            }
            let mut row = vec![Cell::Empty];
            for c in l.split_whitespace() {
                row.push(Cell::from_str(c)?);
            }
            while row.len() < col_max + 2 {
                row.push(Cell::Empty);
            }
            s.board.0.push(row);
        }
        s.board.0.push(vec![Cell::Empty; col_max + 2]);
        s.input_a = a;
        s.input_b = b;

        // Replace A and B immediately after parsing.
        for l in s.board.0.iter_mut() {
            for c in l.iter_mut() {
                match c {
                    Cell::InputA => {
                        *c = Cell::Number(s.input_a.clone());
                    }
                    Cell::InputB => {
                        *c = Cell::Number(s.input_b.clone());
                    }
                    _ => {}
                }
            }
        }

        Ok(s)
    }

    pub fn new_with_input_port(board: &str, a: BigInt, b: BigInt) -> anyhow::Result<Self> {
        let mut s: State = Default::default();

        let mut col_max = 0;
        for l in board.lines() {
            let mut col_len = 0;
            let _ = l.split_whitespace().for_each(|_| col_len += 1);
            col_max = col_max.max(col_len);
        }

        s.board.0.push(vec![Cell::Empty; col_max + 2]);
        for l in board.lines() {
            if l.is_empty() || l.chars().nth(0).unwrap() == '?' {
                continue;
            }
            let mut row = vec![Cell::Empty];
            for c in l.split_whitespace() {
                row.push(Cell::from_str(c)?);
            }
            while row.len() < col_max + 2 {
                row.push(Cell::Empty);
            }
            s.board.0.push(row);
        }
        s.board.0.push(vec![Cell::Empty; col_max + 2]);
        s.input_a = a.clone();
        s.input_b = b.clone();
        Ok(s)
    }

    pub fn used_x(&self) -> i32 {
        self.max_x - self.min_x + 1
    }
    pub fn used_y(&self) -> i32 {
        self.max_y - self.min_y + 1
    }

    pub fn score(&self) -> i32 {
        self.used_x() * self.used_y() * (self.max_tick + 1)
    }

    pub fn resolve_label(&mut self) -> anyhow::Result<()> {
        let mut labels = vec![];
        let mut refs = vec![];
        for y in 0..self.board.0.len() {
            for x in 0..self.board.0[y].len() {
                if let Cell::Label(c, init) = &self.board.0[y][x] {
                    labels.push((c.clone(), x, y, init.clone()));
                } else if let Cell::InputA = self.board.0[y][x] {
                    labels.push(("A".to_owned(), x, y, None));
                } else if let Cell::InputB = self.board.0[y][x] {
                    labels.push(("B".to_owned(), x, y, None));
                } else if let Cell::Warp(c) = &self.board.0[y][x] {
                    if c == "_" {
                        continue;
                    }
                    refs.push((c.clone(), x, y));
                }
            }
        }

        for (l, tx, ty, _) in labels.iter() {
            if l == "A" || l == "B" {
                continue;
            }
            let mut found = false;
            for (c, x, y) in refs.iter() {
                if *c == *l {
                    found = true;
                    break;
                }
            }
            if !found {
                bail!("Label {} at {},{} is not used", l, tx, ty);
            }
        }

        for (c, x, y) in refs.iter() {
            let mut found = false;
            for (l, tx, ty, _) in labels.iter() {
                if *c == *l {
                    found = true;
                    let dx = *x as i32 - *tx as i32;
                    let dy = *y as i32 - *ty as i32;
                    self.board.0[*y][x - 1] = Cell::Number(dx.into());
                    self.board.0[*y][x + 1] = Cell::Number(dy.into());
                }
            }
            if !found {
                bail!("Label {} not found used by {},{}", c, x, y);
            }
        }

        // Clear labels
        for (l, x, y, init) in labels.iter() {
            if l == "A" || l == "B" {
                continue;
            }
            if init.is_none() {
                self.board.0[*y][*x] = Cell::Empty;
            } else {
                self.board.0[*y][*x] = *init.clone().unwrap();
            }
        }
        Ok(())
    }

    pub fn onestep(&mut self) -> anyhow::Result<()> {
        self.monotonic_tick += 1;
        for y in 0..self.board.0.len() {
            for x in 0..self.board.0[y].len() {
                if let Cell::Label(_, _) = self.board.0[y][x] {
                    bail!("Please call resolve_label() before one_step()");
                }
            }
        }
        self.history.push(self.board.clone());
        self.tick += 1;
        self.max_tick = self.max_tick.max(self.tick);

        let mut warp_requests = vec![];

        let mut new_board = self.board.0.clone();
        self.written = vec![vec![false; new_board[0].len()]; new_board.len()];
        for y in 0..new_board.len() {
            for x in 0..new_board[y].len() {
                match new_board[y][x] {
                    Cell::InputA => new_board[y][x] = Cell::Number(self.input_a.clone()),
                    Cell::InputB => new_board[y][x] = Cell::Number(self.input_b.clone()),
                    Cell::Up => {
                        self.move_v(&mut new_board, x, y, 0, -1)?;
                    }
                    Cell::Down => {
                        self.move_v(&mut new_board, x, y, 0, 1)?;
                    }
                    Cell::Right => {
                        self.move_v(&mut new_board, x, y, 1, 0)?;
                    }
                    Cell::Left => {
                        self.move_v(&mut new_board, x, y, -1, 0)?;
                    }
                    Cell::Number(_) => {}
                    Cell::Empty => {}
                    Cell::Submit => {}
                    Cell::Plus | Cell::Minus | Cell::Mul | Cell::Div | Cell::Rem => {
                        self.binop_arith(&mut new_board, x, y)?;
                    }
                    Cell::Eq | Cell::Neq => {
                        self.binop_comp(&mut new_board, x, y)?;
                    }
                    Cell::Warp(_) => {
                        self.warp(x, y, &mut warp_requests)?;
                    }
                    _ => bail!("Not implemented {:?}", new_board[y][x]),
                }
            }
        }

        if self.output.is_some() {
            // Do not process warp requests if 'S' is already written
            return Ok(());
        }

        if !warp_requests.is_empty() {
            let dt = warp_requests[0].0;
            for (ddt, _, _, _) in &warp_requests {
                if dt != *ddt {
                    bail!("warp: dt is not consistent {} vs {}", dt, ddt);
                }
            }
            let target_t = (self.history.len() as i32 - dt - 1) as usize;
            new_board = self.history[target_t].0.clone();
            self.tick = (target_t + 1) as i32;
            self.history = self.history.split_at(target_t).0.to_vec();

            self.written = vec![vec![false; new_board[0].len()]; new_board.len()];
            for (_, x, y, v) in &warp_requests {
                if self.written[*y as usize][*x as usize] {
                    // Check if the same value is used or not.
                    if let Cell::Number(w) = &new_board[*y as usize][*x as usize] {
                        if v != w {
                            bail!(
                                "The different value is goint to be written by warp ({},{}) {} vs {}",
                                x,
                                y,
                                v,
                                w
                            );
                        }
                    }
                }
                self.write_to(&mut new_board, *x, *y, Cell::Number(v.clone()))?;
            }
        }

        self.board.0 = new_board;
        Ok(())
    }

    fn move_v(
        &mut self,
        board: &mut Vec<Vec<Cell>>,
        x: usize,
        y: usize,
        dx: i32,
        dy: i32,
    ) -> anyhow::Result<()> {
        self.update_min_max((x as i32, y as i32));
        let from_x = x as i32 - dx;
        let from_y = y as i32 - dy;
        let to_x = x as i32 + dx;
        let to_y = y as i32 + dy;

        if !inside(&board, (from_x, from_y)) || !inside(&board, (to_x, to_y)) {
            bail!(
                "Invalid move from {},{} to {},{}",
                from_x,
                from_y,
                to_x,
                to_y
            );
        }
        if !readable(&self.board.0, (from_x, from_y)) {
            // Arg is not ready yet.
            return Ok(());
        }
        if !inside(board, (to_x, to_y)) {
            return Ok(());
        }
        if !self.writable(board, (to_x, to_y)) {
            bail!("Trying to write the cell twice {},{}", to_x, to_y);
        }
        if let Some(i) = self.get_number((from_x, from_y)) {
            self.write_to(board, to_x, to_y, Cell::Number(i))?;
            // Not updating written
            if self.writable(board, (from_x, from_y)) {
                self.raw_write(board, (from_x, from_y), Cell::Empty)?;
            }
        } else {
            bail!("@@@@@@");
        }

        Ok(())
    }

    fn write_to(
        &mut self,
        board: &mut Vec<Vec<Cell>>,
        x: i32,
        y: i32,
        v: Cell,
    ) -> anyhow::Result<()> {
        if !inside(&board, (x, y)) {
            bail!("Invalid write to {},{}", x, y);
        }
        let x = x as usize;
        let y = y as usize;
        if let Cell::Submit = board[y][x] {
            if let Cell::Number(i) = v.clone() {
                self.output = Some(i);
            }
        }
        board[y][x] = v;
        self.written[y][x] = true;
        self.update_min_max((x as i32, y as i32));
        Ok(())
    }

    fn raw_write(
        &mut self,
        board: &mut Vec<Vec<Cell>>,
        pos: (i32, i32),
        v: Cell,
    ) -> anyhow::Result<()> {
        if !inside(&board, pos) {
            bail!("Invalid write to {:?}", pos);
        }
        board[pos.1 as usize][pos.0 as usize] = v;
        self.update_min_max(pos);
        Ok(())
    }

    fn binop_arith(
        &mut self,
        board: &mut Vec<Vec<Cell>>,
        x: usize,
        y: usize,
    ) -> anyhow::Result<()> {
        let arg1 = (x as i32 - 1, y as i32);
        let arg2 = (x as i32, y as i32 - 1);
        let to1 = (x as i32 + 1, y as i32);
        let to2 = (x as i32, y as i32 + 1);
        if !readable(&self.board.0, arg1) || !readable(&self.board.0, arg2) {
            // Args are not ready yet.
            return Ok(());
        }

        self.update_min_max((x as i32, y as i32));
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
        if inside(board, to1) {
            if !self.writable(board, to1) {
                bail!("Writing to the same cell twice: {:?}", to1);
            }
            self.write_to(board, to1.0, to1.1, Cell::Number(result.clone()))?;
        }
        if inside(&board, to2) {
            if !self.writable(board, to2) {
                bail!("Writing to the same cell twice: {:?}", to2);
            }
            self.write_to(board, to2.0, to2.1, Cell::Number(result.clone()))?;
        }

        // Make the argument cells empty only when no other ops wrote there.
        if self.writable(board, arg1) {
            self.raw_write(board, arg1, Cell::Empty)?;
        }
        if self.writable(board, arg2) {
            self.raw_write(board, arg2, Cell::Empty)?;
        }

        Ok(())
    }

    fn binop_comp(&mut self, board: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> anyhow::Result<()> {
        let arg1 = (x as i32 - 1, y as i32);
        let arg2 = (x as i32, y as i32 - 1);
        let to1 = (x as i32 + 1, y as i32);
        let to2 = (x as i32, y as i32 + 1);
        if is_cell_emtpy(&self.board.0, arg1) || is_cell_emtpy(&self.board.0, arg2) {
            // Args are not ready yet.
            return Ok(());
        }
        let op1 = get_cell(&self.board.0, arg1).unwrap_or(Cell::Empty);
        let op2 = get_cell(&self.board.0, arg2).unwrap_or(Cell::Empty);

        let res = match board[y][x] {
            Cell::Eq => op1 == op2,
            Cell::Neq => op1 != op2,
            _ => bail!("Invalid comp op {}", board[y][x]),
        };
        if res {
            if inside(board, to1) {
                if !self.writable(board, to1) {
                    bail!("Trying to write to the same cell twice: {:?}", to1);
                }
                self.write_to(board, to1.0, to1.1, op2)?;
            }
            if inside(board, to2) {
                if !self.writable(&board, to2) {
                    bail!("Trying to write to the same cell twice: {:?}", to2);
                }
                self.write_to(board, to2.0, to2.1, op1)?;
            }
            if self.writable(board, arg1) {
                self.raw_write(board, arg1, Cell::Empty)?;
            }
            if self.writable(board, arg2) {
                self.raw_write(board, arg2, Cell::Empty)?;
            }
        }
        Ok(())
    }

    fn writable(&self, board: &Vec<Vec<Cell>>, pos: (i32, i32)) -> bool {
        if !inside(board, pos) {
            return false;
        }
        return !self.written[pos.1 as usize][pos.0 as usize];
    }

    fn get_number(&self, pos: (i32, i32)) -> Option<BigInt> {
        if !readable(&self.board.0, pos) {
            return None;
        }
        match &self.board.0[pos.1 as usize][pos.0 as usize] {
            Cell::Number(i) => Some(i.clone()),
            Cell::InputA => Some(self.input_a.clone()),
            Cell::InputB => Some(self.input_b.clone()),
            _ => None,
        }
    }

    fn warp(
        &mut self,
        x: usize,
        y: usize,
        warp_requests: &mut Vec<(i32, i32, i32, BigInt)>,
    ) -> anyhow::Result<()> {
        const DX: [i32; 4] = [-1, 0, 1, 0];
        const DY: [i32; 4] = [0, -1, 0, 1];
        for i in 0..4 {
            let nx = x as i32 + DX[i];
            let ny = y as i32 + DY[i];
            if !inside(&self.board.0, (nx, ny)) {
                bail!("Outside of board {},{}", nx, ny);
            }
            if !readable(&self.board.0, (nx, ny)) {
                // Argments are not ready yet.
                return Ok(());
            }
        }

        let x = x as i32;
        let y = y as i32;
        let dx: i32 = self.get_number((x - 1, y)).unwrap().try_into()?;
        let dy: i32 = self.get_number((x + 1, y)).unwrap().try_into()?;
        let dt = self.get_number((x, y + 1)).unwrap();
        let v = self.get_number((x, y - 1)).unwrap();
        warp_requests.push((dt.try_into()?, x - dx, y - dy, v));

        Ok(())
    }

    fn update_min_max(&mut self, pos: (i32, i32)) {
        self.min_x = self.min_x.min(pos.0);
        self.max_x = self.max_x.max(pos.0);
        self.min_y = self.min_y.min(pos.1);
        self.max_y = self.max_y.max(pos.1);
    }
}

#[cfg(test)]
mod tests {}
