pub mod board {
    pub mod bitboard;
    pub mod position;
    pub mod pieces;
    pub mod material_layer;
    pub mod chessboard;
}

pub mod parsers {
    pub mod fen;
    pub mod error;
}

pub mod engine {
    pub mod attack_tables;
    pub mod xor_rand;
    pub mod lazy;
    pub mod magic_numbers;
    pub mod attack_masks;
}

pub mod consts;
