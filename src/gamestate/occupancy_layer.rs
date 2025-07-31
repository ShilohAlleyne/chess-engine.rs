use std::ops::Index;

use crate::board::bitboard as BITBOARD;
use crate::board::colour as COLOUR;

// New type struct for occupancy
#[derive(Debug)]
pub struct OccupancyLayer(pub [BITBOARD::Bitboard; 2]);

impl OccupancyLayer {
    pub fn new() -> Self {
        Self([
            // White (bottom) at index 0
            BITBOARD::Bitboard({
                let rank_mask: u64 = 0xFF;
                rank_mask | (rank_mask << 8) // ranks 1 and 2
            }),
            // Red (top) at index 1
            BITBOARD::Bitboard({
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

pub fn get_both(occ: &OccupancyLayer) -> BITBOARD::Bitboard {
    occ.0[0] | occ.0[1]
}

impl Index<COLOUR::Colour<()>> for OccupancyLayer {
    type Output = BITBOARD::Bitboard;

    fn index(&self, index: COLOUR::Colour<()>) -> &Self::Output {
        match index {
            COLOUR::Colour::White(()) => &self.0[0],
            COLOUR::Colour::Red(()) => &self.0[1],
        }
    }
}
