pub mod board {
    pub mod bitboard;
    pub mod position;
    pub mod pieces;
    pub mod colour;
    pub mod castling;
    pub mod error;
}

pub mod parsers {
    pub mod fen;
    pub mod error;
}

pub mod engine {
    pub mod attack_generation;
    pub mod lazy_statics;
    pub mod magic_numbers;
    pub mod attack_masks;
    pub mod move_gen;
    pub mod movement;
    pub mod error;
}

pub mod gamestate {
    pub mod boardstate;
    pub mod occupancy_layer;
    pub mod material_layer;
}

pub mod traits {
    pub mod static_lookup;
    pub mod const_lookup;
}

pub mod effects {
    pub mod magic_number_provider;
    pub mod relavant_bits_provider;
    pub mod static_mask_provider;
    pub mod static_attack_provider;
    pub mod file_mask_provider;
}

pub mod consts;
