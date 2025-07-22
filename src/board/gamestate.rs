use strum::IntoEnumIterator;

use super::{
    bitboard::Bitboard,
    occupancy_layer::OccupancyLayer,
    pieces::{Kind, Piece},
};
use crate::{
    board::{
        material_layer::MaterialLayer,
        pieces::Colour,
        position::{CastlingRights, Position},
    },
    engine::attack_tables::AttackTables,
    parsers::error::ParserError,
};
use std::fmt;

// === The full chessboard with meta data ===
#[derive(Debug)]
pub struct Gamestate {
    pub material_layer: MaterialLayer,
    pub occpancy_layer: OccupancyLayer,
    pub side_to_move: Colour<()>,
    pub en_passant: Option<Position>,
    pub castling: u8,
    pub half_moves: u32,
    pub full_moves: u32,
}

// === Display the full chessboard information ===
impl fmt::Display for Gamestate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_to_move: &str = match self.side_to_move {
            Colour::White(()) => "White",
            Colour::Red(()) => "Red",
        };

        writeln!(f, "{}", self.material_layer)?;
        writeln!(f, "Side to move: {}", side_to_move)?;

        match self.en_passant {
            Some(pos) => writeln!(f, "En passant: {}", pos)?,
            None => writeln!(f, "En passant: No")?,
        }

        write!(f, "Castling rights: ")?;
        if self.castling > 0 {
            for (i, castle) in self.castling_rights_from_bits().iter().enumerate() {
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
impl Default for Gamestate {
    fn default() -> Self {
        Self {
            material_layer: MaterialLayer([Bitboard::new(); 12]),
            occpancy_layer: OccupancyLayer([Bitboard::new(); 2]),
            side_to_move: Colour::White(()),
            en_passant: None,
            castling: 0,
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl Gamestate {
    pub fn new() -> Self {
        Self {
            material_layer: MaterialLayer::new(),
            occpancy_layer: OccupancyLayer::new(),
            side_to_move: Colour::White(()),
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
    pub fn set_castling_rights(&mut self, rights: &[CastlingRights]) {
        self.castling = rights.iter().fold(0, |acc, r| acc | r.get_castlings_bits())
    }

    pub fn add_castling_right(&mut self, right: CastlingRights) {
        self.castling |= right.get_castlings_bits();
    }

    pub fn castling_rights_from_bits(&self) -> Vec<CastlingRights> {
        let all = [
            CastlingRights::WK,
            CastlingRights::WQ,
            CastlingRights::RK,
            CastlingRights::RQ,
        ];
        all.into_iter()
            .filter(|r| self.castling & r.get_castlings_bits() != 0)
            .collect()
    }

    pub fn toggle_castling_rights(&mut self, right: CastlingRights) {
        self.castling ^= right.get_castlings_bits()
    }
}

// === Attacks ===
pub fn is_attacked(board: &Gamestate, pos: Position, attk_tbls: &AttackTables) -> bool {
    let occ = board.occpancy_layer.get_both();
    let attacker = &board.side_to_move.opp(); // the side that could be attacking

    [
        board.material_layer[Piece::from_colour_kind(attacker, Kind::Pawn)]
            & attk_tbls.pawn_attacks[attacker][pos], // ← correct direction
        board.material_layer[Piece::from_colour_kind(attacker, Kind::Knight)]
            & attk_tbls.knight_attacks[pos],
        board.material_layer[Piece::from_colour_kind(attacker, Kind::King)]
            & attk_tbls.king_attacks[pos],
        board.material_layer[Piece::from_colour_kind(attacker, Kind::Bishop)]
            & attk_tbls.get_bishop_attacks(pos, occ),
        board.material_layer[Piece::from_colour_kind(attacker, Kind::Rook)]
            & attk_tbls.get_rook_attacks(pos, occ),
        board.material_layer[Piece::from_colour_kind(attacker, Kind::Queen)]
            & attk_tbls.get_queen_attacks(pos, occ),
    ]
    .iter()
    .any(|bb| bb.0 != 0)
}

pub fn current_attacks(board: &Gamestate, attk_tbls: &AttackTables) -> Bitboard {
    // return a bitboard whos occupancy is the current avaible attacks for a side
    let mut bb: Bitboard = Bitboard::new();

    for pos in Position::iter() {
        if is_attacked(board, pos, attk_tbls) {
            bb.mutate_set_bit(pos);
        }
    }

    bb
}

pub fn get_piece_at_pos(board: &Gamestate, pos: Position) -> Option<Piece> {
    for (i, bb) in board.material_layer.0.iter().enumerate() {
        if bb.is_occupied(pos) {
            return Piece::try_from(i).ok();
        }
    }

    None
}
