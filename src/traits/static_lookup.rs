use crate::board::bitboard as BITBOARD;
use crate::board::position as POSITION;
use crate::board::colour as COLOUR;

// This is a trait that is used as marking a 
// function as STATIC lookup
pub trait StaticAttack {
    fn pawn(&self, pos: POSITION::Position, colour: COLOUR::Colour<()>) -> BITBOARD::Bitboard;
    fn knight(&self, pos: POSITION::Position) -> BITBOARD::Bitboard;
    fn bishop(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard;
    fn rook(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard;

    fn queen(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        self.bishop(pos, occ) | self.rook(pos, occ)
    }
}

impl<T: StaticAttack> StaticAttack for &T {
    fn pawn(&self, pos: POSITION::Position, colour: COLOUR::Colour<()>) -> BITBOARD::Bitboard {
        (*self).pawn(pos, colour)
    }

    fn knight(&self, pos: POSITION::Position) -> BITBOARD::Bitboard {
        (*self).knight(pos)
    }

    fn bishop(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        (*self).bishop(pos, occ)
    }

    fn rook(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        (*self).rook(pos, occ)
    }

    fn queen(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        // You can delegate or reuse default:
        (*self).queen(pos, occ)
    }
}


pub trait StaticMask {
    fn bishop(&self, pos: POSITION::Position) -> BITBOARD::Bitboard;
    fn rook(&self, pos: POSITION::Position) -> BITBOARD::Bitboard;
}
