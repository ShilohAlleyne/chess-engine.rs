use crate::{
    board::{bitboard::Bitboard, pieces::Colour, position::Position},
    consts::{
        self as CONSTS, BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS, ROOK_MAGIC_NUMBERS,
        ROOK_RELEVANT_BITS,
    },
};
use strum::IntoEnumIterator;

use super::lazy::{BISHOP_ATTACKS, BISHOP_MASKS, KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, ROOK_ATTACKS, ROOK_MASKS};

// This struct holds all our attack tables
pub struct AttackTables {
    pub(crate) pawn_attacks: [[Bitboard; 64]; 2],
    pub(crate) knight_attacks: [Bitboard; 64],
    pub(crate) king_attacks: [Bitboard; 64],
    pub(crate) bishop_attacks: Box<[[Bitboard; 512]; 64]>,
    pub(crate) rook_attacks: Box<[[Bitboard; 4096]; 64]>,
}


impl AttackTables {
    pub fn new() -> Self {
        Self {
            pawn_attacks: *PAWN_ATTACKS,
            knight_attacks: *KNIGHT_ATTACKS,
            king_attacks: *KING_ATTACKS,
            bishop_attacks: BISHOP_ATTACKS.clone(),
            rook_attacks: ROOK_ATTACKS.clone(),
        }
    }

    // === Get Attacks ===
    // Will upgrate these with oncecell lazy eval at somepoint
    pub fn get_bishop_attacks(&self, position: Position, occ: Bitboard) -> Bitboard {
        let masked_occ = occ & BISHOP_MASKS[position];
        let index = (masked_occ
            .0
            .wrapping_mul(BISHOP_MAGIC_NUMBERS[position])
            >> (64 - BISHOP_RELEVANT_BITS[position])) as usize;

        self.bishop_attacks[position][index]
    }

    pub fn get_rook_attacks(&self, position: Position, occ: Bitboard) -> Bitboard {
        let masked_occ = occ & ROOK_MASKS[position];
        let index = (masked_occ
            .0
            .wrapping_mul(ROOK_MAGIC_NUMBERS[position])
            >> (64 - ROOK_RELEVANT_BITS[position])) as usize;

        self.rook_attacks[position][index]
    }

    pub fn get_queen_attacks(&self, position: Position, occ: Bitboard) -> Bitboard {
        self.get_bishop_attacks(position, occ) | self.get_rook_attacks(position, occ)
    }
}

impl Default for AttackTables {
    fn default() -> Self {
        Self::new()
    }
}

// === Attack generation ===

// A generic function for generating attack tables
pub(crate) fn gen_attacks<F>(f: F, side: Colour<()>) -> [Bitboard; 64]
where
    F: Fn(Position, &Colour<()>) -> Bitboard,
{
    Position::iter()
        .map(|p| f(p, &side))
        .collect::<Vec<Bitboard>>()
        .try_into()
        .expect("Error generating attack table")
}     

pub(crate) fn fly_gen_bishop_attks(position: impl Into<u64>, block: &Bitboard) -> Bitboard {
    let pos = position.into();
    let t_rank = pos / 8;
    let t_file = pos % 8;
    let mut attacks = 0u64;

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]; // NE, NW, SE, SW

    for (dr, df) in directions {
        let mut r = t_rank as i64 + dr;
        let mut f = t_file as i64 + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = Position::from_u64((r * 8 + f) as u64).unwrap();
            attacks |= 1u64 << sq as u64;
            if block.is_occupied(sq) {
                break; // Blocker found — stop here
            }
            r += dr;
            f += df;
        }
    }

    Bitboard::from(attacks)
}

// Generate Rook Attacks on the fly
pub(crate) fn fly_gen_rook_attks(position: Position, block: &Bitboard) -> Bitboard {
    let pos = position as u64;
    let t_rank = pos / 8;
    let t_file = pos % 8;
    let mut attacks = 0u64;

    let directions = [
        (1, 0),  // North
        (-1, 0), // South
        (0, 1),  // East
        (0, -1), // West
    ];

    for (dr, df) in directions {
        let mut r = t_rank as i64 + dr;
        let mut f = t_file as i64 + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = Position::from_u64((r * 8 + f) as u64).unwrap();
            attacks |= 1u64 << sq as u64;
            if block.is_occupied(sq) {
                break; // Truncate at blocker
            }
            r += dr;
            f += df;
        }
    }

    Bitboard::from(attacks)
}

pub(crate) fn gen_bishop_attacks() -> [[Bitboard; 512]; 64] {
    let mut slider: [[Bitboard; 512]; 64] = [[Bitboard::default(); 512]; 64];

    for p in Position::iter() {
        let attk_mask = BISHOP_MASKS[p];
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let mut occupancy = Bitboard::new();
            occupancy.set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy.0.wrapping_mul(BISHOP_MAGIC_NUMBERS[p]))
                >> (64 - CONSTS::BISHOP_RELEVANT_BITS[p]);

            debug_assert!(
                magic_idx < 512,
                "Magic index out of bounds at square {} → {}",
                p,
                magic_idx
            );

            slider[p][magic_idx as usize] = fly_gen_bishop_attks(p, &occupancy);
        }
    }

    slider
}

pub(crate) fn gen_rook_attacks() -> [[Bitboard; 4096]; 64] {
    let mut slider: [[Bitboard; 4096]; 64] = [[Bitboard::default(); 4096]; 64];

    for p in Position::iter() {
        let attk_mask = ROOK_MASKS[p];
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let mut occupancy = Bitboard::new();
            occupancy.set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy
                .0
                .wrapping_mul(CONSTS::ROOK_MAGIC_NUMBERS[p]))
                >> (64 - CONSTS::ROOK_RELEVANT_BITS[p]);

            // Sanity guard to avoid out-of-bounds indexing
            debug_assert!(
                magic_idx < 4096,
                "Magic index out of bounds: square {} → {}",
                p,
                magic_idx
            );

            slider[p][magic_idx as usize] = fly_gen_rook_attks(p, &occupancy);
        }
    }

    slider
}
