use crate::board::{bitboard as BITBOARD, colour as COLOUR, position as POSITION};
use crate::traits::const_lookup as CONST_LOOKUP;

// === Attack Masks ===
pub(crate) fn mask_pawn_attacks<C: CONST_LOOKUP::ConstFileMask>(
    position: POSITION::Position,
    side: &COLOUR::Colour<()>,
    lookup: C,
) -> BITBOARD::Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = BITBOARD::Bitboard::new().set_bit(position);

    match side {
        COLOUR::Colour::White(()) => {
            if (bitboard.0 >> 7) & lookup.not_a_file() != 0 {
                attacks |= bitboard.0 >> 7;
            }
            if (bitboard.0 >> 9) & lookup.not_h_file() != 0 {
                attacks |= bitboard.0 >> 9;
            }
        }
        COLOUR::Colour::Red(()) => {
            if (bitboard.0 << 7) & lookup.not_h_file() != 0 {
                attacks |= bitboard.0 << 7;
            }
            if (bitboard.0 << 9) & lookup.not_a_file() != 0 {
                attacks |= bitboard.0 << 9;
            }
        }
    };

    BITBOARD::Bitboard::from(attacks)
}

pub(crate) fn mask_knight_attacks<C: CONST_LOOKUP::ConstFileMask>(
    position: POSITION::Position,
    _side: &COLOUR::Colour<()>,
    lookup: C,
) -> BITBOARD::Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = BITBOARD::Bitboard::new().set_bit(position);

    // generate knight attacks
    if bitboard.0 >> 17 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 >> 17;
    }
    if bitboard.0 >> 15 & lookup.not_a_file() != 0 {
        attacks |= bitboard.0 >> 15;
    }
    if bitboard.0 >> 10 & lookup.not_hg_file() != 0 {
        attacks |= bitboard.0 >> 10;
    }
    if bitboard.0 >> 6 & lookup.not_ab_file() != 0 {
        attacks |= bitboard.0 >> 6;
    }

    if bitboard.0 << 17 & lookup.not_a_file() != 0 {
        attacks |= bitboard.0 << 17;
    }
    if bitboard.0 << 15 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 << 15;
    }
    if bitboard.0 << 10 & lookup.not_ab_file() != 0 {
        attacks |= bitboard.0 << 10;
    }
    if bitboard.0 << 6 & lookup.not_hg_file() != 0 {
        attacks |= bitboard.0 << 6;
    }

    BITBOARD::Bitboard::from(attacks)
}

pub(crate) fn mask_king_attacks<C: CONST_LOOKUP::ConstFileMask>(
    position: POSITION::Position,
    _side: &COLOUR::Colour<()>,
    lookup: C,
) -> BITBOARD::Bitboard {
    let bitboard = BITBOARD::Bitboard::new().set_bit(position);
    let mut attacks: u64 = 0;

    // generate knight attacks
    if bitboard.0 >> 8 != 0 {
        attacks |= bitboard.0 >> 8;
    }
    if bitboard.0 >> 9 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 >> 9;
    }
    if bitboard.0 >> 7 & lookup.not_a_file() != 0 {
        attacks |= bitboard.0 >> 7;
    }
    if bitboard.0 >> 1 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 >> 1;
    }

    if bitboard.0 << 8 != 0 {
        attacks |= bitboard.0 << 8;
    }
    if bitboard.0 << 9 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 << 9;
    }
    if bitboard.0 << 7 & lookup.not_a_file() != 0 {
        attacks |= bitboard.0 << 7;
    }
    if bitboard.0 << 1 & lookup.not_h_file() != 0 {
        attacks |= bitboard.0 << 1;
    }

    BITBOARD::Bitboard::from(attacks)
}

pub(crate) fn mask_bishop_attacks(
    position: POSITION::Position,
    _side: &COLOUR::Colour<()>,
) -> BITBOARD::Bitboard {
    let mut mask = 0u64;

    let t_rank = position as u64 / 8;
    let t_file = position as u64 % 8;

    let tb = |r: u64, f: u64| r * 8 + f;

    // Directions: NE, NW, SE, SW
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for &(dr, df) in &directions {
        let mut r = t_rank as i64 + dr;
        let mut f = t_file as i64 + df;

        while (1..7).contains(&r) && (1..7).contains(&f) {
            mask |= 1u64 << tb(r as u64, f as u64);
            r += dr;
            f += df;
        }
    }

    BITBOARD::Bitboard::from(mask)
}

pub(crate) fn mask_rook_attacks(
    position: POSITION::Position,
    _side: &COLOUR::Colour<()>,
) -> BITBOARD::Bitboard {
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

    BITBOARD::Bitboard::from(attacks)
}
