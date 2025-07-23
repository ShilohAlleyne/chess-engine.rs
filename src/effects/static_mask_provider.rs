use crate::traits::static_lookup as STATIC_LOOKUP;
use crate::board::position as POSITION;
use crate::board::bitboard as BITBOARD;
use crate::engine::lazy_statics as STATIC;

pub(crate) struct StaticMaskProvider;

impl STATIC_LOOKUP::StaticMask for StaticMaskProvider {
    fn bishop(&self, pos: POSITION::Position) -> BITBOARD::Bitboard {
        STATIC::BISHOP_MASKS[pos]
    }

    fn rook(&self, pos: POSITION::Position) -> BITBOARD::Bitboard {
        STATIC::ROOK_MASKS[pos]
    }
}
