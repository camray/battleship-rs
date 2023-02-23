use std::io::ErrorKind;

use super::field;
use super::types::Point;

pub struct User {
    pub name: String,
    pub field: field::Field,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            field: field::Field::new(),
        }
    }
}

pub struct Game {
    pub u1: User,
    pub u2: User,
}

impl Game {
    pub fn new(u1: String, u2: String) -> Self {
        Self {
            u1: User::new(u1),
            u2: User::new(u2),
        }
    }
}

const ALPHAS: [&str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
const NUMERALS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

pub fn convert_input_to_point(input: &String) -> Result<Point, ErrorKind> {
    let alpha = input[0..1].to_string();
    let numeral = input[1..].to_string(); // Numeral could be "10" which is 2 chars long

    // Find the index of the alpha and numeral in the arrays
    let x = NUMERALS.iter().position(|&r| r == numeral);
    let y = ALPHAS.iter().position(|&r| r == alpha);

    if x.is_some() && y.is_some() {
        return Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
        });
    }

    Err(ErrorKind::Other)
}

pub fn convert_point_to_alpha_numeral(point: &Point) -> String {
    format!("{}{}", ALPHAS[point.y], NUMERALS[point.x])
}
