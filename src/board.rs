use crate::utils::Coordinate;

#[derive(Debug)]
pub enum GameState {
    None,
    Draw,
    X,
    O,
}

#[derive(Clone)]
pub struct Board {
    grid: [[Option<bool>; 3]; 3],
    moves: u8,
}

impl Board {
    pub fn new() -> Self {
        return Board {
            grid: [[None; 3]; 3],
            moves: 0,
        };
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<bool> {
        return &self.grid[x][y];
    }

    pub fn place(&mut self, coord: &Coordinate, val: bool) -> Result<bool, &str> {
        match self.grid[coord.x()][coord.y()] {
            Some(_) => return Err("Cell is already occupied"),
            None => {
                self.grid[coord.x()][coord.y()] = Some(val);
                self.moves += 1;
                return Ok(val);
            }
        }
    }

    pub fn draw(&self) {
        let mut index: u8 = 0;

        for row in &self.grid {
            for &cell in row {
                index += 1;

                match cell {
                    Some(true) => print!("| X |"),
                    Some(false) => print!("| O |"),
                    None => print!("|_{}_|", index),
                };
            }
            println!();
        }
        println!();
    }

    pub fn get_state(&self) -> GameState {
        let check_3_cells =
            |x1: usize, y1: usize, x2: usize, y2: usize, x3: usize, y3: usize| match (
                self.grid[x1][y1],
                self.grid[x2][y2],
                self.grid[x3][y3],
            ) {
                (Some(true), Some(true), Some(true)) => return GameState::X,
                (Some(false), Some(false), Some(false)) => return GameState::O,
                _ => return GameState::None,
            };

        match check_3_cells(0, 0, 1, 1, 2, 2) {
            GameState::None | GameState::Draw => {}
            other => return other,
        }

        match check_3_cells(0, 2, 1, 1, 2, 0) {
            GameState::None | GameState::Draw => {}
            other => return other,
        }

        for i in 0..=2 {
            match check_3_cells(i, 0, i, 1, i, 2) {
                GameState::None | GameState::Draw => {}
                other => return other,
            }

            match check_3_cells(0, i, 1, i, 2, i) {
                GameState::None | GameState::Draw => {}
                other => return other,
            }
        }

        return if self.moves < 9 {
            GameState::None
        } else {
            GameState::Draw
        };
    }
}
