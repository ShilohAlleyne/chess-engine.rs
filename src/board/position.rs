use std::{fmt, ops::{Index, IndexMut}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// The board positions
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
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

// Convert Position enum to u64
impl From<&Position> for u64 {
    fn from(value: &Position) -> Self {
        *value as u64
    }
}

// === Index for better composition ===
impl<T> Index<Position> for [T; 64] {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        &self[pos as usize]
    }
}

impl<T> IndexMut<Position> for [T; 64] {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self[pos as usize]
    }
}

// === Display ===
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Position::A8 => "A8",
            Position::B8 => "B8",
            Position::C8 => "C8",
            Position::D8 => "D8",
            Position::E8 => "E8",
            Position::F8 => "F8",
            Position::G8 => "G8",
            Position::H8 => "H8",
            Position::A7 => "A7",
            Position::B7 => "B7",
            Position::C7 => "C7",
            Position::D7 => "D7",
            Position::E7 => "E7",
            Position::F7 => "F7",
            Position::G7 => "G7",
            Position::H7 => "H7",
            Position::A6 => "A6",
            Position::B6 => "B6",
            Position::C6 => "C6",
            Position::D6 => "D6",
            Position::E6 => "E6",
            Position::F6 => "F6",
            Position::G6 => "G6",
            Position::H6 => "H6",
            Position::A5 => "A5",
            Position::B5 => "B5",
            Position::C5 => "C5",
            Position::D5 => "D5",
            Position::E5 => "E5",
            Position::F5 => "F5",
            Position::G5 => "G5",
            Position::H5 => "H5",
            Position::A4 => "A4",
            Position::B4 => "B4",
            Position::C4 => "C4",
            Position::D4 => "D4",
            Position::E4 => "E4",
            Position::F4 => "F4",
            Position::G4 => "G4",
            Position::H4 => "H4",
            Position::A3 => "A3",
            Position::B3 => "B3",
            Position::C3 => "C3",
            Position::D3 => "D3",
            Position::E3 => "E3",
            Position::F3 => "F3",
            Position::G3 => "G3",
            Position::H3 => "H3",
            Position::A2 => "A2",
            Position::B2 => "B2",
            Position::C2 => "C2",
            Position::D2 => "D2",
            Position::E2 => "E2",
            Position::F2 => "F2",
            Position::G2 => "G2",
            Position::H2 => "H2",
            Position::A1 => "A1",
            Position::B1 => "B1",
            Position::C1 => "C1",
            Position::D1 => "D1",
            Position::E1 => "E1",
            Position::F1 => "F1",
            Position::G1 => "G1",
            Position::H1 => "H1",
        };
        write!(f, "{}", label)
    }
}

impl Position {
    // Convert u64 to Position
    pub fn from_u64(value: u64) -> Option<Position> {
        if value >= 64 {
            None
        } else {
            Position::iter().find(|&p| p as u64 == value)
        }
    }

    // Conver u32 to Position
    pub(crate) fn from_u32(value: u32) -> Option<Position> {
        if value >= 64 {
            None
        } else {
            Position::iter().find(|&p| p as u32 == value)
        }
    }

    pub(crate) fn from_coords(rank: u64, file: u64) -> Option<Self> {
        Position::from_u64((7 - rank) * 8 + file)
    }

    pub fn from_chars(file: char, rank: char) -> Option<Self> {
        use Position::*;

        match (file, rank) {
            ('a', '8') => Some(A8),
            ('b', '8') => Some(B8),
            ('c', '8') => Some(C8),
            ('d', '8') => Some(D8),
            ('e', '8') => Some(E8),
            ('f', '8') => Some(F8),
            ('g', '8') => Some(G8),
            ('h', '8') => Some(H8),
            ('a', '7') => Some(A7),
            ('b', '7') => Some(B7),
            ('c', '7') => Some(C7),
            ('d', '7') => Some(D7),
            ('e', '7') => Some(E7),
            ('f', '7') => Some(F7),
            ('g', '7') => Some(G7),
            ('h', '7') => Some(H7),
            ('a', '6') => Some(A6),
            ('b', '6') => Some(B6),
            ('c', '6') => Some(C6),
            ('d', '6') => Some(D6),
            ('e', '6') => Some(E6),
            ('f', '6') => Some(F6),
            ('g', '6') => Some(G6),
            ('h', '6') => Some(H6),
            ('a', '5') => Some(A5),
            ('b', '5') => Some(B5),
            ('c', '5') => Some(C5),
            ('d', '5') => Some(D5),
            ('e', '5') => Some(E5),
            ('f', '5') => Some(F5),
            ('g', '5') => Some(G5),
            ('h', '5') => Some(H5),
            ('a', '4') => Some(A4),
            ('b', '4') => Some(B4),
            ('c', '4') => Some(C4),
            ('d', '4') => Some(D4),
            ('e', '4') => Some(E4),
            ('f', '4') => Some(F4),
            ('g', '4') => Some(G4),
            ('h', '4') => Some(H4),
            ('a', '3') => Some(A3),
            ('b', '3') => Some(B3),
            ('c', '3') => Some(C3),
            ('d', '3') => Some(D3),
            ('e', '3') => Some(E3),
            ('f', '3') => Some(F3),
            ('g', '3') => Some(G3),
            ('h', '3') => Some(H3),
            ('a', '2') => Some(A2),
            ('b', '2') => Some(B2),
            ('c', '2') => Some(C2),
            ('d', '2') => Some(D2),
            ('e', '2') => Some(E2),
            ('f', '2') => Some(F2),
            ('g', '2') => Some(G2),
            ('h', '2') => Some(H2),
            ('a', '1') => Some(A1),
            ('b', '1') => Some(B1),
            ('c', '1') => Some(C1),
            ('d', '1') => Some(D1),
            ('e', '1') => Some(E1),
            ('f', '1') => Some(F1),
            ('g', '1') => Some(G1),
            ('h', '1') => Some(H1),
            _ => None,
        }
    }

    // Position manipluation
    pub fn rank(&self) -> usize {
        7 - ((*self as usize) / 8)
    }

    pub fn file(&self) -> usize {
        (*self as usize) % 8
    }

    pub fn change_rank(&self, delta: i8) -> Option<Self> {
        let rank = self.rank() as i8 + delta;
        let file = self.file() as i8;

        if (0..8).contains(&rank) {
            let actual_rank = 7 - rank;
            let index = actual_rank * 8 + file;
            Position::from_u64(index as u64)
        } else {
            None
        }
    }

    pub fn add_file(&self, delta: i8) -> Option<Self> {
        let rank = self.rank() as i8;
        let file = self.file() as i8 + delta;

        if (0..8).contains(&file) {
            let index = rank * 8 + file;
            Position::from_u64(index as u64)
        } else {
            None
        }
    }
}

// Castling bit binaary representation
#[derive(Debug, Clone, Copy)]
pub(crate) enum CastlingRights {
    None,
    WK,
    WQ,
    RK,
    RQ,
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastlingRights::None => writeln!(f, "None"),
            CastlingRights::WK => write!(f, "White King side"),
            CastlingRights::WQ => write!(f, "White Queen side"),
            CastlingRights::RK => write!(f, "Red King side"),
            CastlingRights::RQ => write!(f, "Red Queen side"),
        }
    }
}

impl CastlingRights {
    pub fn get_castlings_bits(&self) -> u8 {
        match self {
            CastlingRights::None => 0,
            CastlingRights::WK => 1,
            CastlingRights::WQ => 2,
            CastlingRights::RK => 4,
            CastlingRights::RQ => 8,
        }
    }
}
