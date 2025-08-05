use crate::board::{castling, pieces, position};

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
    // The promoted piece was at target square
    position::Position::try_from(((delta.0 >> 10) & 0xF) as u8)
        .expect("Invalid Position encoding in State Delta")
}

// We only need to keep track of where the promotion happend?
pub fn target(delta: Delta) -> position::Position {
    // The promoted piece was at target square
    position::Position::try_from(((delta.0 >> 6) & 0xF) as u8)
        .expect("Invalid Position encoding in State Delta")
}

pub fn captured(delta: Delta) -> Option<pieces::Piece> {
    pieces::try_from_u8((delta.0 & 0xF) as u8)
        .expect("Invalid Piece encoding in State Delta")
}
