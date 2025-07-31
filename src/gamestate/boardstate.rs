use super::{material_layer as ML, occupancy_layer as OCCUPANCY};
use crate::{
    board::{
        self, bitboard as BITBOARD, castling as CR, colour as COLOUR, pieces as PIECE, position as POSITION
    },
    effects::static_attack_provider as STATIC_ATTK_LOOKUP,
    parsers::error::ParserError,
    traits::static_lookup as PRECOMP,
};
use std::fmt;
use strum::IntoEnumIterator;

// === The full chessboard with meta data ===
#[derive(Debug)]
pub struct State {
    pub material_layer: ML::MaterialLayer,
    pub occpancy_layer: OCCUPANCY::OccupancyLayer,
    pub side_to_move: COLOUR::Colour<()>,
    pub en_passant: Option<POSITION::Position>,
    pub castling: u8,
    pub half_moves: u32,
    pub full_moves: u32,
}

// === Display the full chessboard information ===
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_to_move: &str = match self.side_to_move {
            COLOUR::Colour::White(()) => "White",
            COLOUR::Colour::Red(()) => "Red",
        };

        writeln!(f, "{}", self.material_layer)?;
        writeln!(f, "Side to move: {}", side_to_move)?;

        match self.en_passant {
            Some(pos) => writeln!(f, "En passant: {}", pos)?,
            None => writeln!(f, "En passant: No")?,
        }

        write!(f, "Castling rights: ")?;
        if self.castling > 0 {
            for (i, castle) in castling_rights_from_bits(self).enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", castle)?;
            }
        } else {
            write!(f, "None")?;
        }
        writeln!(f, "\nHalf move count: {}", self.half_moves)?;
        writeln!(f, "Full move count: {}", self.full_moves)?;

        Ok(())
    }
}

// === Default impl ~ generates a black material_layer instead of a normal setup ===
impl Default for State {
    fn default() -> Self {
        Self {
            material_layer: ML::MaterialLayer([BITBOARD::Bitboard::new(); 12]),
            occpancy_layer: OCCUPANCY::OccupancyLayer([BITBOARD::Bitboard::new(); 2]),
            side_to_move: COLOUR::Colour::White(()),
            en_passant: None,
            castling: 0,
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            material_layer: ML::MaterialLayer::new(),
            occpancy_layer: OCCUPANCY::OccupancyLayer::new(),
            side_to_move: COLOUR::Colour::White(()),
            en_passant: None,
            castling: 0,
            half_moves: 0,
            full_moves: 0,
        }
    }

    // === Chess notation parsers ===
    pub fn try_from_fen(fen: &str) -> Result<Self, ParserError> {
        crate::parsers::fen::parse(fen)
    }

    // === Castling Rights bitmask operations ===
    pub fn set_castling_rights(&mut self, rights: &[CR::CastlingRights]) {
        self.castling = rights.iter().fold(0, |acc, r| acc | r.get_castlings_bits())
    }

    pub fn add_castling_right(&mut self, right: CR::CastlingRights) {
        self.castling |= right.get_castlings_bits();
    }

    pub fn toggle_castling_rights(&mut self, right: CR::CastlingRights) {
        self.castling ^= right.get_castlings_bits()
    }
}

// === Castling ===
pub fn castling_rights_from_bits(board: &State) -> impl Iterator<Item = CR::CastlingRights> + '_ {
    [
        CR::CastlingRights::WK,
        CR::CastlingRights::WQ,
        CR::CastlingRights::RK,
        CR::CastlingRights::RQ,
    ]
    .iter()
    .copied()
    .filter(move |r| board.castling & r.get_castlings_bits() != 0)
}


// === Attacks ===
pub fn is_attacked<A: PRECOMP::StaticAttack>(
    board: &State,
    pos: POSITION::Position,
    sttk_attk: A,
) -> bool {
    let occ = OCCUPANCY::get_both(&board.occpancy_layer);
    let attacker = board.side_to_move.opp();

    [
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::Pawn)]
            & sttk_attk.pawn(pos, attacker),
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::Knight)]
            & sttk_attk.knight(pos),
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::King)]
            & sttk_attk.knight(pos), // ← intentional reuse? Maybe clarify if king table is separate
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::Bishop)]
            & sttk_attk.bishop(pos, occ),
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::Rook)]
            & sttk_attk.rook(pos, occ),
        board.material_layer[PIECE::Piece::from_colour_kind(&attacker, PIECE::Kind::Queen)]
            & sttk_attk.queen(pos, occ),
    ]
    .iter()
    .any(|bb| bb.0 != 0)
}

pub fn current_attacks(board: &State) -> BITBOARD::Bitboard {
    // Marker for static lookup
    let atk_provider = STATIC_ATTK_LOOKUP::StaticAttackProvider;
    let mut bb = BITBOARD::Bitboard::new();

    for pos in POSITION::Position::iter() {
        if is_attacked(board, pos, atk_provider) {
            bb.mutate_set_bit(pos);
        }
    }

    bb
}

pub fn get_piece_at_pos(board: &State, pos: POSITION::Position) -> Option<PIECE::Piece> {
    for (i, bb) in board.material_layer.0.iter().enumerate() {
        if bb.is_occupied(pos) {
            return PIECE::Piece::try_from(i).ok();
        }
    }

    None
}
