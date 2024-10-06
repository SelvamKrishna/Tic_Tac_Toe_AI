use crate::utils::*;

pub struct User;

impl User {
    pub fn new() -> Self {
        return User;
    }

    pub fn choice(&self) -> Coordinate {
        let get_usize_input = |dimension: &str| loop {
            println!("Enter {} (0-2): ", dimension);
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(val) => {
                    return val as usize;
                }
                Err(_) => {
                    warn("Please enter a number between 0 and 2.");
                    continue;
                }
            };
        };
        return Coordinate::new(get_usize_input("row"), get_usize_input("column"));
    }
}
