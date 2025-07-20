use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::board::{bitboard::Bitboard, pieces::Colour, position::Position};

use super::attack_tables::{gen_attacks, gen_bishop_attacks, gen_rook_attacks};
use super::attack_masks::{mask_bishop_attacks, mask_king_attacks, mask_knight_attacks, mask_pawn_attacks, mask_rook_attacks};


// Attacks
pub(crate) static PAWN_ATTACKS: Lazy<[[Bitboard; 64]; 2]> = Lazy::new(|| [
    gen_attacks(mask_pawn_attacks, Colour::White(())),
    gen_attacks(mask_pawn_attacks, Colour::Red(())),
]);

pub(crate) static KNIGHT_ATTACKS: Lazy<[Bitboard; 64]> = Lazy::new(|| {
    gen_attacks(mask_knight_attacks, Colour::White(()))
});

pub(crate) static KING_ATTACKS: Lazy<[Bitboard; 64]> = Lazy::new(|| {
    gen_attacks(mask_king_attacks, Colour::White(()))
});

pub(crate) static BISHOP_ATTACKS: Lazy<Box<[[Bitboard; 512]; 64]>> = Lazy::new(|| {
    Box::new(gen_bishop_attacks())
});

pub(crate) static ROOK_ATTACKS: Lazy<Box<[[Bitboard; 4096]; 64]>> = Lazy::new(|| {
    Box::new(gen_rook_attacks())
});

// Masks
pub(crate) static BISHOP_MASKS: Lazy<[Bitboard; 64]> = Lazy::new(|| {
    let mut masks = [Bitboard::default(); 64];
    for pos in Position::iter() {
        masks[pos] = mask_bishop_attacks(pos, &Colour::White(()));
    }
    masks
});

pub(crate) static ROOK_MASKS: Lazy<[Bitboard; 64]> = Lazy::new(|| {
    let mut masks = [Bitboard::default(); 64];
    for pos in Position::iter() {
        masks[pos] = mask_rook_attacks(pos, &Colour::White(())); // Or consolidated logic
    }
    masks
});

