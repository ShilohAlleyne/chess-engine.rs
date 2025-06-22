use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// The board positions
#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Position {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

// Convert Position enum to u64
impl From<Position> for u64 {
    fn from(value: Position) -> Self {
        value as u64
    }
}

impl Position {
    // Convert u64 to Position
    pub fn from_u64(value: u64) -> Option<Position> {
        if value > 64 {
            None
        } else {
            // Defaults to INVALID Position if error
            Position::iter()
                .take_while_inclusive(|&p| (p as u64) < value)
                .last()
        }
    }

    // Conver u32 to Position
    pub fn from_u32(value: u32) -> Option<Position> {
        if value > 64 {
            None
        } else {
            Position::iter()
                .take_while_inclusive(|&p| (p as u32) < value)
                .last()
        }
    }

    pub fn from_coords(rank: u64, file: u64) -> Option<Self> {
        Position::from_u64(rank * 8 + file)
    }
}

// Player colour
pub enum Colour {
    White,
    Black,
}

impl From<bool> for Colour {
    fn from(value: bool) -> Colour {
        match value {
            false => Colour::White,
            true  => Colour::Black,
        }
    }
}
