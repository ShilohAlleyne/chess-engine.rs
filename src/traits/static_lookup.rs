use crate::board::bitboard;
use crate::board::position;
use crate::board::colour;

// This is a trait that is used as marking a 
// function as STATIC lookup
pub trait StaticAttack {
    fn pawn(&self, pos: position::Position, colour: colour::Colour<()>) -> bitboard::Bitboard;
    fn knight(&self, pos: position::Position) -> bitboard::Bitboard;
    fn bishop(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard;
    fn rook(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard;
    fn queen(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        self.bishop(pos, occ) | self.rook(pos, occ)
    }
    fn king(&self, pos: position::Position) -> bitboard::Bitboard;
}

impl<T: StaticAttack> StaticAttack for &T {
    fn pawn(&self, pos: position::Position, colour: colour::Colour<()>) -> bitboard::Bitboard {
        (*self).pawn(pos, colour)
    }

    fn knight(&self, pos: position::Position) -> bitboard::Bitboard {
        (*self).knight(pos)
    }

    fn bishop(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        (*self).bishop(pos, occ)
    }

    fn rook(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        (*self).rook(pos, occ)
    }

    fn queen(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        // You can delegate or reuse default:
        (*self).queen(pos, occ)
    }

    fn king(&self, pos: position::Position) -> bitboard::Bitboard {
        (*self).king(pos)
    }
}


pub trait StaticMask {
    fn bishop(&self, pos: position::Position) -> bitboard::Bitboard;
    fn rook(&self, pos: position::Position) -> bitboard::Bitboard;
}
