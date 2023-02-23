use std::io::ErrorKind;

use super::field;
use super::types::{Direction, Point};

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

    pub active_user_name: String,
    pub non_active_user_name: String,
}

impl Game {
    pub fn new(u1_name: String, u2_name: String) -> Self {
        let u1 = User::new(u1_name.clone());
        let u2 = User::new(u2_name.clone());

        Self {
            u1,
            u2,
            active_user_name: u1_name,
            non_active_user_name: u2_name,
        }
    }

    pub fn start_next_turn(&mut self) {
        if self.active_user_name.eq(&self.u1.name) {
            self.active_user_name = self.u2.name.clone();
            self.non_active_user_name = self.u1.name.clone();
        } else {
            self.active_user_name = self.u1.name.clone();
            self.non_active_user_name = self.u2.name.clone();
        };
    }

    pub fn are_all_ships_placed(&self) -> bool {
        self.u1.field.are_all_ships_placed() && self.u2.field.are_all_ships_placed()
    }

    pub fn get_active_user(&self) -> &User {
        if self.active_user_name.eq(&self.u1.name) {
            &self.u1
        } else {
            &self.u2
        }
    }

    pub fn get_active_user_mut(&mut self) -> &mut User {
        if self.active_user_name.eq(&self.u1.name) {
            &mut self.u1
        } else {
            &mut self.u2
        }
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
    match input.to_uppercase().as_str() {
        "H" => Ok(Direction::Horizontal),
        "V" => Ok(Direction::Vertical),
        _ => Err(ErrorKind::Other),
    }
}

pub fn convert_point_to_alpha_numeral(point: &Point) -> String {
    format!("{}{}", ALPHAS[point.y], NUMERALS[point.x])
}
