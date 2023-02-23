#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub point: Point,
    pub direction: Direction,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Horizontal => write!(f, "Horizontal"),
            Direction::Vertical => write!(f, "Vertical"),
        }
    }
}
