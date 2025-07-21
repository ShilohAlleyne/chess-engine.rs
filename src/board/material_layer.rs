use crate::{
    board::bitboard::Bitboard,
    board::pieces::{Colour, Kind, Piece},
    board::position::Position,
};
use core::fmt;
use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};

// This struct contains the bitboards for all
// of the pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialLayer(pub [Bitboard; 12]);

// === Traits for easy indexing/bitboard acess ===
impl Index<usize> for MaterialLayer {
    type Output = Bitboard;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MaterialLayer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Piece> for MaterialLayer {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.0[piece.index()]
    }
}

impl IndexMut<Piece> for MaterialLayer {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.0[piece.index()]
    }
}

impl Index<Position> for MaterialLayer {
    type Output = Bitboard;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Position> for MaterialLayer {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

// Interator traits for easy looping
impl<'a> IntoIterator for &'a MaterialLayer {
    type Item = &'a Bitboard;
    type IntoIter = Iter<'a, Bitboard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for MaterialLayer {
    type Item = Bitboard;
    type IntoIter = std::array::IntoIter<Bitboard, 12>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// === Displaying the material layer ===
impl fmt::Display for MaterialLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Composite board of optional pieces
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        // Populate board from bitboards
        for (i, b) in self.0.iter().enumerate() {
            let piece = Piece::try_from(i).expect("Invalid board index");

            for x in 0..8 {
                for y in 0..8 {
                    let idx = x * 8 + y;
                    if let Some(pos) = Position::from_u64(idx as u64) {
                        if b.is_occupied(pos) {
                            board[x][y] = Some(piece);
                        }
                    }
                }
            }
        }

        // Render board
        writeln!(f)?;
        for (i, rank) in board.iter().enumerate() {
            write!(f, "{} | ", 8 - i)?;
            for &tile in rank {
                match tile {
                    Some(p) => write!(f, "{} ", p)?,
                    None => write!(f, ". ")?,
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
        let mut board = MaterialLayer([Bitboard::new(); 12]);

        board[Piece(Colour::White(Kind::Rook))].mutate_set_bit(Position::A1);
        board[Piece(Colour::White(Kind::Knight))].mutate_set_bit(Position::B1);
        board[Piece(Colour::White(Kind::Bishop))].mutate_set_bit(Position::C1);
        board[Piece(Colour::White(Kind::Queen))].mutate_set_bit(Position::D1);
        board[Piece(Colour::White(Kind::King))].mutate_set_bit(Position::E1);
        board[Piece(Colour::White(Kind::Bishop))].mutate_set_bit(Position::F1);
        board[Piece(Colour::White(Kind::Knight))].mutate_set_bit(Position::G1);
        board[Piece(Colour::White(Kind::Rook))].mutate_set_bit(Position::H1);

        for file in 0..8 {
            if let Some(pos) = Position::from_u64(file + 48) {
                board[Piece(Colour::White(Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board[Piece(Colour::Red(Kind::Rook))].mutate_set_bit(Position::A8);
        board[Piece(Colour::Red(Kind::Knight))].mutate_set_bit(Position::B8);
        board[Piece(Colour::Red(Kind::Bishop))].mutate_set_bit(Position::C8);
        board[Piece(Colour::Red(Kind::Queen))].mutate_set_bit(Position::D8);
        board[Piece(Colour::Red(Kind::King))].mutate_set_bit(Position::E8);
        board[Piece(Colour::Red(Kind::Bishop))].mutate_set_bit(Position::F8);
        board[Piece(Colour::Red(Kind::Knight))].mutate_set_bit(Position::G8);
        board[Piece(Colour::Red(Kind::Rook))].mutate_set_bit(Position::H8);

        for file in 0..8 {
            if let Some(pos) = Position::from_u64(file + 8) {
                board[Piece(Colour::Red(Kind::Pawn))].mutate_set_bit(pos);
            }
        }

        board
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Bitboard> {
        self.0.iter()
    }
}

impl Default for MaterialLayer {
    fn default() -> Self {
        Self::new()
    }
}
