#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Position {
    pub point: Point,
    pub direction: Direction,
}
