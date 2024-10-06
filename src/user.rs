use crate::utils::{warn, Coordinate};

pub struct User;

const INPUT_CELL: [(usize, usize); 9] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

impl User {
    pub fn new() -> Self {
        return User;
    }

    pub fn choice(&self) -> Coordinate {
        loop {
            println!("Enter cell (1-9): ");
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u8>() {
                Ok(val) => {
                    if val < 10 {
                        let (x, y) = INPUT_CELL[val as usize - 1];
                        return Coordinate::new(x, y);
                    }
                }
                Err(_) => {}
            }
            warn("Please enter a number between 1 and 9.");
            continue;
        }
    }
}
