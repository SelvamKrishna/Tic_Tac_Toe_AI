pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        return Coordinate { x, y };
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

pub trait Choice {
    fn choice(&self) -> Coordinate;
}

pub fn warn(msg: &str) {
    eprintln!("WARN: {}", msg);
}
