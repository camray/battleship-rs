use std::collections::HashMap;

use crate::types::Position;

#[derive(Debug)]
pub struct Ship {
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
    pub fn default_ships() -> HashMap<String, Ship> {
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