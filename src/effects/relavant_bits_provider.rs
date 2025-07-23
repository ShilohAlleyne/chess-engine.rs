use crate::traits::const_lookup as CONST_LOOKUP;
use crate::consts as CONSTS;
use crate::board::position as POSITION;

pub(crate) struct RelavantBitsProvider;

impl CONST_LOOKUP::RelaventBits for RelavantBitsProvider {
    fn bishop(&self, pos: POSITION::Position) -> u8 {
        CONSTS::BISHOP_RELEVANT_BITS[pos]
    }

    fn rook(&self, pos: POSITION::Position) -> u8 {
        CONSTS::ROOK_RELEVANT_BITS[pos]
    }
}
