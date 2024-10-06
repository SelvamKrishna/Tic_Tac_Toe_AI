mod ai;
mod board;
mod user;
mod utils;

use std::io;

use ai::AI;
use board::{Board, GameState};
use user::User;
use utils::*;

fn get_user_turn() -> bool {
    println!("Do you want to play first? ( Y / N ): ");
    let mut user_input: String = String::new();
    loop {
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => match user_input.trim().to_uppercase().as_str() {
                "Y" => return true,
                "N" => return false,
                _ => {
                    warn("Invalid input. Please enter Y or N.");
                    continue;
                }
            },
            Err(_) => continue,
        }
    }
}

pub struct Master<'a> {
    board: Board,
    user: User,
    ai: AI<'a>,
    turn: bool,
}

impl<'a> Master<'a> {
    const LINE: &'a str = "-------------------";

    pub fn new() -> Self {
        return Master {
            board: Board::new(),
            user: User::new(),
            ai: AI::new(),
            turn: true,
        };
    }

    pub fn run(&mut self) {
        let display_board = |this: &Self| {
            println!("{}", Self::LINE);
            this.board.draw();
            println!("{}", Self::LINE);
        };

        let user_value: bool = get_user_turn();

        let mut input_cell: Coordinate;

        loop {
            match self.board.get_state() {
                GameState::InProgress => {}
                GameState::Draw => {
                    println!("Game Over! Its a Draw!");
                    break;
                }
                GameState::XWin => {
                    println!("Game Over! X is the Winner!");
                    break;
                }
                GameState::OWin => {
                    println!("Game Over! O is the Winner!");
                    break;
                }
            }

            display_board(self);
            println!("{}'s turn:", if self.turn { "X" } else { "O" });

            input_cell = match user_value == self.turn {
                true => self.user.choice(),
                false => self.ai.choice(),
            };

            match self.board.place(input_cell, self.turn) {
                Ok(_) => {}
                Err(msg) => {
                    warn(msg);
                    continue;
                }
            }
            self.turn = !self.turn;
        }

        display_board(self);
    }
}
