use super::bitboard::Bitboard;

// New type struct for occupancy
#[derive(Debug)]
pub struct OccupancyLayer(pub [Bitboard; 2]);

impl OccupancyLayer {
    pub fn new() -> Self {
        Self([
            // White (bottom) at index 0
            Bitboard({
                let rank_mask: u64 = 0xFF;
                rank_mask | (rank_mask << 8) // ranks 1 and 2
            }),
            // Red (top) at index 1
            Bitboard({
                let rank_mask: u64 = 0xFF;
                (rank_mask << 48) | (rank_mask << 56) // ranks 7 and 8
            }),
        ])
    }

    pub fn get_both(&self) -> Bitboard {
        self.0[0] | self.0[1]
    }
}
