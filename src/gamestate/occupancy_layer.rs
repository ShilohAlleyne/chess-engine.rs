use std::ops::Index;

use crate::board::bitboard;
use crate::board::colour;

// New type struct for occupancy
#[derive(Debug, Clone, Copy)]
pub struct OccupancyLayer(pub [bitboard::Bitboard; 2]);

impl OccupancyLayer {
    pub fn new() -> Self {
        Self([
            // White (bottom) at index 0
            bitboard::Bitboard({
                let rank_mask: u64 = 0xFF;
                rank_mask | (rank_mask << 8) // ranks 1 and 2
            }),
            // Red (top) at index 1
            bitboard::Bitboard({
                let rank_mask: u64 = 0xFF;
                (rank_mask << 48) | (rank_mask << 56) // ranks 7 and 8
            }),
        ])
    }
}

impl Default for OccupancyLayer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_both(occ: &OccupancyLayer) -> bitboard::Bitboard {
    occ.0[0] | occ.0[1]
}

impl Index<colour::Colour<()>> for OccupancyLayer {
    type Output = bitboard::Bitboard;

    fn index(&self, index: colour::Colour<()>) -> &Self::Output {
        match index {
            colour::Colour::White(()) => &self.0[0],
            colour::Colour::Black(()) => &self.0[1],
        }
    }
}
