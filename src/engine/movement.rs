use std::fmt::{self, Display};

use colored::Colorize;
use itertools::Itertools;

use crate::board::{pieces as PIECE, position as POSITION};

#[derive(Debug)]
pub struct Detail {
    pub piece: PIECE::Piece,
    pub source: POSITION::Position,
    pub target: POSITION::Position,
}

#[derive(Debug)]
pub enum MoveError {
    DecodeErr(String),
    EncodeErr(String)
}

impl std::error::Error for MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::DecodeErr(s) => writeln!(f, "{}", s)?,
            MoveError::EncodeErr(s) => writeln!(f, "{}", s)?,
        }

        Ok(())
    }
}


// A tightly packed u32 repesentation of a move
// | Bit Index     | Field Name    | Size (bits) | Description                                 |
// |---------------|---------------|-------------|---------------------------------------------|
// | 31-20         | MoveType      | 12          | Bitfield of traits (QUIET, CHECK, etc.)     |
// | 19-16         | Piece         | 4           | Moving piece ID                             |
// | 15-10         | Source        | 6           | Source square index (0-63)                  |
// | 9-4           | Target        | 6           | Target square index (0-63)                  |
// | 3-0           | CapturedPiece | 4           | ID of captured piece (or 0 if none)         |

// We can say castle and then n

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(pub u32);

// Iterator for lazy moves
impl IntoIterator for Move {
    type Item = Move;
    type IntoIter = std::iter::Once<Move>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

// === Display ===
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let traits_str = traits(*self)
            .map(|t| format!("{}", t)) // uses MoveTrait's Display
            .join(", ");

        let cap = match capture(*self) {
            Some(c) => format!("{}", c),
            None => "None".to_owned(),
        };

        writeln!(
            f,
            "Move: {:08X} | Traits: [{}] | Piece: {} | Movement: {} -> {} | Captures: {}",
            self.0,
            traits_str,
            piece(*self).expect("Piece to move."),
            source(*self),
            target(*self),
            cap,
        )
    }
}

impl Move {
    pub fn with_traits(&mut self, move_traits: &[MoveTrait]) {
        let flags: u16 = move_traits
            .iter()
            .map(|t| t.bit())
            .fold(0, |acc, b| acc | b);
        // Clear current trait bits first
        self.0 &= !(0xFFF << 20);
        // Set new traits
        self.0 |= (flags as u32) << 20;
    }
}

// === Action Data access funcs ===
pub fn traits(action: Move) -> impl Iterator<Item = MoveTrait> {
    MoveTrait::ALL.iter().flat_map(move |(trait_type, mask)| {
        if (((action.0 >> 20) & 0xFFF) as u16) & mask != 0 {
            return Some(*trait_type);
        }
        None
    })
}

pub fn piece(action: Move) -> Option<PIECE::Piece> {
    PIECE::try_from_u8(((action.0 >> 16) & 0xF) as u8)
        .expect("Move has invalid piece configuration.")
}

pub fn source(action: Move) -> POSITION::Position {
    POSITION::Position::try_from(((action.0 >> 10) & 0x3F) as u8)
        .expect("Move has invalid source position configuration.")
}

pub fn target(action: Move) -> POSITION::Position {
    POSITION::Position::try_from(((action.0 >> 4) & 0x3F) as u8)
        .expect("Move has invalid target positon configuration.")
}

pub fn capture(action: Move) -> Option<PIECE::Piece> {
    PIECE::try_from_u8((action.0 & 0xF) as u8)
        .expect("Move has invalid capture piece configuration.")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveTrait {
    Quiet,
    Capture,
    Check,
    Promotion,
    Enpassant,
    Castle,
}

// === Display ===
impl fmt::Display for MoveTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MoveTrait::*;
        let styled = match self {
            Quiet     => "Quiet".white(),
            Capture   => "Capture".red(),
            Check     => "Check".yellow(),
            Promotion => "Promotion".magenta(),
            Enpassant => "Enpassant".cyan(),
            Castle    => "Castle".blue(),
        };
        write!(f, "{}", styled)?;

        Ok(())
    }
}

impl MoveTrait {
    const ALL: [(MoveTrait, u16); 6] = [
        (MoveTrait::Quiet, 1 << 0),
        (MoveTrait::Capture, 1 << 1),
        (MoveTrait::Check, 1 << 2),
        (MoveTrait::Promotion, 1 << 3),
        (MoveTrait::Enpassant, 1 << 4),
        (MoveTrait::Castle, 1 << 5),
    ];

    pub fn bit(self) -> u16 {
        Self::ALL
            .iter()
            .find_map(
                |(kind, mask)| {
                    if *kind == self {
                        Some(*mask)
                    } else {
                        None
                    }
                },
            )
            .unwrap_or(0) // or panic if trait is unknown
    }
}

// === Builder ===
#[derive(Clone, Copy)]
pub struct MoveBuilder(pub u32);

impl MoveBuilder {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set_traits(self, move_traits: &[MoveTrait]) -> Self {
        let mut result = self;

        let flags: u16 = move_traits
            .iter()
            .map(|t| t.bit())
            .fold(0, |acc, b| acc | b);

        result.0 &= !(0xFFF << 20); // Clear current trait bits
        result.0 |= (flags as u32) << 20; // Set new traits

        result
    }

    pub fn set_piece(self, piece: PIECE::Piece) -> Self {
        let mut result = self;

        let piece_bit: u8 = piece.into();
        result.0 |= (piece_bit as u32) << 16;

        result
    }

    pub fn set_source(self, src: POSITION::Position) -> Self {
        let mut result = self;

        let pos_bits: u8 = src.into();
        result.0 |= (pos_bits as u32) << 10;

        result
    }

    pub fn set_target(self, trgt: POSITION::Position) -> Self {
        let mut result = self;

        let trgt_bits: u8 = trgt.into();
        result.0 |= (trgt_bits as u32) << 4;

        result
    }

    pub fn captures(self, captures: PIECE::Piece) -> Self {
        let mut result = self;

        let cap: u8 = captures.into();
        result.0 |= cap as u32;

        result
    }

    pub fn build(self) -> Move {
        Move(self.0)
    }
}

impl Default for MoveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
