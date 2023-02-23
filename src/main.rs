#![allow(dead_code)]
#![allow(unused_variables)]

mod types;
use types::{Direction};
mod field;
use field::Field;
mod ship;

struct User {
    name: String,
    field: Field,
}

impl User {
    fn new(name: String) -> Self {
        Self {
            name,
            field: Field::new(),
        }
    }
}

struct Game {
    u1: User,
    u2: User,
}

fn main() {
    let mut u1 = User::new("Cam".into());
    u1.field.place_ship(
        "destroyer".into(),
        types::Position {
            direction: Direction::Horizontal,
            point: types::Point { x: 0, y: 0 },
        },
    ).unwrap();
    println!("{}", u1.field.are_all_ships_placed());
    let u2 = User::new("Maya".into());

    println!("{:?}", u1.field);
}
