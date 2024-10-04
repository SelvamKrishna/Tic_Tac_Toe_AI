use tic_tac_toe::{Board, Coord};

fn main() {
    let mut board: Board = Board::new();
    board.place(Coord::new(1, 1), true).unwrap();
    board.draw();
}
