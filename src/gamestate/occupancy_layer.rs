use std::ops::Index;

use crate::board::bitboard;
use crate::board::colour;
use crate::board::pieces;

use super::material_layer;

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

pub fn generate_occ(mat_layer: material_layer::MaterialLayer) -> OccupancyLayer {
    // White occupancy: combine all white piece bitboards
    OccupancyLayer([
        mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::Rook)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::Knight)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::Bishop)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::Queen)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::King)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::White(()), pieces::Kind::Pawn)],
        // Black occupancy: combine all black piece bitboards
        mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::Rook)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::Knight)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::Bishop)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::Queen)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::King)]
            | mat_layer[pieces::from_colour_kind(&colour::Colour::Black(()), pieces::Kind::Pawn)],
    ])
}
