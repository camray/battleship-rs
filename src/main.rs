#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, io::ErrorKind};

use types::{Direction, Point};
mod types;
const MAP_SIZE: usize = 10;

#[derive(Debug)]
struct Ship {
    position: Option<types::Position>,
    size: u8,
}

impl Ship {
    fn is_placed(&self) -> bool {
        match &self.position {
            Some(p) => true,
            None => false,
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
                    position: None,
                },
            ),
            (
                "battleship".into(),
                Ship {
                    size: 4,
                    position: None,
                },
            ),
            (
                "cruiser".into(),
                Ship {
                    size: 3,
                    position: None,
                },
            ),
            (
                "submarine".into(),
                Ship {
                    size: 3,
                    position: None,
                },
            ),
            (
                "destroyer".into(),
                Ship {
                    size: 2,
                    position: None,
                },
            ),
        ])
    }
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
            ships: Ship::default_ships(),
        };
    }

    fn is_ship_sunk(&self, ship: &Ship) -> Option<bool> {
        match self.get_ship_hits(ship) {
            Some(hits) => Some(hits.iter().all(|hit| *hit)),
            None => None,
        }
    }

    fn get_ship_hits(&self, ship: &Ship) -> Option<Vec<bool>> {
        match self.get_ship_points(ship) {
            None => None,
            Some(p) => Some(
                p.iter()
                    .map(|point| {
                        return self.spaces[point.x][point.y];
                    })
                    .collect(),
            ),
        }
    }

    fn get_ship_points(&self, ship: &Ship) -> Option<Vec<types::Point>> {
        let mut i: usize = 0;
        let mut result: Vec<types::Point> = vec![];

        match &ship.position {
            None => None,
            Some(p) => {
                while i < ship.size.into() {
                    match p.direction {
                        Direction::Vertical => {
                            result.push(types::Point {
                                x: p.point.x,
                                y: p.point.x + i,
                            });
                        }
                        Direction::Horizontal => {
                            result.push(types::Point {
                                x: p.point.x + i,
                                y: p.point.x,
                            });
                        }
                    }
                    i += 1;
                }
                Some(result)
            }
        }
    }

    fn get_ship_at_point(&self, point: types::Point) -> Option<Ship> {
        for ship in &self.ships {
            match self.get_ship_points(&ship.1) {
                Some(points) => {
                    if points
                        .iter()
                        .any(|test_point| test_point.x == point.x && test_point.y == point.y)
                    {
                        Some(ship)
                    } else {
                        continue;
                    }
                }
                None => {
                    continue;
                }
            };
        }

        None
    }

    fn shoot(&mut self, point: &types::Point) -> Result<Ship, ErrorKind> {
        // Shot was already taken here
        if self.spaces[point.x][point.y] {
            return Result::Err(ErrorKind::Other);
        }

        // Shot off the map
        if point.x > MAP_SIZE || point.y > MAP_SIZE {
            return Result::Err(ErrorKind::Other);
        }

        self.spaces[point.x][point.y] = true;
        let ship = self.get_ship_at_point(*point);
        Result::Ok(ship.expect(""))
    }

    /**
     * Determine if all ships have been placed
     */
    fn are_all_ships_placed(&self) -> bool {
        self.ships.iter().all(|s| s.1.is_placed())
    }

    /**
     * Get a vector of the a ships position
     */
    fn get_positions(ship: &Ship) -> Option<Vec<Point>> {
        let mut positions: Vec<Point> = vec![Point { x: 0, y: 0 }; ship.size.into()];

        if ship.position.is_none() {
            return None;
        }

        let ship_position = ship.position.as_ref().unwrap();
        match ship_position.direction {
            Direction::Horizontal => {
                for (i, new_position) in positions.iter_mut().enumerate() {
                    new_position.y = ship_position.point.y;
                    new_position.x = ship_position.point.x + i;
                }
                return Some(positions);
            }
            Direction::Vertical => {
                for (i, new_position) in positions.iter_mut().enumerate() {
                    new_position.y = ship_position.point.y + i;
                    new_position.x = ship_position.point.x;
                }
                return Some(positions);
            }
        }
    }

    fn find_ship_on_point(&self, p: &Point) -> Option<&Ship> {
        for (name, ship) in &self.ships {
            let ship_positions = Field::get_positions(&ship);
            if ship_positions.is_some()
                && ship_positions
                    .expect("")
                    .iter()
                    .find(|maybe_point| maybe_point.x == p.x && maybe_point.y == p.y)
                    .is_some()
            {
                return Some(&ship);
            }
        }
        return None;
    }

    /**
     * 1) Ensure that a ship has not already been placed
     * 2) Ensure that you are not placing the ship on top of an existing ship
     * 3) Ensure that the ship can fit on the board
     */
    fn place_ship(&mut self, name: String, position: types::Position) -> Result<(), ErrorKind> {
        let ship = self.ships.get(&name);

        if ship.is_none() {
            return Result::Err(ErrorKind::Other);
        }

        let ship = ship.expect("How did we get here?");

        // Ship is already placed
        if ship.is_placed() {
            return Result::Err(ErrorKind::AlreadyExists);
        }

        let iterator = vec![false; ship.size.into()];

        let positions = iterator
            .iter()
            .enumerate()
            .fold(vec![], |mut accum, (i, _)| {
                match position.direction {
                    types::Direction::Horizontal => {
                        accum.push((
                            types::Point {
                                x: position.point.x + i,
                                y: position.point.y,
                            },
                            false,
                        ));
                    }
                    types::Direction::Vertical => {
                        accum.push((
                            types::Point {
                                x: position.point.x,
                                y: position.point.y + i,
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

        let ship = self.ships.get_mut(&name).unwrap();

        ship.position = Some(position);

        Result::Ok(())
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
        types::Position {
            direction: Direction::Horizontal,
            point: types::Point { x: 0, y: 0 },
        },
    ).unwrap();
    println!("{}", u1.field.are_all_ships_placed());
    let u2 = User::new("Maya".into());

    println!("{:?}", u1.field);
}
