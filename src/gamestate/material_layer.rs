use crate::{
    board::bitboard, board::colour, board::pieces,
    board::position,
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
pub struct MaterialLayer(pub [bitboard::Bitboard; 12]);

// === Traits for easy indexing/bitboard acess ===
impl Index<usize> for MaterialLayer {
    type Output = bitboard::Bitboard;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MaterialLayer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<pieces::Piece> for MaterialLayer {
    type Output = bitboard::Bitboard;

    fn index(&self, piece: pieces::Piece) -> &Self::Output {
        &self.0[piece.index()]
    }
}

impl IndexMut<pieces::Piece> for MaterialLayer {
    fn index_mut(&mut self, piece: pieces::Piece) -> &mut Self::Output {
        &mut self.0[piece.index()]
    }
}

impl Index<position::Position> for MaterialLayer {
    type Output = bitboard::Bitboard;

    fn index(&self, index: position::Position) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<position::Position> for MaterialLayer {
    fn index_mut(&mut self, index: position::Position) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

// Interator traits for easy looping
impl<'a> IntoIterator for &'a MaterialLayer {
    type Item = &'a bitboard::Bitboard;
    type IntoIter = Iter<'a, bitboard::Bitboard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for MaterialLayer {
    type Item = bitboard::Bitboard;
    type IntoIter = std::array::IntoIter<bitboard::Bitboard, 12>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// === Displaying the material layer ===
impl fmt::Display for MaterialLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Composite board of optional pieces
        let mut board: [[Option<pieces::Piece>; 8]; 8] = [[None; 8]; 8];

        // Populate board from bitboards
        for (i, b) in self.0.iter().enumerate() {
            let piece = pieces::Piece::try_from(i).expect("Invalid board index");

            (0..8).for_each(|x| {
                for y in 0..8 {
                    let idx = x * 8 + y;
                    if let Some(pos) = position::Position::from_u64(idx as u64) {
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
        let mut board = MaterialLayer([bitboard::Bitboard::new(); 12]);

        board[pieces::Piece(colour::Colour::White(pieces::Kind::Rook))]
            .mutate_set_bit(position::Position::A1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Knight))]
            .mutate_set_bit(position::Position::B1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Bishop))]
            .mutate_set_bit(position::Position::C1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Queen))]
            .mutate_set_bit(position::Position::D1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::King))]
            .mutate_set_bit(position::Position::E1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Bishop))]
            .mutate_set_bit(position::Position::F1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Knight))]
            .mutate_set_bit(position::Position::G1);
        board[pieces::Piece(colour::Colour::White(pieces::Kind::Rook))]
            .mutate_set_bit(position::Position::H1);

        for file in 0..8 {
            if let Some(pos) = position::Position::from_u64(file + 48) {
                board[pieces::Piece(colour::Colour::White(pieces::Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Rook))]
            .mutate_set_bit(position::Position::A8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Knight))]
            .mutate_set_bit(position::Position::B8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Bishop))]
            .mutate_set_bit(position::Position::C8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Queen))]
            .mutate_set_bit(position::Position::D8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::King))]
            .mutate_set_bit(position::Position::E8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Bishop))]
            .mutate_set_bit(position::Position::F8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Knight))]
            .mutate_set_bit(position::Position::G8);
        board[pieces::Piece(colour::Colour::Black(pieces::Kind::Rook))]
            .mutate_set_bit(position::Position::H8);

        for file in 0..8 {
            if let Some(pos) = position::Position::from_u64(file + 8) {
                board[pieces::Piece(colour::Colour::Black(pieces::Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board
    }

    pub fn iter(&self) -> std::slice::Iter<'_, bitboard::Bitboard> {
        self.0.iter()
    }
}

impl Default for MaterialLayer {
    fn default() -> Self {
        Self::new()
    }
}
