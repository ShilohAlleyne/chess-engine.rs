use crate::board::position as POSITION;

// === traits for signalling const lookup quasi-side effects ===
pub trait ConstFileMask {
    fn not_a_file(&self) -> u64;
    fn not_h_file(&self) -> u64;
    fn not_ab_file(&self) -> u64;
    fn not_hg_file(&self) -> u64;
}

pub trait ConstMagicNumber {
    fn bishop(&self, pos: POSITION::Position) -> u64;
    fn rook(&self, pos: POSITION::Position) -> u64;
}

pub trait RelaventBits {
    fn bishop(&self, pos: POSITION::Position) -> u8;
    fn rook(&self, pos: POSITION::Position) -> u8;
}
