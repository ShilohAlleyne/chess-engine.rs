use crate::board::bitboard as BITBOARD;

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

pub fn get_both(occ: &OccupancyLayer) -> BITBOARD::Bitboard {
    occ.0[0] | occ.0[1]
}

