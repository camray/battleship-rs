use crate::engine::ship::Ship;
use crate::engine::types::{Direction, Point, Position};
use std::io::ErrorKind;

const MAP_SIZE: usize = 10;

#[derive(Debug)]
pub struct Field {
    pub spaces: [[bool; MAP_SIZE]; MAP_SIZE],
    pub ships: Vec<Ship>,
}

impl Field {
    pub fn new() -> Self {
        let spaces = [[false; MAP_SIZE]; MAP_SIZE];

        return Field {
            spaces,
            ships: Ship::default_ships(),
        };
    }

    pub fn is_ship_sunk(&self, ship: &Ship) -> bool {
        if !ship.is_placed() {
            return false;
        }
        let hits = self.get_ship_hits(ship);
        hits.iter().all(|hit| *hit)
    }

    pub fn get_ship_hits(&self, ship: &Ship) -> Vec<bool> {
        ship.get_points()
            .iter()
            .map(|point| {
                return self.spaces[point.x][point.y];
            })
            .collect()
    }

    pub fn get_ship_at_point(&self, point: Point) -> Option<&Ship> {
        for ship in &self.ships {
            let points = ship.get_points();
            if points
                .iter()
                .any(|test_point| test_point.x == point.x && test_point.y == point.y)
            {
                return Some(ship);
            } else {
                continue;
            }
        }

        None
    }

    pub fn shoot(&mut self, point: &Point) -> Result<&Ship, ErrorKind> {
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
        if let Some(ship) = ship {
            return Result::Ok(ship);
        } else {
            return Result::Err(ErrorKind::Other);
        }
    }

    /**
     * Determine if all ships have been placed
     */
    pub fn are_all_ships_placed(&self) -> bool {
        self.ships.iter().all(|s| s.is_placed())
    }

    /**
     * Determine if all ships have been sunk
     */
    pub fn are_all_ships_sunk(&self) -> bool {
        self.ships.iter().all(|s| self.is_ship_sunk(&s))
    }

    pub fn get_unplaced_ships(&self) -> Vec<&Ship> {
        self.ships
            .iter()
            .map(|s| s.clone())
            .filter(|s| !s.is_placed())
            .collect()
    }

    pub fn get_unaplaced_ship_names(&self) -> Vec<String> {
        self.get_unplaced_ships()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }

    pub fn get_placed_ships(&self) -> Vec<&Ship> {
        self.ships
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.is_placed())
            .collect()
    }

    /**
     * Get a vector of the a ships position
     */
    pub fn get_positions(ship: &Ship) -> Option<Vec<Point>> {
        if ship.position.is_none() {
            return None;
        }

        let mut positions: Vec<Point> = vec![Point { x: 0, y: 0 }; ship.size.into()];

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

    pub fn find_ship_on_point(&self, p: &Point) -> Option<&Ship> {
        for ship in &self.ships {
            let ship_positions = Field::get_positions(&ship);
            if ship_positions?
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
    pub fn place_ship(&mut self, name: String, position: Position) -> Result<(), ErrorKind> {
        let ship = self.ships.iter().find(|s| s.name.eq(&name));

        if ship.is_none() {
            return Result::Err(ErrorKind::Other);
        }

        let ship = ship.unwrap();

        if ship.is_placed() {
            return Result::Err(ErrorKind::AlreadyExists);
        }

        let iterator = vec![false; ship.size.into()];

        let positions = iterator
            .iter()
            .enumerate()
            .fold(vec![], |mut accum, (i, _)| {
                match position.direction {
                    Direction::Horizontal => {
                        accum.push((
                            Point {
                                x: position.point.x + i,
                                y: position.point.y,
                            },
                            false,
                        ));
                    }
                    Direction::Vertical => {
                        accum.push((
                            Point {
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

        let ship = self.ships.iter_mut().find(|s| s.name.eq(&name)).unwrap();

        ship.position = Some(position);

        Result::Ok(())
    }

    pub fn get_next_ship_to_place(&self) -> Option<&Ship> {
        self.get_unplaced_ships().first().map(|s| *s)
    }

    pub fn player_view(&self) -> String {
        let alphas = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
        let numerals = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        let mut result = String::new();
        result.push_str("----".repeat(MAP_SIZE + 1).as_str());
        result.push_str("-\n|   ");
        for i in numerals {
            result.push_str(format!("| {} ", i).as_str());
        }
        result.push_str("\n-");
        result.push_str("----".repeat(MAP_SIZE + 1).as_str());
        result.push_str("\n");
        for i in 0..MAP_SIZE {
            result.push_str(format!("| {} ", alphas[i]).as_str());
            for j in 0..MAP_SIZE {
                let ship_at_point = self.get_ship_at_point(Point { x: j, y: i });
                let has_ship_at_point = ship_at_point.is_some();
                result.push_str(&format!(
                    "| {} ",
                    if self.spaces[j][i] {
                        "X"
                    } else if has_ship_at_point {
                        &ship_at_point.unwrap().character
                    } else {
                        " "
                    }
                ));
            }
            result.push_str(&format!("|\n-"));
            result.push_str("----".repeat(MAP_SIZE + 1).as_str());
            result.push_str(&format!("\n"));
        }

        result
    }

    pub fn enemy_view(&self) -> String {
        let alphas = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
        let numerals = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        let mut result = String::new();
        result.push_str("----".repeat(MAP_SIZE + 1).as_str());
        result.push_str("-\n|   ");
        for i in numerals {
            result.push_str(format!("| {} ", i).as_str());
        }
        result.push_str("\n-");
        result.push_str("----".repeat(MAP_SIZE + 1).as_str());
        result.push_str("\n");
        for i in 0..MAP_SIZE {
            result.push_str(format!("| {} ", alphas[i]).as_str());
            for j in 0..MAP_SIZE {
                let ship_at_point = self.get_ship_at_point(Point { x: j, y: i });
                let has_ship_at_point = ship_at_point.is_some();
                result.push_str(&format!(
                    "| {} ",
                    if self.spaces[j][i] && has_ship_at_point {
                        "X"
                    } else if self.spaces[j][i] {
                        "-"
                    } else {
                        " "
                    }
                ));
            }
            result.push_str(&format!("|\n-"));
            result.push_str("----".repeat(MAP_SIZE + 1).as_str());
            result.push_str(&format!("\n"));
        }

        result
    }
}
