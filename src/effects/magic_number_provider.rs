use crate::board::position as POSITION;
use crate::consts as CONSTS;
use crate::traits::const_lookup as CONST_LOOKUP;

#[derive(Debug, Clone, Copy)]
pub(crate) struct MagicNumberProvider;

impl CONST_LOOKUP::ConstMagicNumber for MagicNumberProvider {
    fn bishop(&self, pos: POSITION::Position) -> u64 {
        CONSTS::BISHOP_MAGIC_NUMBERS[pos]
    }

    fn rook(&self, pos: POSITION::Position) -> u64 {
        CONSTS::ROOK_MAGIC_NUMBERS[pos]
    }
}
