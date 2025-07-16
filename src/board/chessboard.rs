use crate::{
    board::{
        material_layer::MaterialLayer,
        pieces::Colour,
        position::{CastlingRights, Position},
    },
    parsers::error::ParserError,
};
use std::fmt;

// === The full chessboard with meta data ===
#[derive(Debug)]
pub struct Chessboard {
    pub material_layer: MaterialLayer,
    pub side_to_move: Colour<()>,
    pub en_passant: Option<Position>,
    pub castling: u8,
    pub half_moves: u32,
    pub full_moves: u32,
}

// === Display the full chessboard information ===
impl fmt::Display for Chessboard {
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

impl Chessboard {
    pub fn new() -> Self {
        Self {
            material_layer: MaterialLayer::new(),
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
