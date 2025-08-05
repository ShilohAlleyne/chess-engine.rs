use super::{material_layer, occupancy_layer};
use crate::{
    board::{
        bitboard, castling, colour, pieces,
        position,
    },
    effects::static_attack_provider as STATIC_ATTK_LOOKUP,
    parsers::error::Error,
    traits::static_lookup as PRECOMP,
};
use std::fmt;
use strum::IntoEnumIterator;

// === The full chessboard with meta data ===
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub material_layer: material_layer::MaterialLayer,
    pub occpancy_layer: occupancy_layer::OccupancyLayer,
    pub side_to_move: colour::Colour<()>,
    pub en_passant: Option<position::Position>,
    pub castling: castling::CastlingRights,
    pub half_moves: u32,
    pub full_moves: u32,
}

// === Display the full chessboard information ===
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_to_move: &str = match self.side_to_move {
            colour::Colour::White(()) => "White",
            colour::Colour::Black(()) => "Black",
        };

        writeln!(f, "{}", self.material_layer)?;
        writeln!(f, "Side to move: {}", side_to_move)?;

        match self.en_passant {
            Some(pos) => writeln!(f, "En passant: {}", pos)?,
            None => writeln!(f, "En passant: No")?,
        }

        write!(f, "Castling rights: ")?;
        if self.castling.0 > 0 {
            for (i, castle) in castling::castling_rights_from_bits(self.castling).enumerate() {
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
            material_layer: material_layer::MaterialLayer([bitboard::Bitboard::new(); 12]),
            occpancy_layer: occupancy_layer::OccupancyLayer([bitboard::Bitboard::new(); 2]),
            side_to_move: colour::Colour::White(()),
            en_passant: None,
            castling: castling::CastlingRights::new(),
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            material_layer: material_layer::MaterialLayer::new(),
            occpancy_layer: occupancy_layer::OccupancyLayer::new(),
            side_to_move: colour::Colour::White(()),
            en_passant: None,
            castling: castling::CastlingRights::new(),
            half_moves: 0,
            full_moves: 0,
        }
    }
}

// === Chess notation parsers ===
pub fn try_from_fen(fen: &str) -> Result<State, Error> {
    crate::parsers::fen::parse(fen)
}

// === Chess notation serializers ===
pub fn to_fen(state: State) -> Result<String, crate::parsers::error::Error> {
    crate::parsers::fen::serialize(state)
}

// === Attacks ===
pub fn is_attacked<A: PRECOMP::StaticAttack>(
    board: &State,
    pos: position::Position,
    sttk_attk: A,
) -> bool {
    let occ = occupancy_layer::get_both(&board.occpancy_layer);
    let attacker = board.side_to_move.opp();

    [
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::Pawn)]
            & sttk_attk.pawn(pos, attacker),
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::Knight)]
            & sttk_attk.knight(pos),
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::King)]
            & sttk_attk.knight(pos), // ← intentional reuse? Maybe clarify if king table is separate
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::Bishop)]
            & sttk_attk.bishop(pos, occ),
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::Rook)]
            & sttk_attk.rook(pos, occ),
        board.material_layer[pieces::from_colour_kind(&attacker, pieces::Kind::Queen)]
            & sttk_attk.queen(pos, occ),
    ]
    .iter()
    .any(|bb| bb.0 != 0)
}

pub fn current_attacks(board: &State) -> bitboard::Bitboard {
    // Marker for static lookup
    let atk_provider = STATIC_ATTK_LOOKUP::StaticAttackProvider;
    let mut bb = bitboard::Bitboard::new();

    for pos in position::Position::iter() {
        if is_attacked(board, pos, atk_provider) {
            bb.mutate_set_bit(pos);
        }
    }

    bb
}

pub fn get_piece_at_pos(board: &State, pos: position::Position) -> Option<pieces::Piece> {
    for (i, bb) in board.material_layer.0.iter().enumerate() {
        if bb.is_occupied(pos) {
            return pieces::Piece::try_from(i).ok();
        }
    }

    None
}
