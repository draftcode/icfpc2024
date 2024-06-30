use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Context};

#[derive(Debug, Clone)]
pub struct LMap {
    data: Vec<Vec<LCell>>,
    pills: usize,
    pos: (usize, usize),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LCell {
    Wall,
    Empty,
    Pill,
    Lambdaman,
}

impl TryFrom<char> for LCell {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(LCell::Wall),
            ' ' => Ok(LCell::Empty),
            '.' => Ok(LCell::Pill),
            'L' => Ok(LCell::Lambdaman),
            _ => bail!("unknown cell type: {c:?}"),
        }
    }
}

impl FromStr for LMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut data: Vec<Vec<LCell>> = Vec::new();
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
                    .map(|c| c.try_into())
                    .collect::<Result<_, _>>()?,
            );
            height += 1;
        }

        // Count pills.
        let mut pills = 0;
        for row in &data {
            for cell in row {
                if *cell == LCell::Pill {
                    pills += 1;
                }
            }
        }

        // Locate lambdaman.
        let mut pos: Option<(usize, usize)> = None;
        for x in 0..height {
            for y in 0..width {
                if data[x][y] == LCell::Lambdaman {
                    if pos.is_some() {
                        bail!("corrupted map: multiple lambdamen");
                    }
                    pos = Some((x, y));
                }
            }
        }
        let pos = pos.context("lambdaman not found")?;

        Ok(LMap { data, pills, pos })
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

    pub fn remaining_pills(&self) -> usize {
        self.pills
    }

    pub fn do_move(&mut self, instr: &str) -> anyhow::Result<()> {
        let height = self.data.len();
        let width = self.data[0].len();
        for c in instr.chars() {
            let (x, y) = self.pos;
            let (dx, dy) = match c {
                'U' => (-1, 0),
                'D' => (1, 0),
                'L' => (0, -1),
                'R' => (0, 1),
                _ => bail!("unknown instruction: {}", c),
            };
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= height as isize || ny < 0 || ny >= width as isize {
                bail!("out of map");
            }
            let nx = nx as usize;
            let ny = ny as usize;
            match self.data[nx][ny] {
                LCell::Wall => continue,
                LCell::Empty => {}
                LCell::Pill => {
                    self.pills -= 1;
                }
                LCell::Lambdaman => panic!("two lambdamen: identity crisis"),
            }
            self.data[x][y] = LCell::Empty;
            self.data[nx][ny] = LCell::Lambdaman;
            self.pos = (nx, ny);
        }
        Ok(())
    }
}
