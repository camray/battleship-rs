#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

/**
 * Tuple of the point on the map and a bool of whether or not a strike happened
 */
pub type Position = (Point, bool);
