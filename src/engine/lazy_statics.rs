use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::board::{bitboard, colour, position};
use crate::effects::{
    magic_number_provider as MAGIC_NUM_LOOKUP, 
    relavant_bits_provider as REL_BITS_LOOKUP,
    static_mask_provider as STATIC_MASK_LOOKUP,
    file_mask_provider as FILE_MASK_LOOKUP,
};
use crate::engine::{attack_generation as ATTK_GEN, attack_masks as ATTK_MSK};

// Attacks
pub(crate) static PAWN_ATTACKS: Lazy<[[bitboard::Bitboard; 64]; 2]> = Lazy::new(|| {
    let lookup = FILE_MASK_LOOKUP::ConstFileMasks;
    [
        ATTK_GEN::gen_attacks(
            ATTK_MSK::mask_pawn_attacks,
            colour::Colour::White(()),
            lookup,
        ),
        ATTK_GEN::gen_attacks(ATTK_MSK::mask_pawn_attacks, colour::Colour::Black(()), lookup),
    ]
});

pub(crate) static KNIGHT_ATTACKS: Lazy<[bitboard::Bitboard; 64]> = Lazy::new(|| {
    let lookup = FILE_MASK_LOOKUP::ConstFileMasks;
    ATTK_GEN::gen_attacks(
        ATTK_MSK::mask_knight_attacks,
        colour::Colour::White(()),
        lookup,
    )
});

pub(crate) static KING_ATTACKS: Lazy<[bitboard::Bitboard; 64]> = Lazy::new(|| {
    let lookup = FILE_MASK_LOOKUP::ConstFileMasks;
    ATTK_GEN::gen_attacks(
        ATTK_MSK::mask_king_attacks,
        colour::Colour::White(()),
        lookup,
    )
});

pub(crate) static BISHOP_ATTACKS: Lazy<Box<[[bitboard::Bitboard; 512]; 64]>> = Lazy::new(|| {
    let mask_lookup = STATIC_MASK_LOOKUP::StaticMaskProvider;
    let rel_bits_lookup = REL_BITS_LOOKUP::RelavantBitsProvider;
    let magic_lookup = MAGIC_NUM_LOOKUP::MagicNumberProvider;

    Box::new(ATTK_GEN::gen_bishop_attacks(
        mask_lookup,
        rel_bits_lookup,
        magic_lookup,
    ))
});

pub(crate) static ROOK_ATTACKS: Lazy<Box<[[bitboard::Bitboard; 4096]; 64]>> = Lazy::new(|| {
    let mask_lookup = STATIC_MASK_LOOKUP::StaticMaskProvider;
    let rel_bits_lookup = REL_BITS_LOOKUP::RelavantBitsProvider;
    let magic_lookup = MAGIC_NUM_LOOKUP::MagicNumberProvider;

    Box::new(ATTK_GEN::gen_rook_attacks(
        mask_lookup,
        rel_bits_lookup,
        magic_lookup,
    ))
});

// Masks
pub(crate) static BISHOP_MASKS: Lazy<[bitboard::Bitboard; 64]> = Lazy::new(|| {
    let mut masks = [bitboard::Bitboard::default(); 64];
    for pos in position::Position::iter() {
        masks[pos] = ATTK_MSK::mask_bishop_attacks(pos, &colour::Colour::White(()));
    }
    masks
});

pub(crate) static ROOK_MASKS: Lazy<[bitboard::Bitboard; 64]> = Lazy::new(|| {
    let mut masks = [bitboard::Bitboard::default(); 64];
    for pos in position::Position::iter() {
        masks[pos] = ATTK_MSK::mask_rook_attacks(pos, &colour::Colour::White(()));
        // Or consolidated logic
    }
    masks
});
