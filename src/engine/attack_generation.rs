use crate::{
    board::{bitboard as BITBOARD, colour as COLOUR, position as POSITION},
    traits::{const_lookup as CONST_LOOKUP, static_lookup as PRECOMP},
};
use strum::IntoEnumIterator;

// === Attack generation ===
// A generic function for generating attack tables
pub(crate) fn gen_attacks<C, F>(
    f: F,
    side: COLOUR::Colour<()>,
    lookup: C,
) -> [BITBOARD::Bitboard; 64]
where
    C: CONST_LOOKUP::ConstFileMask + Copy,
    F: for<'x> Fn(POSITION::Position, &'x COLOUR::Colour<()>, C) -> BITBOARD::Bitboard,
{
    POSITION::Position::iter()
        .map(|p| f(p, &side, lookup))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Error generating attack table")
}

pub(crate) fn fly_gen_bishop_attks(
    position: impl Into<u64>,
    block: &BITBOARD::Bitboard,
) -> BITBOARD::Bitboard {
    let pos = position.into();
    let t_rank = pos / 8;
    let t_file = pos % 8;
    let mut attacks = 0u64;

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]; // NE, NW, SE, SW

    for (dr, df) in directions {
        let mut r = t_rank as i64 + dr;
        let mut f = t_file as i64 + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = POSITION::Position::from_u64((r * 8 + f) as u64).unwrap();
            attacks |= 1u64 << sq as u64;
            if block.is_occupied(sq) {
                break; // Blocker found — stop here
            }
            r += dr;
            f += df;
        }
    }

    BITBOARD::Bitboard::from(attacks)
}

// Generate Rook Attacks on the fly
pub(crate) fn fly_gen_rook_attks(
    position: POSITION::Position,
    block: &BITBOARD::Bitboard,
) -> BITBOARD::Bitboard {
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
            let sq = POSITION::Position::from_u64((r * 8 + f) as u64).unwrap();
            attacks |= 1u64 << sq as u64;
            if block.is_occupied(sq) {
                break; // Truncate at blocker
            }
            r += dr;
            f += df;
        }
    }

    BITBOARD::Bitboard::from(attacks)
}

pub(crate) fn gen_bishop_attacks<A, B, C>(
    static_lookup: A,
    relavent_bit_lookup: B,
    magic_number_lookup: C,
) -> [[BITBOARD::Bitboard; 512]; 64]
where
    A: PRECOMP::StaticMask,
    B: CONST_LOOKUP::RelaventBits,
    C: CONST_LOOKUP::ConstMagicNumber,
{
    let mut slider: [[BITBOARD::Bitboard; 512]; 64] = [[BITBOARD::Bitboard::default(); 512]; 64];

    for p in POSITION::Position::iter() {
        let attk_mask = static_lookup.bishop(p);
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let mut occupancy = BITBOARD::Bitboard::new();
            occupancy.set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy
                .0
                .wrapping_mul(magic_number_lookup.bishop(p)))
                >> (64 - relavent_bit_lookup.bishop(p));

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

pub(crate) fn gen_rook_attacks<A, B, C>(
    static_lookup: A,
    relavent_bit_lookup: B,
    magic_number_lookup: C,
) -> [[BITBOARD::Bitboard; 4096]; 64]
where
    A: PRECOMP::StaticMask,
    B: CONST_LOOKUP::RelaventBits,
    C: CONST_LOOKUP::ConstMagicNumber,
{
    let mut slider: [[BITBOARD::Bitboard; 4096]; 64] = [[BITBOARD::Bitboard::default(); 4096]; 64];

    for p in POSITION::Position::iter() {
        let attk_mask = static_lookup.rook(p);
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let mut occupancy = BITBOARD::Bitboard::new();
            occupancy.set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy.0.wrapping_mul(magic_number_lookup.rook(p)))
                >> (64 - relavent_bit_lookup.rook(p));

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
