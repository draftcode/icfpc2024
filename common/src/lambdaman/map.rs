use std::{fmt::Display, str::FromStr};

use anyhow::bail;

#[derive(Debug, Clone)]
pub struct LMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<LCell>>,
}

impl Display for LMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        LCell::Wall => "#",
                        LCell::Empty => " ",
                        LCell::Pill => ".",
                        LCell::Lambdaman => "L",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LCell {
    Wall,
    Empty,
    Pill,
    Lambdaman,
}

impl FromStr for LCell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(LCell::Wall),
            " " => Ok(LCell::Empty),
            "." => Ok(LCell::Pill),
            "L" => Ok(LCell::Lambdaman),
            _ => bail!("unknown cell type: {}", s),
        }
    }
}

impl FromStr for LMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                bail!("inconsistent line length");
            }
            data.push(
                line.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<_, _>>()?,
            );
            height += 1;
        }
        Ok(LMap {
            width,
            height,
            data,
        })
    }
}

impl LMap {
    pub fn from_id(id: usize) -> anyhow::Result<Self> {
        let content = std::fs::read(
            std::path::Path::new("./problems/lambdaman").join(id.to_string() + ".txt"),
        )?;
        let content = String::from_utf8(content)?;

        Ok(content.parse()?)
    }

    fn lambdaman_position(&self) -> (usize, usize) {
        for x in 0..self.height {
            for y in 0..self.width {
                if self.data[x][y] == LCell::Lambdaman {
                    return (x, y);
                }
            }
        }
        panic!("lambdaman not found");
    }

    pub fn remaining_pills(&self) -> usize {
        let mut count = 0;
        for x in 0..self.height {
            for y in 0..self.width {
                if self.data[x][y] == LCell::Pill {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn do_move(&mut self, instr: &str) -> anyhow::Result<()> {
        let (mut x, mut y) = self.lambdaman_position();
        for c in instr.chars() {
            let (dx, dy) = match c {
                'U' => (-1, 0),
                'D' => (1, 0),
                'L' => (0, -1),
                'R' => (0, 1),
                _ => bail!("unknown instruction: {}", c),
            };
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= self.height as isize || ny < 0 || ny >= self.width as isize {
                bail!("out of map");
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if self.data[nx][ny] == LCell::Wall {
                continue;
            } else {
                self.data[x][y] = LCell::Empty;
                self.data[nx][ny] = LCell::Lambdaman;
            }

            x = nx;
            y = ny;
        }
        Ok(())
    }
}
