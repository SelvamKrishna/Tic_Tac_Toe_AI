use crate::utils::Coordinate;

pub struct Board
{
    grid: [[Option<bool>; 3]; 3],
    moves: u8,
}

impl Board
{
    pub fn new() -> Self
    { 
        return Board 
        { 
            grid: [[None; 3]; 3],
            moves: 0,
        };
    }

    pub fn place(
        &mut self,
        coord: Coordinate,
        val: bool
    ) -> Result<bool, &str>
    {
        match self.grid[coord.x()][coord.y()] 
        {
            Some(_) => return Err("Cell is already occupied"),
            None => 
            {
                self.grid[coord.x()][coord.y()] = Some(val);
                self.moves += 1;
                return Ok(val);
            }
        }
    }

    pub fn is_full(&self) -> bool { return self.moves >= 9; }

    pub fn draw(&self)
    {
        for row in &self.grid
        {
            for &cell in row
            {
                let cell_val: char = match cell
                {
                    Some(true) => 'X',
                    Some(false) => 'O',
                    None => ' ',
                };
                print!("|{}|", cell_val);
            }
            println!();
        }
        println!();
    }
}