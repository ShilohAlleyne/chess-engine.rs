use crate::{
    board::bitboard as BITBOARD, board::colour as COLOUR, board::pieces as PIECE,
    board::position as POSITION,
};
use core::fmt;
use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};
use colored::*;

// This struct contains the bitboards for all
// of the pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialLayer(pub [BITBOARD::Bitboard; 12]);

// === Traits for easy indexing/bitboard acess ===
impl Index<usize> for MaterialLayer {
    type Output = BITBOARD::Bitboard;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MaterialLayer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<PIECE::Piece> for MaterialLayer {
    type Output = BITBOARD::Bitboard;

    fn index(&self, piece: PIECE::Piece) -> &Self::Output {
        &self.0[piece.index()]
    }
}

impl IndexMut<PIECE::Piece> for MaterialLayer {
    fn index_mut(&mut self, piece: PIECE::Piece) -> &mut Self::Output {
        &mut self.0[piece.index()]
    }
}

impl Index<POSITION::Position> for MaterialLayer {
    type Output = BITBOARD::Bitboard;

    fn index(&self, index: POSITION::Position) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<POSITION::Position> for MaterialLayer {
    fn index_mut(&mut self, index: POSITION::Position) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

// Interator traits for easy looping
impl<'a> IntoIterator for &'a MaterialLayer {
    type Item = &'a BITBOARD::Bitboard;
    type IntoIter = Iter<'a, BITBOARD::Bitboard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for MaterialLayer {
    type Item = BITBOARD::Bitboard;
    type IntoIter = std::array::IntoIter<BITBOARD::Bitboard, 12>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// === Displaying the material layer ===
impl fmt::Display for MaterialLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Composite board of optional pieces
        let mut board: [[Option<PIECE::Piece>; 8]; 8] = [[None; 8]; 8];

        // Populate board from bitboards
        for (i, b) in self.0.iter().enumerate() {
            let piece = PIECE::Piece::try_from(i).expect("Invalid board index");

            (0..8).for_each(|x| {
                for y in 0..8 {
                    let idx = x * 8 + y;
                    if let Some(pos) = POSITION::Position::from_u64(idx as u64) {
                        if b.is_occupied(pos) {
                            board[x][y] = Some(piece);
                        }
                    }
                }
            });
        }

        // Render board
        writeln!(f)?;
        for (rank_idx, rank) in board.iter().enumerate() {
            write!(f, "{} | ", 8 - rank_idx)?;
            for (file_idx, &tile) in rank.iter().enumerate() {
                let shift = rank_idx % 2;
                match tile {
                    Some(p) => write!(f, "{} ", p)?,
                    None => {
                        if (file_idx + shift) % 2 == 0 {
                            write!(f, "{} ", "\u{26F6}".bright_black())?
                        } else {
                            write!(f, "{} ", "\u{26F6}".black())?
                        }
                    },
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "  -----------------")?;
        writeln!(f, "    a b c d e f g h")
    }
}

impl MaterialLayer {
    pub fn new() -> Self {
        let mut board = MaterialLayer([BITBOARD::Bitboard::new(); 12]);

        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Rook))]
            .mutate_set_bit(POSITION::Position::A1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Knight))]
            .mutate_set_bit(POSITION::Position::B1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Bishop))]
            .mutate_set_bit(POSITION::Position::C1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Queen))]
            .mutate_set_bit(POSITION::Position::D1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::King))]
            .mutate_set_bit(POSITION::Position::E1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Bishop))]
            .mutate_set_bit(POSITION::Position::F1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Knight))]
            .mutate_set_bit(POSITION::Position::G1);
        board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Rook))]
            .mutate_set_bit(POSITION::Position::H1);

        for file in 0..8 {
            if let Some(pos) = POSITION::Position::from_u64(file + 48) {
                board[PIECE::Piece(COLOUR::Colour::White(PIECE::Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Rook))]
            .mutate_set_bit(POSITION::Position::A8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Knight))]
            .mutate_set_bit(POSITION::Position::B8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Bishop))]
            .mutate_set_bit(POSITION::Position::C8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Queen))]
            .mutate_set_bit(POSITION::Position::D8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::King))]
            .mutate_set_bit(POSITION::Position::E8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Bishop))]
            .mutate_set_bit(POSITION::Position::F8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Knight))]
            .mutate_set_bit(POSITION::Position::G8);
        board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Rook))]
            .mutate_set_bit(POSITION::Position::H8);

        for file in 0..8 {
            if let Some(pos) = POSITION::Position::from_u64(file + 8) {
                board[PIECE::Piece(COLOUR::Colour::Red(PIECE::Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BITBOARD::Bitboard> {
        self.0.iter()
    }
}

impl Default for MaterialLayer {
    fn default() -> Self {
        Self::new()
    }
}
