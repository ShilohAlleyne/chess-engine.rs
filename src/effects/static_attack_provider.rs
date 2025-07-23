use crate::board::bitboard as BITBOARD;
use crate::board::colour as COLOUR;
use crate::board::position as POSITION;
use crate::consts as CONST;
use crate::engine::lazy_statics as STATIC;
use crate::traits::static_lookup as STATIC_ATTK_LOOKUP;

#[derive(Debug, Clone, Copy, Default)]
pub struct StaticAttackProvider;

impl STATIC_ATTK_LOOKUP::StaticAttack for StaticAttackProvider {
    fn pawn(&self, pos: POSITION::Position, colour: COLOUR::Colour<()>) -> BITBOARD::Bitboard {
        STATIC::PAWN_ATTACKS[colour][pos]
    }

    fn knight(&self, pos: POSITION::Position) -> BITBOARD::Bitboard {
        STATIC::KNIGHT_ATTACKS[pos]
    }

    fn bishop(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        let masked_occ = occ & STATIC::BISHOP_MASKS[pos];
        let index = (masked_occ.0.wrapping_mul(CONST::BISHOP_MAGIC_NUMBERS[pos])
            >> (64 - CONST::BISHOP_RELEVANT_BITS[pos])) as usize;

        STATIC::BISHOP_ATTACKS[pos][index]
    }

    fn rook(&self, pos: POSITION::Position, occ: BITBOARD::Bitboard) -> BITBOARD::Bitboard {
        let masked_occ = occ & STATIC::ROOK_MASKS[pos];
        let index = (masked_occ.0.wrapping_mul(CONST::ROOK_MAGIC_NUMBERS[pos])
            >> (64 - CONST::ROOK_RELEVANT_BITS[pos])) as usize;

        STATIC::ROOK_ATTACKS[pos][index]
    }
}
