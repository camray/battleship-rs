use crate::engine::types::Position;

#[derive(Debug)]
pub struct Ship {
    pub name: String,
    pub position: Option<Position>,
    pub size: u8,
}

impl Ship {
    pub fn is_placed(&self) -> bool {
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
    pub fn default_ships() -> Vec<Ship> {
        vec![
            Ship {
                size: 5,
                position: None,
                name: "carrier".into(),
            },
            Ship {
                size: 4,
                position: None,
                name: "battleship".into(),
            },
            Ship {
                size: 3,
                position: None,
                name: "cruiser".into(),
            },
            Ship {
                size: 3,
                position: None,
                name: "submarine".into(),
            },
            Ship {
                size: 2,
                position: None,
                name: "destroyer".into(),
            },
        ]
    }
}
