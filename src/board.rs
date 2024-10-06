use crate::utils::Coordinate;

pub enum GameState {
    InProgress,
    Draw,
    XWin,
    OWin,
}

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

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        return self.grid[x][y];
    }

    pub fn get_moves(&self) -> u8 {
        return self.moves;
    }

    pub fn place(&mut self, coord: Coordinate, val: bool) -> Result<bool, &str> {
        match self.grid[coord.x()][coord.y()] {
            Some(_) => return Err("Cell is already occupied"),
            None => {
                self.grid[coord.x()][coord.y()] = Some(val);
                self.moves += 1;
                return Ok(val);
            }
        }
    }

    pub fn check_row(&self, row: usize) -> bool {
        match self.grid[row] {
            [Some(val1), Some(val2), Some(val3)] => {
                return val1 == val2 && val2 == val3;
            }
            _ => return false,
        }
    }

    pub fn check_column(&self, col: usize) -> bool {
        match (self.grid[0][col], self.grid[1][col], self.grid[2][col]) {
            (Some(val1), Some(val2), Some(val3)) => {
                return val1 == val2 && val2 == val3;
            }
            _ => return false,
        }
    }

    pub fn check_diagonal(&self) -> bool {
        match (self.grid[0][0], self.grid[1][1], self.grid[2][2]) {
            (Some(val1), Some(val2), Some(val3)) => {
                return val1 == val2 && val2 == val3;
            }
            _ => {}
        }

        match (self.grid[0][2], self.grid[1][1], self.grid[2][0]) {
            (Some(val1), Some(val2), Some(val3)) => {
                return val1 == val2 && val2 == val3;
            }
            _ => return false,
        }
    }

    pub fn get_state(&self) -> GameState {
        let mut state: GameState = if self.moves < 9 {
            GameState::InProgress
        } else {
            GameState::Draw
        };

        if self.check_diagonal() {
            if self.grid[1][1] == Some(true) {
                state = GameState::XWin;
            } else {
                state = GameState::OWin;
            }

            return state;
        }

        for i in 0..2 {
            if self.check_row(i) {
                state = if self.grid[i][0] == Some(true) {
                    GameState::XWin
                } else {
                    GameState::OWin
                };

                return state;
            }

            if self.check_column(i) {
                state = if self.grid[0][i] == Some(true) {
                    GameState::XWin
                } else {
                    GameState::OWin
                };

                return state;
            }
        }

        return state;
    }

    pub fn draw(&self) {
        for row in &self.grid {
            for &cell in row {
                let cell_val: char = match cell {
                    Some(true) => 'X',
                    Some(false) => 'O',
                    None => ' ',
                };
                print!("|{}|", cell_val);
            }
            println!();
        }
        println!();
    }
}
