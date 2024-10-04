pub fn warn(msg: &str)
{ eprintln!("WARN: {}", msg); }

pub fn get_usize_input() -> usize
{
    loop {
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

pub struct Coord
{
    x: usize,
    y: usize,
}

impl Coord 
{
    pub fn new(x: usize, y: usize) -> Self
    { return Coord{x, y}; }
}

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
        coord: Coord,
        val: bool
    ) -> Result<bool, &str>
    {
        match self.grid[coord.x][coord.y] 
        {
            Some(_) => return Err(""),
            None => 
            {
                self.grid[coord.x][coord.y] = Some(val);
                self.moves += 1;
                return Ok(val);
            }
        }
    }

    pub fn is_full(&self) -> bool { return self.moves >= 9; }

    pub fn is_winner(&self) -> Option<bool>
    {
        unimplemented!();
    }

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

pub struct User(bool);

impl User
{
    pub fn new(val: bool) -> Self
    { return User(val); }

    pub fn get_input_coord(&self) -> Coord
    { return Coord::new(get_usize_input(), get_usize_input()); }

    pub fn play(&self, board: &mut Board)
    {
        let coord: Coord = self.get_input_coord();
        if let Err(msg) = board.place(coord, self.0)
        {
            warn(msg);
            return;
        }
    }
}