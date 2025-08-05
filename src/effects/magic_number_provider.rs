use crate::board::position;
use crate::consts;
use crate::traits::const_lookup;

#[derive(Debug, Clone, Copy)]
pub(crate) struct MagicNumberProvider;

impl const_lookup::ConstMagicNumber for MagicNumberProvider {
    fn bishop(&self, pos: position::Position) -> u64 {
        consts::BISHOP_MAGIC_NUMBERS[pos]
    }

    fn rook(&self, pos: position::Position) -> u64 {
        consts::ROOK_MAGIC_NUMBERS[pos]
    }
}
