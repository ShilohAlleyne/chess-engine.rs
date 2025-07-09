use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    bitboard::Bitboard,
    position::{Colour, Position},
};

// This Struct will hold all the pre-generated
// attack tables for each piece
pub struct AttackTables {
    pub prawn: AttkTbl,
    pub knight: AttkTbl,
    pub king: AttkTbl,
    // pub bishop: AttkTbl,
    // pub rook: AttkTbl,
}

impl AttackTables {
    pub fn new() -> Self {
        Self {
            prawn: AttkTbl::gen_prawn_attk_tbl(),
            knight: AttkTbl::gen_knight_attk_tbl(),
            king: AttkTbl::gen_king_attk_tbl(),
            // bishop: AttkTbl::gen_slider_attacks(true),
            // rook: AttkTbl::gen_slider_attacks(false),
        }
    }
}

// The actual attack table, I wanted black and white to be named
pub struct AttkTbl {
    pub white: Option<[Bitboard; 64]>,
    pub black: Option<[Bitboard; 64]>,
}

impl AttkTbl {
    // bishop relevant occupancy bit count for every square on board
    const BISHOP_RELEVANT_BITS: [u64; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7,
        5, 5, 5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5,
        5, 5, 5, 6,
    ];

    // rook relevant occupancy bit count for every square on board
    const ROOK_RELEVANT_BITS: [u64; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10,
        11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10,
        10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
    ];

    // rook magic numbers
    const ROOK_MAGIC_NUMBERS: [u64; 64] = [
        0x8a80104000800020,
        0x140002000100040,
        0x2801880a0017001,
        0x100081001000420,
        0x200020010080420,
        0x3001c0002010008,
        0x8480008002000100,
        0x2080088004402900,
        0x800098204000,
        0x2024401000200040,
        0x100802000801000,
        0x120800800801000,
        0x208808088000400,
        0x2802200800400,
        0x2200800100020080,
        0x801000060821100,
        0x80044006422000,
        0x100808020004000,
        0x12108a0010204200,
        0x140848010000802,
        0x481828014002800,
        0x8094004002004100,
        0x4010040010010802,
        0x20008806104,
        0x100400080208000,
        0x2040002120081000,
        0x21200680100081,
        0x20100080080080,
        0x2000a00200410,
        0x20080800400,
        0x80088400100102,
        0x80004600042881,
        0x4040008040800020,
        0x440003000200801,
        0x4200011004500,
        0x188020010100100,
        0x14800401802800,
        0x2080040080800200,
        0x124080204001001,
        0x200046502000484,
        0x480400080088020,
        0x1000422010034000,
        0x30200100110040,
        0x100021010009,
        0x2002080100110004,
        0x202008004008002,
        0x20020004010100,
        0x2048440040820001,
        0x101002200408200,
        0x40802000401080,
        0x4008142004410100,
        0x2060820c0120200,
        0x1001004080100,
        0x20c020080040080,
        0x2935610830022400,
        0x44440041009200,
        0x280001040802101,
        0x2100190040002085,
        0x80c0084100102001,
        0x4024081001000421,
        0x20030a0244872,
        0x12001008414402,
        0x2006104900a0804,
        0x1004081002402,
    ];

    const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
        0x40040844404084,
        0x2004208a004208,
        0x10190041080202,
        0x108060845042010,
        0x581104180800210,
        0x2112080446200010,
        0x1080820820060210,
        0x3c0808410220200,
        0x4050404440404,
        0x21001420088,
        0x24d0080801082102,
        0x1020a0a020400,
        0x40308200402,
        0x4011002100800,
        0x401484104104005,
        0x801010402020200,
        0x400210c3880100,
        0x404022024108200,
        0x810018200204102,
        0x4002801a02003,
        0x85040820080400,
        0x810102c808880400,
        0xe900410884800,
        0x8002020480840102,
        0x220200865090201,
        0x2010100a02021202,
        0x152048408022401,
        0x20080002081110,
        0x4001001021004000,
        0x800040400a011002,
        0xe4004081011002,
        0x1c004001012080,
        0x8004200962a00220,
        0x8422100208500202,
        0x2000402200300c08,
        0x8646020080080080,
        0x80020a0200100808,
        0x2010004880111000,
        0x623000a080011400,
        0x42008c0340209202,
        0x209188240001000,
        0x400408a884001800,
        0x110400a6080400,
        0x1840060a44020800,
        0x90080104000041,
        0x201011000808101,
        0x1a2208080504f080,
        0x8012020600211212,
        0x500861011240000,
        0x180806108200800,
        0x4000020e01040044,
        0x300000261044000a,
        0x802241102020002,
        0x20906061210001,
        0x5a84841004010310,
        0x4010801011c04,
        0xa010109502200,
        0x4a02012000,
        0x500201010098b028,
        0x8040002811040900,
        0x28000010020204,
        0x6000020202d0240,
        0x8918844842082200,
        0x4010011029020020,
    ];

    // Gnerates a attack board for every pos
    // Higher order, uses a attack gen func as an arg
    fn gen_attacks<F>(f: F, side: Colour) -> Option<[Bitboard; 64]>
    where
        F: Fn(Position, &Colour) -> Bitboard,
    {
        let attk_tbl: [Bitboard; 64] = Position::iter()
            .map(|p| f(p, &side))
            .collect::<Vec<Bitboard>>()
            .try_into()
            .expect("Error Generating Attack Table");

        Some(attk_tbl)
    }

    // Prawn
    fn gen_prawn_attk_tbl() -> Self {
        Self {
            white: Self::gen_attacks(Self::mask_prawn_attacks, Colour::White),
            black: Self::gen_attacks(Self::mask_prawn_attacks, Colour::Black),
        }
    }

    fn mask_prawn_attacks(position: Position, side: &Colour) -> Bitboard {
        let mut attacks: u64 = 0;
        let bitboard = Bitboard::new().set_bit(position);

        match side {
            Colour::White => {
                if (bitboard.0 >> 7) & Bitboard::NOT_A_FILE != 0 {
                    attacks |= bitboard.0 >> 7;
                }
                if (bitboard.0 >> 9) & Bitboard::NOT_H_FILE != 0 {
                    attacks |= bitboard.0 >> 9;
                }
            }
            Colour::Black => {
                if (bitboard.0 << 7) & Bitboard::NOT_H_FILE != 0 {
                    attacks |= bitboard.0 << 7;
                }
                if (bitboard.0 << 9) & Bitboard::NOT_A_FILE != 0 {
                    attacks |= bitboard.0 << 9;
                }
            }
        };

        Bitboard::from(attacks)
    }

    // Knight
    fn gen_knight_attk_tbl() -> Self {
        Self {
            // Knights only have one attak table,
            // so we will only gen white and use it for both players
            white: Self::gen_attacks(Self::mask_knight_attacks, Colour::White),
            black: None,
        }
    }

    fn mask_knight_attacks(position: Position, _side: &Colour) -> Bitboard {
        let mut attacks: u64 = 0;
        let bitboard = Bitboard::new().set_bit(position);

        // generate knight attacks
        if bitboard.0 >> 17 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 >> 17;
        }
        if bitboard.0 >> 15 & Bitboard::NOT_A_FILE != 0 {
            attacks |= bitboard.0 >> 15;
        }
        if bitboard.0 >> 10 & Bitboard::NOT_HG_FILE != 0 {
            attacks |= bitboard.0 >> 10;
        }
        if bitboard.0 >> 6 & Bitboard::NOT_AB_FILE != 0 {
            attacks |= bitboard.0 >> 6;
        }

        if bitboard.0 << 17 & Bitboard::NOT_A_FILE != 0 {
            attacks |= bitboard.0 << 17;
        }
        if bitboard.0 << 15 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 << 15;
        }
        if bitboard.0 << 10 & Bitboard::NOT_AB_FILE != 0 {
            attacks |= bitboard.0 << 10;
        }
        if bitboard.0 << 6 & Bitboard::NOT_HG_FILE != 0 {
            attacks |= bitboard.0 << 6;
        }

        Bitboard::from(attacks)
    }

    // King
    fn gen_king_attk_tbl() -> Self {
        // Same as Knights
        Self {
            white: Self::gen_attacks(Self::mask_king_attacks, Colour::White),
            black: None,
        }
    }

    fn mask_king_attacks(position: Position, _side: &Colour) -> Bitboard {
        let bitboard = Bitboard::new().set_bit(position);
        let mut attacks: u64 = 0;

        // generate knight attacks
        if bitboard.0 >> 8 != 0 {
            attacks |= bitboard.0 >> 8;
        }
        if bitboard.0 >> 9 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 >> 9;
        }
        if bitboard.0 >> 7 & Bitboard::NOT_A_FILE != 0 {
            attacks |= bitboard.0 >> 7;
        }
        if bitboard.0 >> 1 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 >> 1;
        }

        if bitboard.0 << 8 != 0 {
            attacks |= bitboard.0 << 8;
        }
        if bitboard.0 << 9 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 << 9;
        }
        if bitboard.0 << 7 & Bitboard::NOT_A_FILE != 0 {
            attacks |= bitboard.0 << 7;
        }
        if bitboard.0 << 1 & Bitboard::NOT_H_FILE != 0 {
            attacks |= bitboard.0 << 1;
        }

        Bitboard::from(attacks)
    }

    // Mask occupancy bits for a magic bitboard
    fn mask_bishop_attacks(position: Position, _side: &Colour) -> Bitboard {
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

    // Generate bishop attacks on the fly
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

    // Rook
    fn gen_rook_attk_tbl() -> Self {
        Self {
            white: Self::gen_attacks(Self::mask_rook_attacks, Colour::White),
            black: None,
        }
    }

    fn mask_rook_attacks(position: Position, _side: &Colour) -> Bitboard {
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

    fn gen_slider_attacks(bishop: bool) -> [[Bitboard; 512]; 64] {
        let mut slider: [[Bitboard; 512]; 64] = [[Bitboard::default(); 512]; 64];

        for p in Position::iter() {
            // Init current mask
            let attk_mask = match bishop {
                true => Self::mask_bishop_attacks(p, &Colour::White),
                false => Self::mask_rook_attacks(p, &Colour::White),
            };

            // Init rel bits count
            let rel_bits_count = attk_mask.count_bits();

            let occupancy_indices = 1 << rel_bits_count;

            for idx in 0..occupancy_indices {
                if bishop {
                    let occupancy = Bitboard::new().set_occupancy(idx, &attk_mask);

                    let magic_idx = (occupancy.0 * Self::BISHOP_MAGIC_NUMBERS[p as usize])
                        >> (64 - Self::BISHOP_RELEVANT_BITS[p as usize]);

                    slider[p as usize][magic_idx as usize] =
                        Self::fly_gen_bishop_attks(p, &occupancy);
                } else {
                    let occupancy = Bitboard::new().set_occupancy(idx, &attk_mask);

                    let magic_idx = (occupancy.0 * Self::ROOK_MAGIC_NUMBERS[p as usize])
                        >> (64 - Self::ROOK_RELEVANT_BITS[p as usize]);

                    slider[p as usize][magic_idx as usize] =
                        Self::fly_gen_rook_attks(p, &occupancy);
                }
            }
        }

        slider
    }
}
