use core::fmt;

use itertools::Itertools;

use crate::{board::{castling, pieces, position}, engine::movement};

// | Bit Index | Field Name    | Size (bits) | Description                             |
// |-----------|---------------|-------------|-----------------------------------------|
// | 31–26     | `extra`       | 6           | Reserved for overlays, flags, extensions|
// | 25        | `en_passant`  | 1           | En passant flag (`0 = no`, `1 = valid`) |
// | 24        | `promotion`   | 1           | Promotion flag (`0 = no`, `1 = yes`)    |
// | 23–20     | `castling`    | 4           | Castling info (side, rook pos, etc.)    |
// | 19–16     | `moved_piece` | 4           | Piece type that was moved               |
// | 15–10     | `source`      | 6           | Destination square index                |
// | 9–4       | `target`      | 6           | Source square index                     |
// | 3–0       | `captured`    | 4           | Captured piece ID (or `0` if none)      |

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Delta(pub u32);

// Iterator for lazy delta chains
impl IntoIterator for Delta {
    type Item = Delta;
    type IntoIter = std::iter::Once<Delta>;
    
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

// === Display ===
impl fmt::Display for Delta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let castling_str = castling(*self)
            .map(|cr| format!("{}", cr))
            .join(", ");

        let cap = match captured(*self) {
            Some(c) => format!("{}", c),
            None => "None".to_owned(),
        };

        writeln!(
            f,
            "Delta: {:08X} | Castling rights lost: [{}] | Piece: {} | Movement: {} -> {} | Captures: {}",
            self.0,
            castling_str,
            moved_piece(*self).expect("Pieces always move"),
            source(*self),
            target(*self),
            cap
        )
    }
}





//=== Delta Data access funcs ===
pub fn is_en_passant(delta: Delta) -> bool {
    delta.0 >> 25 != 0
}

pub fn is_promotion(delta: Delta) -> bool {
    delta.0 >> 24 != 0
}

pub fn castling(delta: Delta) -> impl Iterator<Item = castling::Castling> {
    castling::castling_rights_from_bits(castling::CastlingRights(((delta.0 >> 20) & 0xF) as u8))
}

pub fn moved_piece(delta: Delta) -> Option<pieces::Piece> {
    pieces::try_from_u8(((delta.0 >> 16) & 0xF) as u8)
        .expect("Invalid Piece encoding in State Delta")
}

pub fn source(delta: Delta) -> position::Position {
    position::Position::try_from(((delta.0 >> 10) & 0x3F) as u8)
        .expect("Invalid Position encoding in State Delta")
}

pub fn target(delta: Delta) -> position::Position {
    position::Position::try_from(((delta.0 >> 4) & 0x3F) as u8)
        .expect("Invalid Position encoding in State Delta")
}

pub fn captured(delta: Delta) -> Option<pieces::Piece> {
    pieces::try_from_u8((delta.0 & 0xF) as u8)
        .expect("Invalid Piece encoding in State Delta")
}

// === Invert Delta ===
// This function takes delta D, and produces a new delta d
// which whem appiled, has the opposite eff of D
#[derive(Clone, Copy)]
pub struct DeltaBuilder(pub u32);

impl DeltaBuilder {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn en_passant(self, enpassant: bool) -> Self {
        let mut res = self;

        res.0 &= (enpassant as u32) << 25;

        res
    }

    pub fn promotion(self, promotion: bool) -> Self {
        let mut res = self;

        res.0 &= (promotion as u32) << 24;

        res
    }

    pub fn castling(self, castling: castling::CastlingRights) -> Self {
        let mut res = self;

        let castling_bits = castling.0 & 0xF;

        res.0 &= (castling_bits as u32) << 20;

        res
    }

    // === Piece Movement ===
    pub fn set_movement_from_move(self, mv: movement::Move) -> Self {
        // Extract raw values
        let piece    = (mv.0 >> 16) & 0xF;
        let source   = (mv.0 >> 10) & 0x3F;
        let target   = (mv.0 >> 4)  & 0x3F;
        let captured = mv.0 & 0xF;

        // Reposition into Delta layout
        let piece_bits    = piece << 16;
        let source_bits   = source << 10;
        let target_bits   = target << 4;
        let captured_bits = captured;

        let movement_bits = piece_bits | source_bits | target_bits | captured_bits;

        // Clear and insert
        const MOVEMENT_MASK: u32 = 0x000F_FFFF; // bits 19–0
        Self((self.0 & !MOVEMENT_MASK) | movement_bits)
    }

    pub fn set_piece(self, piece: pieces::Piece) -> Self {
        let mut result = self;

        let piece_bit: u8 = piece.into();
        result.0 |= (piece_bit as u32) << 16;

        result
    }

    pub fn set_source(self, src: position::Position) -> Self {
        let mut result = self;

        let pos_bits: u8 = src as u8;
        result.0 |= (pos_bits as u32) << 10;

        result
    }

    pub fn set_target(self, trgt: position::Position) -> Self {
        let mut result = self;

        let trgt_bits: u8 = trgt as u8;
        result.0 |= (trgt_bits as u32) << 4;

        result
    }

    pub fn captures(self, captures: pieces::Piece) -> Self {
        let mut result = self;

        let cap: u8 = captures.into();
        result.0 |= cap as u32;

        result
    }

    pub fn build(self) -> Delta {
        Delta(self.0)
    }

}

impl Default for DeltaBuilder {
    fn default() -> Self {
        Self::new()
    }
}
