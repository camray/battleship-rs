#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

const MAP_SIZE: usize = 10;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Vertical,
    Horizontal,
}

/**
 * Tuple of the point on the map and a bool of whether or not a strike happened
 */
type Position = (Point, bool);

#[derive(Debug)]
struct Ship {
    positions: Option<Vec<Position>>,
    size: u8,
}

impl Ship {
    fn is_sunk(&self) -> bool {
        match &self.positions {
            Some(p) => p.iter().all(|p| p.1),
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

    // /**
    //  * Determine if a ship resides on a given point
    //  */
    // fn find_ship_on_point(point: Point) -> Ship {

    //     true
    // }

    fn place_ship(&mut self, name: String, position: Point, direction: Direction) -> bool {
        let ship = self.ships.get_mut(&name);
        match ship {
            Some(s) => {
                let positions = vec![false; s.size.into()];

                s.positions = Some(positions.iter().enumerate().fold(
                    vec![],
                    |mut accum, (i, _)| {
                        match direction {
                            Direction::Horizontal => {
                                accum.push((
                                    Point {
                                        x: position.x + i,
                                        y: position.y,
                                    },
                                    false,
                                ));
                            }
                            Direction::Vertical => {
                                accum.push((
                                    Point {
                                        x: position.x,
                                        y: position.y + i,
                                    },
                                    false,
                                ));
                            }
                        }
                        accum
                    },
                ));

                true
            }
            None => false,
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
        Point { x: 1, y: 2 },
        Direction::Horizontal,
    );
    let u2 = User::new("Maya".into());

    println!("{:?}", u1.field);
}
