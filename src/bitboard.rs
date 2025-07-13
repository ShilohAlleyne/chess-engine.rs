use itertools::Itertools;

use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

// Easy converison between u64 and Bitboard
impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor};

// Implement the BitAnd trait for Bitboard and u64
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

// Impls the BitOr trait for Bitboard and u64
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

impl BitXor<u64> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 ^ rhs)
    }
}

// Implement the BitOrAssign trait for Bitboard
impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

// Implement the BitOrAssign trait for Bitboard and Bitboard
impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

use std::fmt;

// Print a bitboard
impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Generate board state
        let board: Vec<Vec<u64>> = (0..8)
            .map(|x| {
                (0..8)
                    .map(|y| {
                        if let Some(pos) = Position::from_u64(x * 8 + y) {
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

    pub fn pop_bit(self, position: Position) -> Self {
        let pos: u64 = position.into();
        if self.is_occupied(position) {
            self ^ 1u64 << pos
        } else {
            self
        }
    }

    // Pops a bit at a given position
    // Mutates self inplace
    pub fn mutate_pop_bit(&mut self, position: Position) {
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
    pub fn get_ls1b(&self) -> Option<Position> {
        if self.0 == 0 {
            None
        } else {
            Position::from_u32(self.0.trailing_zeros())
        }
    }

    // Set occupancies of a bitboard
    pub fn set_occupancy(&self, idx: u64, mask: &Bitboard) -> Self {
        // Check if occupancy is on board
        let check_occ = |idx: u64, count: u64| (idx & (1u64 << count)) != 0;

        // occupancy map
        let bit_count = mask.count_bits();
        let (occ, _) = (0..bit_count as u64).fold(
            (Bitboard::new(), Bitboard::from(mask.0)),
            |(occ, msk), count| {
                if let Some(lsb) = msk.get_ls1b() {
                    if check_occ(idx, count) {
                        (occ.set_bit(lsb), msk.pop_bit(lsb))
                    } else {
                        (occ, msk.pop_bit(lsb))
                    }
                } else {
                    (occ, msk)
                }
            },
        );

        occ
    }
}
