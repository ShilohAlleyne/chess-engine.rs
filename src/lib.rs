pub mod board {
    pub mod bitboard;
    pub mod position;
    pub mod pieces;
    pub mod material_layer;
    pub mod gamestate;
    pub mod occupancy_layer;
}

pub mod parsers {
    pub mod fen;
    pub mod error;
}

pub mod engine {
    pub mod attack_tables;
    pub mod xor_rand;
    pub mod lazy_statics;
    pub mod magic_numbers;
    pub mod attack_masks;
    pub mod move_gen;
    pub mod movement;
}

pub mod consts;
