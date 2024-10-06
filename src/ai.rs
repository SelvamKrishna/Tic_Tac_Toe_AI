use crate::board::{Board, GameState};
use crate::utils::Coordinate;

pub struct AI(bool);

impl AI {
    pub fn new(turn: bool) -> Self {
        return AI(turn);
    }

    fn get_empty_cells(grid: &[[Option<bool>; 3]; 3], empty_cells: &mut Vec<Coordinate>) {
        *empty_cells = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                if grid[x][y].is_none() {
                    empty_cells.push(Coordinate::new(x, y));
                }
            }
        }
    }

    fn evalurate_state(turn: bool, board: &Board) -> Option<i8> {
        match board.get_state() {
            GameState::InProgress => None,
            GameState::Draw => return Some(0),
            GameState::XWin => return if turn { Some(1) } else { Some(-1) },
            GameState::OWin => return if turn { Some(-1) } else { Some(1) },
        }
    }

    fn minmax(&self, board: Board, minimizing: bool) -> (i8, Option<Coordinate>) {
        match AI::evalurate_state(self.0, &board) {
            Some(val) => return (val, None),
            None => {}
        }

        let mut score: i8;
        let mut empty_cells: Vec<Coordinate> = vec![];
        AI::get_empty_cells(board.get(), &mut empty_cells);

        let mut best_move = empty_cells[0].clone();

        if minimizing {
            score = 100;

            for cell in empty_cells {
                let mut new_board: Board = board.clone();
                match new_board.place(&cell, !self.0) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("AI wasn't able to place in cell");
                    }
                }
                let (new_score, _) = AI::minmax(self, new_board, false);

                if new_score < score {
                    score = new_score;
                    best_move = cell.clone();
                }
            }
        } else {
            score = -100;

            for cell in empty_cells {
                let mut new_board: Board = board.clone();
                match new_board.place(&cell, self.0) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("AI wasn't able to place in cell");
                    }
                }
                let (new_score, _) = AI::minmax(self, new_board, true);

                if new_score > score {
                    score = new_score;
                    best_move = cell.clone();
                }
            }
        }

        return (score, Some(best_move));
    }

    pub fn choice(&self, board: &Board) -> Coordinate {
        if let (_, Some(coordinate)) = self.minmax(board.clone(), false) {
            return coordinate;
        } else {
            unreachable!();
        }
    }
}
