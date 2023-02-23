#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, io::ErrorKind};
mod types;
const MAP_SIZE: usize = 10;

#[derive(Debug)]
struct Ship {
    positions: Option<Vec<types::Position>>,
    size: u8,
}

impl Ship {
    fn is_sunk(&self) -> bool {
        match &self.positions {
            Some(p) => p.iter().all(|p| p.1),
            None => false,
        }
    }

    fn is_placed(&self) -> bool {
        match &self.positions {
            Some(p) => true,
            None => false,
        }
    }
}

/**
 * 5 ships:
 *
 * Carrier: 5 strikes
 * Battleship: 4 strikes
 * Cruiser: 3 strikes
 * Submarine: 3 strikes
 * Destroyer: 2 strikes
 */
fn default_ships() -> HashMap<String, Ship> {
    HashMap::from([
        (
            "carrier".into(),
            Ship {
                size: 5,
                positions: None,
            },
        ),
        (
            "battleship".into(),
            Ship {
                size: 4,
                positions: None,
            },
        ),
        (
            "cruiser".into(),
            Ship {
                size: 3,
                positions: None,
            },
        ),
        (
            "submarine".into(),
            Ship {
                size: 3,
                positions: None,
            },
        ),
        (
            "destroyer".into(),
            Ship {
                size: 2,
                positions: None,
            },
        ),
    ])
}

#[derive(Debug)]
struct Field {
    spaces: [[bool; MAP_SIZE]; MAP_SIZE],
    ships: HashMap<String, Ship>,
}

impl Field {
    fn new() -> Self {
        let spaces = [[false; MAP_SIZE]; MAP_SIZE];

        return Field {
            spaces,
            ships: default_ships(),
        };
    }

    fn shoot(&self, x: usize, y: usize) -> bool {
        self.spaces[x][y]
    }

    /**
     * Find the ship which resides on a given point
     */
    fn find_ship_on_point(&self, point: &types::Point) -> Option<&Ship> {
        match self.ships.iter().find(|s| match &s.1.positions {
            Some(s) => s.iter().any(|p| &p.0 == point),
            None => false,
        }) {
            Some(s) => Some(s.1),
            None => None,
        }
    }

    /**
     * Determine if all ships have been placed
     */
    fn are_all_ships_placed(&self) -> bool {
        self.ships.iter().all(|s| s.1.is_placed())
    }

    /**
     * 1) Ensure that a ship has not already been placed
     * 2) Ensure that you are not placing the ship on top of an existing ship
     * 3) Ensure that the ship can fit on the board
     */
    fn place_ship(
        &mut self,
        name: String,
        position: types::Point,
        direction: types::Direction,
    ) -> Result<(), ErrorKind> {
        let ship = self.ships.get_mut(&name);

        match ship {
            Some(s) => {
                // Ship is already placed
                if s.is_placed() {
                    return Result::Err(ErrorKind::AlreadyExists);
                }

                let iterator = vec![false; s.size.into()];

                let positions = iterator
                    .iter()
                    .enumerate()
                    .fold(vec![], |mut accum, (i, _)| {
                        match direction {
                            types::Direction::Horizontal => {
                                accum.push((
                                    types::Point {
                                        x: position.x + i,
                                        y: position.y,
                                    },
                                    false,
                                ));
                            }
                            types::Direction::Vertical => {
                                accum.push((
                                    types::Point {
                                        x: position.x,
                                        y: position.y + i,
                                    },
                                    false,
                                ));
                            }
                        }
                        accum
                    });

                // There is a ship already existing on that point
                if positions
                    .iter()
                    .any(|p| return self.find_ship_on_point(&p.0).is_some())
                {
                    return Result::Err(ErrorKind::Other);
                }

                // The ship is off the board
                if positions
                    .iter()
                    .any(|p| p.0.x > MAP_SIZE || p.0.y > MAP_SIZE)
                {
                    return Result::Err(ErrorKind::Other);
                }

                Result::Ok(())
            }
            // Could not find ship
            None => Result::Err(ErrorKind::Other),
        }
    }
}

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
        types::Point { x: 1, y: 2 },
        types::Direction::Horizontal,
    );
    println!("{}", u1.field.are_all_ships_placed());
    let u2 = User::new("Maya".into());

    println!("{:?}", u1.field);
}
