#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct Ship {
    name: String,
    hits: Vec<bool>,
    position: Option<Point>,
    direction: Option<Direction>,
}

impl Ship {
    fn is_sunk(&self) -> bool {
        self.hits.iter().all(|h| *h)
    }
}

/**
 * 5 ships:
 *
 * Carrier: 5 holes
 * Battleship: 4 holes
 * Cruiser: 3 holes
 * Submarine: 3 holes
 * Destroyer: 2 holes
 */
fn default_ships() -> Vec<Ship> {
    vec![
        Ship {
            name: "Carrier".into(),
            hits: vec![false; 5],
            position: None,
            direction: None,
        },
        Ship {
            name: "Battleship".into(),
            hits: vec![false; 4],
            position: None,
            direction: None,
        },
        Ship {
            name: "Cruiser".into(),
            hits: vec![false; 3],
            position: None,
            direction: None,
        },
        Ship {
            name: "Submarine".into(),
            hits: vec![false; 3],
            position: None,
            direction: None,
        },
        Ship {
            name: "Destroyer".into(),
            hits: vec![false; 2],
            position: None,
            direction: None,
        },
    ]
}

#[derive(Debug)]
struct Field {
    spaces: [[bool; 10]; 10],
    ships: Vec<Ship>,
}

impl Field {
    fn new() -> Self {
        let spaces = [[false; 10]; 10];

        // let ships: Vec<Ship> =
        return Field {
            spaces,
            ships: default_ships(),
        };
    }

    fn shoot(&self, x: usize, y: usize) -> bool {
        self.spaces[x][y]
    }

    fn place_ship(&mut self, name: String, position: Point, direction: Direction) -> bool {
        self.ships
            .iter_mut()
            .find(|s| s.name == name)
            .map(|s| {
                s.position = Some(Point {
                    x: position.x,
                    y: position.y,
                });

                s.direction = Some(direction);
                true
            })
            .unwrap_or(false)
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
        "Destroyer".into(),
        Point { x: 1, y: 2 },
        Direction::Horizontal,
    );
    let u2 = User::new("Maya".into());

    println!("{:?}", u1.field);
}

// Generate game board
// Create new matrix n*n
// Get vec of pieces
// Choose if piece is vertical or horizontal
//

// Game board: 10x10
// Ships may be horizontal or vertical
