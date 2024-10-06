use crate::utils::Coordinate;

#[derive(Debug)]
pub enum GameState {
    InProgress,
    Draw,
    XWin,
    OWin,
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

    pub fn check_diagonal1(&self) -> bool {
        match (self.grid[0][0], self.grid[1][1], self.grid[2][2]) {
            (Some(val1), Some(val2), Some(val3)) => {
                return val1 == val2 && val2 == val3;
            }
            _ => return false,
        }
    }

    pub fn check_diagonal2(&self) -> bool {
        match (self.grid[0][2], self.grid[1][1], self.grid[2][0]) {
            (Some(val1), Some(val2), Some(val3)) => {
                return val1 == val2 && val2 == val3;
            }
            _ => return false,
        }
    }

    // pub fn get_state(&self) -> GameState {
    //     let check_winner = |x: usize, y: usize| {
    //         return if self.grid[x][y].is_some_and(|x| x) {
    //             GameState::XWin
    //         } else {
    //             GameState::OWin
    //         };
    //     };

    //     if self.check_diagonal() {
    //         return check_winner(1, 1);
    //     }

    //     for i in 0..2 {
    //         if self.check_row(i) {
    //             return check_winner(i, 0);
    //         }

    //         if self.check_column(i) {
    //             return check_winner(0, i);
    //         }
    //     }

    //     return if self.moves < 9 {
    //         GameState::InProgress
    //     } else {
    //         GameState::Draw
    //     };
    // }

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
        if self.check_diagonal1() || self.check_diagonal2() {
            match self.grid[1][1] {
                Some(true) => return GameState::XWin,
                Some(false) => return GameState::OWin,
                None => unreachable!(),
            };
        }

        for i in 0..2 {
            if self.check_row(i) {
                match self.grid[i][0] {
                    Some(true) => return GameState::XWin,
                    Some(false) => return GameState::OWin,
                    None => unreachable!(),
                };
            }

            if self.check_column(i) {
                match self.grid[0][i] {
                    Some(true) => return GameState::XWin,
                    Some(false) => return GameState::OWin,
                    None => unreachable!(),
                };
            }
        }

        return if self.moves < 9 {
            GameState::InProgress
        } else {
            GameState::Draw
        };
    }
}
