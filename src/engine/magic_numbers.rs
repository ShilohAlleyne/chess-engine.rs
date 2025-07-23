// for the first pass, I'll just use u64
use super::{
    attack_generation as ATTK_GEN,
    attack_masks as ATTK_MSK,
};
use crate::{
    board::{bitboard::Bitboard, colour as COLOUR, position::Position},
    consts::BIT_TABLE,
};
use rand::Rng;

// Gen a radom number
fn rand_u64() -> u64 {
    let mut rng = rand::rng();

    let u1 = rng.random::<u64>() & 0xFFFF;
    let u2 = rng.random::<u64>() & 0xFFFF;
    let u3 = rng.random::<u64>() & 0xFFFF;
    let u4 = rng.random::<u64>() & 0xFFFF;

    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

// Check for low amount of non Zero bits
fn few_bits() -> u64 {
    rand_u64() & rand_u64() & rand_u64()
}

fn pop_1st_bit(bb: u64) -> (u64, u64) {
    let b = bb ^ (bb - 1);
    let fold = ((b & 0xffffffff) ^ (b >> 32)) as u32;
    let new_bb = bb & (bb - 1);
    let index = ((fold.wrapping_mul(0x783a9b23)) >> 26) as usize;
    (new_bb, BIT_TABLE[index])
}

fn index_u64(idx: usize, bits: usize, mut m: u64) -> u64 {
    (0..bits).fold(0u64, |mut res, i| {
        let (new_m, j) = pop_1st_bit(m);
        m = new_m;
        if (idx & (1 << i)) != 0 {
            res |= 1u64 << j;
        }
        res
    })
}

fn transform(b: u64, magic: u64, bits: u32) -> u32 {
    ((b.wrapping_mul(magic)) >> (64 - bits)) as u32
}

pub fn find_magic(sq: Position, m: usize, bishop: bool) -> u64 {
    let mut b: [u64; 4096] = [0u64; 4096];
    let mut a: [u64; 4096] = [0u64; 4096];

    let mask: u64 = if bishop {
        ATTK_MSK::mask_bishop_attacks(sq, &COLOUR::Colour::White(()))
    } else {
        ATTK_MSK::mask_rook_attacks(sq, &COLOUR::Colour::White(()))
    }
    .0;

    let n = mask.count_ones();

    for i in 0..(1 << n) {
        let occ = Bitboard(index_u64(i, n as usize, mask));
        b[i] = occ.0;
        a[i] = if bishop {
            ATTK_GEN::fly_gen_bishop_attks(sq, &occ).0
        } else {
            ATTK_GEN::fly_gen_rook_attks(sq, &occ).0
        };
    }

    // === Inline validation helper ===
    let validate = |magic: u64| -> bool {
        let mut used = [0u64; 4096];
        for i in 0..(1 << n) {
            let index = transform(b[i], magic, m as u32) as usize;
            if used[index] == 0 {
                used[index] = a[i];
            } else if used[index] != a[i] {
                return false;
            }
        }
        true
    };

    for _ in 0..100_000_000 {
        let magic = few_bits();
        let high_entropy = (mask.wrapping_mul(magic) & 0xFF00000000000000).count_ones();
        if high_entropy < 6 {
            continue;
        }

        if validate(magic) {
            return magic;
        }
    }

    // No valid magic found
    0
}
