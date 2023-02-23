use std::io::ErrorKind;

use super::field;
use super::types::{Point, Direction};

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

    active_user_name: String,
}

impl Game {
    pub fn new(u1_name: String, u2_name: String) -> Self {
        let u1 = User::new(u1_name.clone());
        let u2 = User::new(u2_name);

        Self {
            u1,
            u2,
            active_user_name: u1_name,
        }
    }

    pub fn start_next_turn(&mut self) {
    }
}

const ALPHAS: [&str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
const NUMERALS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

pub fn convert_input_to_point(input: &str) -> Result<Point, ErrorKind> {
    let input = input.to_uppercase();
    let alpha = input[0..1].to_string();
    let numeral = input[1..].to_string(); // Numeral could be "10" which is 2 chars long

    // Find the index of the alpha and numeral in the arrays
    let x = NUMERALS.iter().position(|&r| numeral.eq(r));
    let y = ALPHAS.iter().position(|&r| alpha.eq(r));

    if x.is_some() && y.is_some() {
        return Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
        });
    }

    Err(ErrorKind::Other)
}

pub fn convert_input_to_direction(input: &str) -> Result<Direction, ErrorKind> {
    match input {
        "h" => Ok(Direction::Horizontal),
        "v" => Ok(Direction::Vertical),
        _ => Err(ErrorKind::Other),
    }
}

pub fn convert_point_to_alpha_numeral(point: &Point) -> String {
    format!("{}{}", ALPHAS[point.y], NUMERALS[point.x])
}
