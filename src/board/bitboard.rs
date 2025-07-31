use crate::board::position as POSITION;
use itertools::Itertools;
use std::fmt;
use std::num::Wrapping;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Not, ShrAssign
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

// Easy converison between u64 and Bitboard
impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

// === Bitwise trait impls ===
impl BitAnd<u64> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 & rhs)
    }
}

impl BitAnd<Bitboard> for u64 {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self & rhs.0)
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 | rhs)
    }
}

impl BitOr<Bitboard> for u64 {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self | rhs.0)
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 ^ rhs)
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

impl BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

// === Multiplicative trait impls ===
impl Mul<u64> for Bitboard {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 * rhs)
    }
}

impl Mul<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn mul(self, rhs: Bitboard) -> Bitboard {
        Bitboard((Wrapping(self.0) * Wrapping(rhs.0)).0)
    }
}

impl MulAssign<u64> for Bitboard {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 = (Wrapping(self.0) * Wrapping(rhs)).0;
    }
}

impl MulAssign<Bitboard> for Bitboard {
    fn mul_assign(&mut self, rhs: Bitboard) {
        self.0 = (Wrapping(self.0) * Wrapping(rhs.0)).0;
    }
}

impl ShrAssign<u64> for Bitboard {
    fn shr_assign(&mut self, rhs: u64) {
        self.0 >>= rhs;
    }
}

impl ShrAssign<Bitboard> for Bitboard {
    fn shr_assign(&mut self, rhs: Bitboard) {
        self.0 >>= rhs.0;
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard(!self.0)
    }
}

// === LSB iteration ===
impl Iterator for Bitboard {
    type Item = POSITION::Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let lsb = self.get_ls1b()?;
            self.mutate_pop_bit(lsb);
            Some(lsb)
        }
    }
}

// === Display trait ===
impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Generate board state
        let board: Vec<Vec<u64>> = (0..8)
            .map(|x| {
                (0..8)
                    .map(|y| {
                        if let Some(pos) = POSITION::Position::from_u64(x * 8 + y) {
                            if self.is_occupied(pos) {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();

        // Print Bitboard value
        writeln!(f, "\nBitboard: {}d\n", self.0)?;

        // Print board with row labels
        for (i, rank) in board.iter().enumerate() {
            // Row numbers
            write!(f, "{} | ", 8 - i)?;
            for &tile in rank {
                write!(f, "{} ", tile)?;
            }
            writeln!(f)?;
        }

        // Column labels
        writeln!(f, "  -----------------")?;
        writeln!(f, "    a b c d e f g h\n")
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    // Sets a bit at a given position
    // Consumes and returns self
    pub fn set_bit(self, position: impl Into<u64>) -> Self {
        let pos: u64 = position.into();
        self | 1u64 << pos
    }

    // Sets a bit at a position
    // Mutates self inplace
    pub fn mutate_set_bit(&mut self, position: impl Into<u64>) {
        let pos: u64 = position.into();
        self.0 |= 1u64 << pos
    }

    pub fn pop_bit(self, position: POSITION::Position) -> Self {
        let pos: u64 = position.into();
        if self.is_occupied(position) {
            self ^ 1u64 << pos
        } else {
            self
        }
    }

    // Pops a bit at a given position
    // Mutates self inplace
    pub fn mutate_pop_bit(&mut self, position: POSITION::Position) {
        let pos = position as u64;
        if self.is_occupied(pos) {
            self.0 ^= 1u64 << pos;
        }
    }

    // Is the possition occupied
    pub fn is_occupied(&self, position: impl Into<u64>) -> bool {
        let pos = position.into();
        self.0 & (1u64 << pos) != 0
    }

    // How many bits are occupied in a bitboard
    pub fn count_bits(&self) -> usize {
        let mut bb = Bitboard::from(self.0);

        (0..64)
            .take_while_inclusive(|_| {
                bb.0 &= bb.0 - 1;
                bb.0 > 0
            })
            .count()
    }

    // Get the index of the least significant bitboard
    pub fn get_ls1b(&self) -> Option<POSITION::Position> {
        if self.0 == 0 {
            None
        } else {
            POSITION::Position::from_u32(self.0.trailing_zeros())
        }
    }

    // Set occupancies of a bitboard
    pub fn set_occupancy(&mut self, index: u64, mask: &Bitboard) {
        // Collect all bit positions in mask in a consistent order
        let mut positions: Vec<u64> = Vec::new();
        for i in 0..64 {
            if (mask.0 >> i) & 1 != 0 {
                positions.push(i);
            }
        }

        for (i, &pos) in positions.iter().enumerate() {
            if (index >> i) & 1 != 0 {
                self.mutate_set_bit(pos);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
