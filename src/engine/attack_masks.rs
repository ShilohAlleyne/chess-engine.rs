use crate::board::{bitboard, colour, position};
use crate::traits::const_lookup;

// === Attack Masks ===
pub(crate) fn mask_pawn_attacks<C: const_lookup::ConstFileMask>(
    position: position::Position,
    side: &colour::Colour<()>,
    lookup: C,
) -> bitboard::Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = bitboard::Bitboard::new().set_bit(position);

    match side {
        colour::Colour::White(()) => {
            if (bitboard.0 >> 7) & lookup.not_a_file() != 0 {
                attacks |= bitboard.0 >> 7;
            }
            if (bitboard.0 >> 9) & lookup.not_h_file() != 0 {
                attacks |= bitboard.0 >> 9;
            }
        }
        colour::Colour::Black(()) => {
            if (bitboard.0 << 7) & lookup.not_h_file() != 0 {
                attacks |= bitboard.0 << 7;
            }
            if (bitboard.0 << 9) & lookup.not_a_file() != 0 {
                attacks |= bitboard.0 << 9;
            }
        }
    };

    bitboard::Bitboard::from(attacks)
}

pub(crate) fn mask_knight_attacks<C: const_lookup::ConstFileMask>(
    position: position::Position,
    _side: &colour::Colour<()>,
    lookup: C,
) -> bitboard::Bitboard {
    let mut attacks: u64 = 0;
    let bitboard = bitboard::Bitboard::new().set_bit(position);

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

    bitboard::Bitboard::from(attacks)
}

pub(crate) fn mask_king_attacks<C: const_lookup::ConstFileMask>(
    position: position::Position,
    _side: &colour::Colour<()>,
    lookup: C,
) -> bitboard::Bitboard {
    let bitboard = bitboard::Bitboard::new().set_bit(position);
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

    bitboard::Bitboard::from(attacks)
}

pub(crate) fn mask_bishop_attacks(
    position: position::Position,
    _side: &colour::Colour<()>,
) -> bitboard::Bitboard {
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

    bitboard::Bitboard::from(mask)
}

pub(crate) fn mask_rook_attacks(
    position: position::Position,
    _side: &colour::Colour<()>,
) -> bitboard::Bitboard {
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

    bitboard::Bitboard::from(attacks)
}
