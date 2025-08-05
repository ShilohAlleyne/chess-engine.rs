use crate::board::bitboard;
use crate::board::colour;
use crate::board::position;
use crate::consts;
use crate::engine::lazy_statics;
use crate::traits::static_lookup;

#[derive(Debug, Clone, Copy, Default)]
pub struct StaticAttackProvider;

impl static_lookup::StaticAttack for StaticAttackProvider {
    fn pawn(&self, pos: position::Position, colour: colour::Colour<()>) -> bitboard::Bitboard {
        lazy_statics::PAWN_ATTACKS[colour][pos]
    }

    fn knight(&self, pos: position::Position) -> bitboard::Bitboard {
        lazy_statics::KNIGHT_ATTACKS[pos]
    }

    fn bishop(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        let masked_occ = occ & lazy_statics::BISHOP_MASKS[pos];
        let index = (masked_occ.0.wrapping_mul(consts::BISHOP_MAGIC_NUMBERS[pos])
            >> (64 - consts::BISHOP_RELEVANT_BITS[pos])) as usize;

        lazy_statics::BISHOP_ATTACKS[pos][index]
    }

    fn rook(&self, pos: position::Position, occ: bitboard::Bitboard) -> bitboard::Bitboard {
        let masked_occ = occ & lazy_statics::ROOK_MASKS[pos];
        let index = (masked_occ.0.wrapping_mul(consts::ROOK_MAGIC_NUMBERS[pos])
            >> (64 - consts::ROOK_RELEVANT_BITS[pos])) as usize;

        lazy_statics::ROOK_ATTACKS[pos][index]
    }

    fn king(&self, pos: position::Position) -> bitboard::Bitboard {
        lazy_statics::KING_ATTACKS[pos]
    }
}
