mod board;
mod user;
mod utils;

use std::io;

use crate::board::Board;
use crate::user::User;
use crate::utils::{warn, Choice, Coordinate};

pub struct Master {
    board: Board,
    user: (User, bool),
    turn: bool,
}

pub fn get_user_turn() -> bool {
    println!("Do you want to play first? ( Y / N ): ");
    let mut user_input = String::new();
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

impl Master {
    pub fn new() -> Self {
        let user_turn = get_user_turn();

        return Master {
            board: Board::new(),
            user: (User::new(), user_turn),
            turn: true,
        };
    }
}
