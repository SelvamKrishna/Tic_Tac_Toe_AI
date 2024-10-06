use crate::board::Board;
use crate::utils::Coordinate;

struct ScoreSet {
    coordinate: Coordinate,
    score: i32,
}

pub struct AI<'a> {
    score_sets: Vec<ScoreSet>,
    board: Option<&'a Board>,
}

impl<'a> AI<'a> {
    pub fn new() -> Self {
        return AI {
            score_sets: vec![],
            board: None,
        };
    }

    pub fn set_board(&mut self, board: &'a Board) {
        self.board = Some(board)
    }

    pub fn choice(&self) -> Coordinate {
        if self.board.unwrap().get_moves() < 2 {
            if let None = self.board.unwrap().get(1, 1) {
                return Coordinate::new(1, 1);
            }
        }

        match self
            .score_sets
            .iter()
            .max_by(|x: &&ScoreSet, y: &&ScoreSet| x.score.cmp(&y.score))
        {
            Some(best_coords) => {
                return Coordinate::new(best_coords.coordinate.x(), best_coords.coordinate.y());
            }
            None => unreachable!(),
        }
    }
}
