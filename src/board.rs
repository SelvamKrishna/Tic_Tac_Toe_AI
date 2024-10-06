use crate::utils::Coordinate;

#[derive(Debug)]
pub enum GameState {
    InProgress,
    Draw,
    XWin,
    OWin,
}

pub enum Winner {
    X,
    O,
    None,
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

    pub fn check_row(&self, row: usize) -> Winner {
        match self.grid[row] {
            [Some(true), Some(true), Some(true)] => return Winner::X,
            [Some(false), Some(false), Some(false)] => return Winner::O,
            _ => return Winner::None,
        }
    }

    pub fn check_column(&self, col: usize) -> Winner {
        match (self.grid[0][col], self.grid[1][col], self.grid[2][col]) {
            (Some(true), Some(true), Some(true)) => return Winner::X,
            (Some(false), Some(false), Some(false)) => return Winner::O,
            _ => return Winner::None,
        }
    }

    pub fn check_diagonal(&self) -> Winner {
        match (self.grid[0][0], self.grid[1][1], self.grid[2][2]) {
            (Some(true), Some(true), Some(true)) => return Winner::X,
            (Some(false), Some(false), Some(false)) => return Winner::O,
            _ => {}
        }

        match (self.grid[0][2], self.grid[1][1], self.grid[2][0]) {
            (Some(true), Some(true), Some(true)) => return Winner::X,
            (Some(false), Some(false), Some(false)) => return Winner::O,
            _ => return Winner::None,
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
        match self.check_diagonal() {
            Winner::X => return GameState::XWin,
            Winner::O => return GameState::OWin,
            Winner::None => {}
        }

        for i in 0..=2 {
            match self.check_row(i) {
                Winner::X => return GameState::XWin,
                Winner::O => return GameState::OWin,
                Winner::None => {}
            }

            match self.check_column(i) {
                Winner::X => return GameState::XWin,
                Winner::O => return GameState::OWin,
                Winner::None => {}
            }
        }

        return if self.moves < 9 {
            GameState::InProgress
        } else {
            GameState::Draw
        };
    }
}
