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
            println!("Enter cell (0-8): ");
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(val) = input.trim().parse::<u8>() {
                let (x, y) = INPUT_CELL[val as usize - 1];
                return Coordinate::new(x, y);
            } else {
                warn("Please enter a number between 0 and 8.");
                continue;
            }
        }
    }
}
