use anyhow::{bail, ensure, Result};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub const MAX_MOVES: usize = 1_000_000;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl From<u64> for Direction {
    fn from(n: u64) -> Self {
        match n {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => panic!("unknown direction: {n}"),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => panic!("unknown direction: {c:?}"),
        }
    }
}

impl Direction {
    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Wall,
    Empty,
    Pill,
    LambdaMan,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            ' ' => Cell::Empty,
            '.' => Cell::Pill,
            'L' => Cell::LambdaMan,
            _ => panic!("unknown cell type: {c:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Vec<Vec<Cell>>,
    pub height: usize,
    pub width: usize,
    pub pills: usize,
    pub pos: (usize, usize),
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut field: Vec<Vec<Cell>> = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();

        // Ensure the field is rectangular.
        let height = field.len();
        let width = field[0].len();
        ensure!(
            field.iter().all(|row| row.len() == width),
            "inconsistent line length"
        );

        // Count pills.
        let pills = field
            .iter()
            .flatten()
            .filter(|&&cell| cell == Cell::Pill)
            .count();

        // Find the lambdaman.
        let candidates: Vec<(usize, usize)> = field
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &cell)| {
                    if cell == Cell::LambdaMan {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();
        ensure!(candidates.len() == 1, "lambdaman must be exactly one");
        let pos = candidates[0];
        field[pos.0][pos.1] = Cell::Empty;

        Ok(Self {
            field,
            height,
            width,
            pills,
            pos,
        })
    }
}

impl Game {
    pub fn pills(&self) -> usize {
        self.pills
    }

    pub fn step(&mut self, dir: Direction) {
        let (di, dj) = dir.delta();
        let (ci, cj) = self.pos;
        let (ni, nj) = (ci as isize + di, cj as isize + dj);
        // Reject moves that go out of bounds.
        if ni < 0 || ni >= self.height as isize || nj < 0 || nj >= self.width as isize {
            return;
        }

        let (ni, nj) = (ni as usize, nj as usize);
        match self.field[ni][nj] {
            Cell::Wall => return,
            Cell::Empty => {}
            Cell::Pill => {
                self.field[ni][nj] = Cell::Empty;
                self.pills -= 1;
            }
            Cell::LambdaMan => unreachable!(),
        }
        self.pos = (ni, nj);
    }
}

fn find_problems_dir() -> Result<PathBuf> {
    let current_dir = Path::new(".").canonicalize()?;
    for dir in current_dir.ancestors() {
        if dir.join(".git").exists() {
            return Ok(dir.join("problems/lambdaman"));
        }
    }
    bail!("Must be run under a git repository");
}

pub fn load_game(problem_id: usize) -> Result<Game> {
    let problems_dir = find_problems_dir()?;
    let problem_path = problems_dir.join(format!("{}.txt", problem_id));
    let content = std::fs::read_to_string(problem_path)?;
    content.parse()
}
