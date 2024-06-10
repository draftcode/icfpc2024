pub struct LifeGame {
    h: isize,
    w: isize,
    world: Vec<Vec<bool>>,
}

impl LifeGame {
    pub fn new(h: isize, w: isize) -> Self {
        let world = vec![vec![false; w as usize]; h as usize];
        Self { h, w, world }
    }

    pub fn size(&self) -> (isize, isize) {
        (self.h, self.w)
    }

    pub fn get(&self, i: isize, j: isize) -> bool {
        if i < 0 || i >= self.h || j < 0 || j >= self.w {
            false
        } else {
            self.world[i as usize][j as usize]
        }
    }

    pub fn set(&mut self, i: isize, j: isize, b: bool) {
        if i < 0 || i >= self.h || j < 0 || j >= self.w {
            panic!("LifeGame::set: index out of range");
        }
        self.world[i as usize][j as usize] = b;
    }

    pub fn tick(&mut self) {
        let mut new_world = self.world.clone();
        for i in 0..self.h {
            for j in 0..self.w {
                let mut c = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        if self.get(i + di, j + dj) {
                            c += 1;
                        }
                    }
                }
                if self.get(i, j) {
                    new_world[i as usize][j as usize] = c == 2 || c == 3;
                } else {
                    new_world[i as usize][j as usize] = c == 3;
                }
            }
        }
        self.world = new_world;
    }

    pub fn world(&self) -> &Vec<Vec<bool>> {
        &self.world
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blinker() {
        let mut game = LifeGame::new(3, 3);
        game.set(1, 0, true);
        game.set(1, 1, true);
        game.set(1, 2, true);

        assert_eq!(
            game.world(),
            &vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ]
        );

        game.tick();

        assert_eq!(
            game.world(),
            &vec![
                vec![false, true, false],
                vec![false, true, false],
                vec![false, true, false],
            ]
        );

        game.tick();

        assert_eq!(
            game.world(),
            &vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ]
        );
    }
}
