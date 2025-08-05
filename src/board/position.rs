use std::{fmt, ops::{Index, IndexMut}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::error;

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

impl TryFrom<u8> for Position {
    type Error = error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0x3F {
            0 => Ok(Position::A8),
            1 => Ok(Position::B8),
            2 => Ok(Position::C8),
            3 => Ok(Position::D8),
            4 => Ok(Position::E8),
            5 => Ok(Position::F8),
            6 => Ok(Position::G8),
            7 => Ok(Position::H8),

            8 => Ok(Position::A7),
            9 => Ok(Position::B7),
            10 => Ok(Position::C7),
            11 => Ok(Position::D7),
            12 => Ok(Position::E7),
            13 => Ok(Position::F7),
            14 => Ok(Position::G7),
            15 => Ok(Position::H7),

            16 => Ok(Position::A6),
            17 => Ok(Position::B6),
            18 => Ok(Position::C6),
            19 => Ok(Position::D6),
            20 => Ok(Position::E6),
            21 => Ok(Position::F6),
            22 => Ok(Position::G6),
            23 => Ok(Position::H6),

            24 => Ok(Position::A5),
            25 => Ok(Position::B5),
            26 => Ok(Position::C5),
            27 => Ok(Position::D5),
            28 => Ok(Position::E5),
            29 => Ok(Position::F5),
            30 => Ok(Position::G5),
            31 => Ok(Position::H5),

            32 => Ok(Position::A4),
            33 => Ok(Position::B4),
            34 => Ok(Position::C4),
            35 => Ok(Position::D4),
            36 => Ok(Position::E4),
            37 => Ok(Position::F4),
            38 => Ok(Position::G4),
            39 => Ok(Position::H4),

            40 => Ok(Position::A3),
            41 => Ok(Position::B3),
            42 => Ok(Position::C3),
            43 => Ok(Position::D3),
            44 => Ok(Position::E3),
            45 => Ok(Position::F3),
            46 => Ok(Position::G3),
            47 => Ok(Position::H3),

            48 => Ok(Position::A2),
            49 => Ok(Position::B2),
            50 => Ok(Position::C2),
            51 => Ok(Position::D2),
            52 => Ok(Position::E2),
            53 => Ok(Position::F2),
            54 => Ok(Position::G2),
            55 => Ok(Position::H2),

            56 => Ok(Position::A1),
            57 => Ok(Position::B1),
            58 => Ok(Position::C1),
            59 => Ok(Position::D1),
            60 => Ok(Position::E1),
            61 => Ok(Position::F1),
            62 => Ok(Position::G1),
            63 => Ok(Position::H1),

            _ => Err(error::Error::TypeCoversiton(format!("Invalid u8 for position: {}", value))),
        }
    }
}

impl From<Position> for u8 {
    fn from(pos: Position) -> Self {
        match pos {
            Position::A8 => 0,  Position::B8 => 1,  Position::C8 => 2,  Position::D8 => 3,
            Position::E8 => 4,  Position::F8 => 5,  Position::G8 => 6,  Position::H8 => 7,

            Position::A7 => 8,  Position::B7 => 9,  Position::C7 => 10, Position::D7 => 11,
            Position::E7 => 12, Position::F7 => 13, Position::G7 => 14, Position::H7 => 15,

            Position::A6 => 16, Position::B6 => 17, Position::C6 => 18, Position::D6 => 19,
            Position::E6 => 20, Position::F6 => 21, Position::G6 => 22, Position::H6 => 23,

            Position::A5 => 24, Position::B5 => 25, Position::C5 => 26, Position::D5 => 27,
            Position::E5 => 28, Position::F5 => 29, Position::G5 => 30, Position::H5 => 31,

            Position::A4 => 32, Position::B4 => 33, Position::C4 => 34, Position::D4 => 35,
            Position::E4 => 36, Position::F4 => 37, Position::G4 => 38, Position::H4 => 39,

            Position::A3 => 40, Position::B3 => 41, Position::C3 => 42, Position::D3 => 43,
            Position::E3 => 44, Position::F3 => 45, Position::G3 => 46, Position::H3 => 47,

            Position::A2 => 48, Position::B2 => 49, Position::C2 => 50, Position::D2 => 51,
            Position::E2 => 52, Position::F2 => 53, Position::G2 => 54, Position::H2 => 55,

            Position::A1 => 56, Position::B1 => 57, Position::C1 => 58, Position::D1 => 59,
            Position::E1 => 60, Position::F1 => 61, Position::G1 => 62, Position::H1 => 63,
        }
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

pub(crate) fn to_string(pos: Position) -> String {
    match pos {
        Position::A8 => "a8",
        Position::B8 => "b8",
        Position::C8 => "c8",
        Position::D8 => "d8",
        Position::E8 => "e8",
        Position::F8 => "f8",
        Position::G8 => "g8",
        Position::H8 => "h8",
        Position::A7 => "a7",
        Position::B7 => "b7",
        Position::C7 => "c7",
        Position::D7 => "d7",
        Position::E7 => "e7",
        Position::F7 => "f7",
        Position::G7 => "g7",
        Position::H7 => "h7",
        Position::A6 => "a6",
        Position::B6 => "b6",
        Position::C6 => "c6",
        Position::D6 => "d6",
        Position::E6 => "e6",
        Position::F6 => "f6",
        Position::G6 => "g6",
        Position::H6 => "h6",
        Position::A5 => "a5",
        Position::B5 => "b5",
        Position::C5 => "c5",
        Position::D5 => "d5",
        Position::E5 => "e5",
        Position::F5 => "f5",
        Position::G5 => "g5",
        Position::H5 => "h5",
        Position::A4 => "a4",
        Position::B4 => "b4",
        Position::C4 => "c4",
        Position::D4 => "d4",
        Position::E4 => "e4",
        Position::F4 => "f4",
        Position::G4 => "g4",
        Position::H4 => "h4",
        Position::A3 => "a3",
        Position::B3 => "b3",
        Position::C3 => "c3",
        Position::D3 => "d3",
        Position::E3 => "e3",
        Position::F3 => "f3",
        Position::G3 => "g3",
        Position::H3 => "h3",
        Position::A2 => "a2",
        Position::B2 => "b2",
        Position::C2 => "c2",
        Position::D2 => "d2",
        Position::E2 => "e2",
        Position::F2 => "f2",
        Position::G2 => "g2",
        Position::H2 => "h2",
        Position::A1 => "a1",
        Position::B1 => "b1",
        Position::C1 => "c1",
        Position::D1 => "d1",
        Position::E1 => "e1",
        Position::F1 => "f1",
        Position::G1 => "g1",
        Position::H1 => "h1",
    }.to_owned()
}
