use crate::traits::const_lookup;
use crate::consts;
use crate::board::position;

pub(crate) struct RelavantBitsProvider;

impl const_lookup::RelaventBits for RelavantBitsProvider {
    fn bishop(&self, pos: position::Position) -> u8 {
        consts::BISHOP_RELEVANT_BITS[pos]
    }

    fn rook(&self, pos: position::Position) -> u8 {
        consts::ROOK_RELEVANT_BITS[pos]
    }
}
