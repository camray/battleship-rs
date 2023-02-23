use crate::engine::types::Position;

use super::types::{Direction, Point};

#[derive(Debug)]
pub struct Ship {
    pub name: String,
    pub position: Option<Position>,
    pub size: u8,
    pub character: String,
}

impl Ship {
    pub fn is_placed(&self) -> bool {
        match &self.position {
            Some(p) => true,
            None => false,
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut result: Vec<Point> = vec![];

        match &self.position {
            None => result,
            Some(p) => {
                let mut i: usize = 0;

                while i < self.size.into() {
                    match p.direction {
                        Direction::Vertical => {
                            result.push(Point {
                                x: p.point.x,
                                y: p.point.y + i,
                            });
                        }
                        Direction::Horizontal => {
                            result.push(Point {
                                x: p.point.x + i,
                                y: p.point.y,
                            });
                        }
                    }

                    i += 1;
                }

                result
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
    pub fn default_ships() -> Vec<Ship> {
        vec![
            Ship {
                size: 5,
                position: None,
                name: "carrier".into(),
                character: "C".into(),
            },
            Ship {
                size: 4,
                position: None,
                name: "battleship".into(),
                character: "B".into(),
            },
            Ship {
                size: 3,
                position: None,
                name: "cruiser".into(),
                character: "R".into(),
            },
            Ship {
                size: 3,
                position: None,
                name: "submarine".into(),
                character: "S".into(),
            },
            Ship {
                size: 2,
                position: None,
                name: "destroyer".into(),
                character: "D".into(),
            },
        ]
    }
}
