use std::fmt;
use colored::*;
use crate::board::colour as COLOUR;


#[derive(Debug, Clone, Copy)]
pub struct Piece(pub COLOUR::Colour<Kind>);

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            COLOUR::Colour::Red(kind) => write!(f, "{}", kind.to_string().red()),
            COLOUR::Colour::White(kind) => write!(f, "{}", kind.to_string().white()),
        }
    }
}

impl TryFrom<&char> for Piece {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match *c {
            'P' => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            'B' => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            'K' => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            'p' => Ok(Piece(COLOUR::Colour::Red(Kind::Pawn))),
            'n' => Ok(Piece(COLOUR::Colour::Red(Kind::Knight))),
            'b' => Ok(Piece(COLOUR::Colour::Red(Kind::Bishop))),
            'r' => Ok(Piece(COLOUR::Colour::Red(Kind::Rook))),
            'q' => Ok(Piece(COLOUR::Colour::Red(Kind::Queen))),
            'k' => Ok(Piece(COLOUR::Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            'B' => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            'K' => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            'p' => Ok(Piece(COLOUR::Colour::Red(Kind::Pawn))),
            'n' => Ok(Piece(COLOUR::Colour::Red(Kind::Knight))),
            'b' => Ok(Piece(COLOUR::Colour::Red(Kind::Bishop))),
            'r' => Ok(Piece(COLOUR::Colour::Red(Kind::Rook))),
            'q' => Ok(Piece(COLOUR::Colour::Red(Kind::Queen))),
            'k' => Ok(Piece(COLOUR::Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Piece {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            1 => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            2 => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            3 => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            4 => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            5 => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            6 => Ok(Piece(COLOUR::Colour::Red(Kind::Pawn))),
            7 => Ok(Piece(COLOUR::Colour::Red(Kind::Knight))),
            8 => Ok(Piece(COLOUR::Colour::Red(Kind::Bishop))),
            9 => Ok(Piece(COLOUR::Colour::Red(Kind::Rook))),
            10 => Ok(Piece(COLOUR::Colour::Red(Kind::Queen))),
            11 => Ok(Piece(COLOUR::Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl Piece {
    pub(crate) fn index(&self) -> usize {
        match self.0 {
            COLOUR::Colour::White(kind) => match kind {
                Kind::Pawn => 0,
                Kind::Knight => 1,
                Kind::Bishop => 2,
                Kind::Rook => 3,
                Kind::Queen => 4,
                Kind::King => 5,
            },
            COLOUR::Colour::Red(kind) => match kind {
                Kind::Pawn => 6,
                Kind::Knight => 7,
                Kind::Bishop => 8,
                Kind::Rook => 9,
                Kind::Queen => 10,
                Kind::King => 11,
            },
        }
    }

    pub(crate) fn from_colour_kind(colour: &COLOUR::Colour<()>, kind: Kind) -> Self {
        match colour {
            COLOUR::Colour::White(_) => Piece(COLOUR::Colour::White(kind)),
            COLOUR::Colour::Red(_) => Piece(COLOUR::Colour::Red(kind)),
        }
    }
}


pub(crate) fn get_kind(piece: &Piece) -> Kind {
    match piece.0 {
        COLOUR::Colour::White(k) => k,
        COLOUR::Colour::Red(k) => k,
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece: &str = match self {
            Kind::Bishop => "\u{2657}",
            Kind::King => "\u{2654}",
            Kind::Knight => "\u{2658}",
            Kind::Pawn => "\u{2659}\u{FE0E}",
            Kind::Queen => "\u{2655}",
            Kind::Rook => "\u{2656}",
        };
        write!(f, "{}", piece)
    }
}
