use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::consts as CONSTS;
use crate::consts::BISHOP_MAGIC_NUMBERS;
use crate::{bitboard::Bitboard, pieces::Colour, position::Position};

// This struct holds all our attack tables
pub(crate) struct AttackTables {
    pub(crate) pawn_attacks: [[Bitboard; 64]; 2],
    pub(crate) knight_attacks: [Bitboard; 64],
    pub(crate) king_attacks: [Bitboard; 64],
    pub(crate) bishop_attacks: [[Bitboard; 512]; 64],
    pub(crate) rook_attacks: [[Bitboard; 4096]; 64],
}

impl AttackTables {
    pub(crate) fn new() -> Self {
        let pawn_attacks: [[Bitboard; 64]; 2] = [
            gen_attacks(mask_prawn_attacks, Colour::White(())),
            gen_attacks(mask_prawn_attacks, Colour::Red(())),
        ];
        let knight_attacks: [Bitboard; 64] = gen_attacks(mask_knight_attacks, Colour::White(()));
        let king_attacks: [Bitboard; 64] = gen_attacks(mask_king_attacks, Colour::White(()));
        let bishop_attacks: [[Bitboard; 512]; 64] = gen_bishop_attacks();
        let rook_attacks: [[Bitboard; 4096]; 64] = gen_rook_attacks();

        Self {
            pawn_attacks,
            knight_attacks,
            king_attacks,
            bishop_attacks,
            rook_attacks,
        }
    }
}

// A generic function for generating attack tables
fn gen_attacks<F>(f: F, side: Colour<()>) -> [Bitboard; 64]
where
    F: Fn(Position, &Colour<()>) -> Bitboard,
{
    Position::iter()
        .map(|p| f(p, &side))
        .collect::<Vec<Bitboard>>()
        .try_into()
        .expect("Error generating attack table")
}

// === Attack Masks ===
fn mask_prawn_attacks(position: Position, side: &Colour<()>) -> Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = Bitboard::new().set_bit(position);

    match side {
        Colour::White(()) => {
            if (bitboard.0 >> 7) & CONSTS::NOT_A_FILE != 0 {
                attacks |= bitboard.0 >> 7;
            }
            if (bitboard.0 >> 9) & CONSTS::NOT_H_FILE != 0 {
                attacks |= bitboard.0 >> 9;
            }
        }
        Colour::Red(()) => {
            if (bitboard.0 << 7) & CONSTS::NOT_H_FILE != 0 {
                attacks |= bitboard.0 << 7;
            }
            if (bitboard.0 << 9) & CONSTS::NOT_A_FILE != 0 {
                attacks |= bitboard.0 << 9;
            }
        }
    };

    Bitboard::from(attacks)
}

fn mask_knight_attacks(position: Position, _side: &Colour<()>) -> Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = Bitboard::new().set_bit(position);

    // generate knight attacks
    if bitboard.0 >> 17 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 >> 17;
    }
    if bitboard.0 >> 15 & CONSTS::NOT_A_FILE != 0 {
        attacks |= bitboard.0 >> 15;
    }
    if bitboard.0 >> 10 & CONSTS::NOT_HG_FILE != 0 {
        attacks |= bitboard.0 >> 10;
    }
    if bitboard.0 >> 6 & CONSTS::NOT_AB_FILE != 0 {
        attacks |= bitboard.0 >> 6;
    }

    if bitboard.0 << 17 & CONSTS::NOT_A_FILE != 0 {
        attacks |= bitboard.0 << 17;
    }
    if bitboard.0 << 15 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 << 15;
    }
    if bitboard.0 << 10 & CONSTS::NOT_AB_FILE != 0 {
        attacks |= bitboard.0 << 10;
    }
    if bitboard.0 << 6 & CONSTS::NOT_HG_FILE != 0 {
        attacks |= bitboard.0 << 6;
    }

    Bitboard::from(attacks)
}

fn mask_king_attacks(position: Position, _side: &Colour<()>) -> Bitboard {
    let bitboard = Bitboard::new().set_bit(position);
    let mut attacks: u64 = 0;

    // generate knight attacks
    if bitboard.0 >> 8 != 0 {
        attacks |= bitboard.0 >> 8;
    }
    if bitboard.0 >> 9 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 >> 9;
    }
    if bitboard.0 >> 7 & CONSTS::NOT_A_FILE != 0 {
        attacks |= bitboard.0 >> 7;
    }
    if bitboard.0 >> 1 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 >> 1;
    }

    if bitboard.0 << 8 != 0 {
        attacks |= bitboard.0 << 8;
    }
    if bitboard.0 << 9 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 << 9;
    }
    if bitboard.0 << 7 & CONSTS::NOT_A_FILE != 0 {
        attacks |= bitboard.0 << 7;
    }
    if bitboard.0 << 1 & CONSTS::NOT_H_FILE != 0 {
        attacks |= bitboard.0 << 1;
    }

    Bitboard::from(attacks)
}

fn mask_bishop_attacks(position: Position, _side: &Colour<()>) -> Bitboard {
    let tb = |r: u64, f: u64| (r * 8 + f);

    // Target ranks and files
    let t_rank: u64 = position as u64 / 8; // Cloning an int is negligible
    let t_file: u64 = position as u64 % 8;

    let mut attacks: u64 = 0;

    attacks = (t_rank + 1..=6)
        .zip(t_file + 1..=6)
        .fold(attacks, |acc, (r, f)| acc | 1u64 << tb(r, f));

    attacks = (1..t_rank)
        .rev()
        .zip(t_file + 1..=6)
        .fold(attacks, |acc, (r, f)| acc | 1u64 << tb(r, f));

    attacks = (t_rank + 1..=6)
        .zip((1..t_file).rev())
        .fold(attacks, |acc, (r, f)| acc | 1u64 << tb(r, f));

    attacks = (1..t_rank)
        .rev()
        .zip((1..t_file).rev())
        .fold(attacks, |acc, (r, f)| acc | 1u64 << tb(r, f));

    Bitboard::from(attacks)
}

fn mask_rook_attacks(position: Position, _side: &Colour<()>) -> Bitboard {
    let mut attacks: u64 = 0;
    let tb = |r: u64, f: u64| (r * 8 + f);

    // Target ranks and files
    let t_rank: u64 = position as u64 / 8; // Cloning an int is negligible
    let t_file: u64 = position as u64 % 8;

    // Combine vertical directions (upward and downward)
    attacks = (t_rank + 1..=6)
        .chain((1..t_rank).rev())
        .fold(attacks, |acc, r| acc | 1u64 << tb(r, t_file));

    // Combine horizontal directions (right and left)
    attacks = (t_file + 1..=6)
        .chain((1..t_file).rev())
        .fold(attacks, |acc, f| acc | 1u64 << tb(t_rank, f));

    Bitboard::from(attacks)
}

// === Attack generation ===
fn fly_gen_bishop_attks(position: impl Into<u64>, block: &Bitboard) -> Bitboard {
    let idx = |rank: u64, file: u64| {
        Position::from_u64(rank * 8 + file).expect("Out of bounds rank or file")
    };

    // Target ranks and files
    let pos: u64 = position.into();
    let t_rank: u64 = pos / 8;
    let t_file: u64 = pos % 8;

    // // Bottom right
    let mut attacks = (t_rank + 1..=7)
        .zip(t_file + 1..=7)
        .take_while_inclusive(|&(r, f)| !block.is_occupied(idx(r, f)))
        .fold(0u64, |acc, (r, f)| acc | 1u64 << idx(r, f) as u64);

    // Top right
    attacks = (0..t_rank)
        .rev()
        .zip(t_file + 1..=7)
        .take_while_inclusive(|&(r, f)| !block.is_occupied(idx(r, f)))
        .fold(attacks, |acc, (r, f)| acc | 1u64 << idx(r, f) as u64);

    // Bottom left
    attacks = (t_rank + 1..=7)
        .zip((0..t_file).rev())
        .take_while_inclusive(|&(r, f)| !block.is_occupied(idx(r, f)))
        .fold(attacks, |acc, (r, f)| acc | 1u64 << idx(r, f) as u64);

    // Top left
    attacks = (0..t_rank)
        .rev()
        .zip((0..t_file).rev())
        .take_while_inclusive(|&(r, f)| !block.is_occupied(idx(r, f)))
        .fold(attacks, |acc, (r, f)| acc | 1u64 << idx(r, f) as u64);

    Bitboard::from(attacks)
}

// Generate Rook Attacks on the fly
fn fly_gen_rook_attks(position: Position, block: &Bitboard) -> Bitboard {
    let idx = |rank: u64, file: u64| {
        Position::from_u64(rank * 8 + file).expect("Out of bounds rank or file")
    };

    // Target ranks and files
    let t_rank: u64 = position as u64 / 8; // Cloning an int is negligible
    let t_file: u64 = position as u64 % 8;

    // Combine vertical directions (upward and downward)
    let mut attacks = (t_rank + 1..=7)
        .take_while_inclusive(|&r| !block.is_occupied(idx(r, t_file)))
        .fold(0u64, |acc, r| acc | 1u64 << idx(r, t_file) as u64);

    attacks = (0..t_rank)
        .rev()
        .take_while_inclusive(|&r| !block.is_occupied(idx(r, t_file)))
        .fold(attacks, |acc, r| acc | 1u64 << idx(r, t_file) as u64);

    // Combine horizontal directions (right and left)
    attacks = (t_file + 1..=7)
        .take_while_inclusive(|&r| !block.is_occupied(idx(r, t_rank)))
        .fold(attacks, |acc, f| acc | 1u64 << idx(t_rank, f) as u64);

    attacks = (0..t_file)
        .rev()
        .take_while_inclusive(|&r| !block.is_occupied(idx(r, t_rank)))
        .fold(attacks, |acc, f| acc | 1u64 << idx(t_rank, f) as u64);

    Bitboard::from(attacks)
}

fn gen_bishop_attacks() -> [[Bitboard; 512]; 64] {
    let mut slider: [[Bitboard; 512]; 64] = [[Bitboard::default(); 512]; 64];

    for p in Position::iter() {
        let attk_mask = mask_bishop_attacks(p, &Colour::White(()));
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let occupancy = Bitboard::new().set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy.0.wrapping_mul(BISHOP_MAGIC_NUMBERS[p as usize]))
                >> (64 - CONSTS::BISHOP_RELEVANT_BITS[p as usize]);

            debug_assert!(
                magic_idx < 512,
                "Magic index out of bounds at square {} → {}",
                p,
                magic_idx
            );

            slider[p as usize][magic_idx as usize] = fly_gen_bishop_attks(p, &occupancy);
        }
    }

    slider
}

fn gen_rook_attacks() -> [[Bitboard; 4096]; 64] {
    let mut slider: [[Bitboard; 4096]; 64] = [[Bitboard::default(); 4096]; 64];

    for p in Position::iter() {
        let attk_mask = mask_rook_attacks(p, &Colour::White(()));
        let rel_bits_count = attk_mask.count_bits();
        let occupancy_indices = 1 << rel_bits_count;

        for idx in 0..occupancy_indices {
            let occupancy = Bitboard::new().set_occupancy(idx, &attk_mask);

            let magic_idx = (occupancy.0.wrapping_mul(CONSTS::ROOK_MAGIC_NUMBERS[p as usize]))
                >> (64 - CONSTS::ROOK_RELEVANT_BITS[p as usize]);

            // Sanity guard to avoid out-of-bounds indexing
            debug_assert!(
                magic_idx < 4096,
                "Magic index out of bounds: square {} → {}",
                p,
                magic_idx
            );

            slider[p as usize][magic_idx as usize] = fly_gen_rook_attks(p, &occupancy);
        }
    }

    slider
}
