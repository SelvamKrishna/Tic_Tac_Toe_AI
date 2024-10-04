use crate::utils::{Coordinate, warn};

fn get_usize_input() -> usize
{
    loop 
    {
        print!("Enter row (0-2): ");
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().parse::<usize>()
        {
            Ok(val) =>
            { return val as usize; }
            Err(_) =>
            {
                warn("Please enter a number between 0 and 2.");
                continue;
            }
        };
    }
}



pub struct User(bool);

impl User
{
    pub fn new(val: bool) -> Self
    { return User(val); }

    pub fn get_input_coord(&self) -> Coordinate
    { return Coordinate::new(get_usize_input(), get_usize_input()); }
}