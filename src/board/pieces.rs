use colored::Colorize;
use std::{fmt, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour<T> {
    Red(T),
    White(T),
}

// === Index trait for easy composition ===
// Implement Index for slices
impl<T> Index<&Colour<()>> for [T] {
    type Output = T;

    fn index(&self, colour: &Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Red(()) => &self[1],
        }
    }
}

impl<T> Index<Colour<()>> for [T] {
    type Output = T;

    fn index(&self, colour: Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Red(()) => &self[1],
        }
    }
}

// Indexing Vec<T> with &Colour<()>
impl<T> Index<&Colour<()>> for Vec<T> {
    type Output = T;

    fn index(&self, colour: &Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Red(()) => &self[1],
        }
    }
}

// Implement Index for Vec<T>
impl<T> Index<Colour<()>> for Vec<T> {
    type Output = T;

    fn index(&self, colour: Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Red(()) => &self[1],
        }
    }
}

impl Colour<()> {
    pub fn opp(&self) -> Self {
        match self {
            Colour::Red(()) => Colour::White(()),
            Colour::White(()) => Colour::Red(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece(pub Colour<Kind>);

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Colour::Red(kind) => write!(f, "{}", kind.to_string().red()),
            Colour::White(kind) => write!(f, "{}", kind.to_string().white()),
        }
    }
}

impl TryFrom<&char> for Piece {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match *c {
            'P' => Ok(Piece(Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(Colour::White(Kind::Knight))),
            'B' => Ok(Piece(Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(Colour::White(Kind::Queen))),
            'K' => Ok(Piece(Colour::White(Kind::King))),
            'p' => Ok(Piece(Colour::Red(Kind::Pawn))),
            'n' => Ok(Piece(Colour::Red(Kind::Knight))),
            'b' => Ok(Piece(Colour::Red(Kind::Bishop))),
            'r' => Ok(Piece(Colour::Red(Kind::Rook))),
            'q' => Ok(Piece(Colour::Red(Kind::Queen))),
            'k' => Ok(Piece(Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(Piece(Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(Colour::White(Kind::Knight))),
            'B' => Ok(Piece(Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(Colour::White(Kind::Queen))),
            'K' => Ok(Piece(Colour::White(Kind::King))),
            'p' => Ok(Piece(Colour::Red(Kind::Pawn))),
            'n' => Ok(Piece(Colour::Red(Kind::Knight))),
            'b' => Ok(Piece(Colour::Red(Kind::Bishop))),
            'r' => Ok(Piece(Colour::Red(Kind::Rook))),
            'q' => Ok(Piece(Colour::Red(Kind::Queen))),
            'k' => Ok(Piece(Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Piece {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece(Colour::White(Kind::Pawn))),
            1 => Ok(Piece(Colour::White(Kind::Knight))),
            2 => Ok(Piece(Colour::White(Kind::Bishop))),
            3 => Ok(Piece(Colour::White(Kind::Rook))),
            4 => Ok(Piece(Colour::White(Kind::Queen))),
            5 => Ok(Piece(Colour::White(Kind::King))),
            6 => Ok(Piece(Colour::Red(Kind::Pawn))),
            7 => Ok(Piece(Colour::Red(Kind::Knight))),
            8 => Ok(Piece(Colour::Red(Kind::Bishop))),
            9 => Ok(Piece(Colour::Red(Kind::Rook))),
            10 => Ok(Piece(Colour::Red(Kind::Queen))),
            11 => Ok(Piece(Colour::Red(Kind::King))),
            _ => Err(()),
        }
    }
}

impl Piece {
    pub(crate) fn index(&self) -> usize {
        match self.0 {
            Colour::White(kind) => match kind {
                Kind::Pawn => 0,
                Kind::Knight => 1,
                Kind::Bishop => 2,
                Kind::Rook => 3,
                Kind::Queen => 4,
                Kind::King => 5,
            },
            Colour::Red(kind) => match kind {
                Kind::Pawn => 6,
                Kind::Knight => 7,
                Kind::Bishop => 8,
                Kind::Rook => 9,
                Kind::Queen => 10,
                Kind::King => 11,
            },
        }
    }

    pub(crate) fn from_colour_kind(colour: &Colour<()>, kind: Kind) -> Self {
        match colour {
            Colour::White(_) => Piece(Colour::White(kind)),
            Colour::Red(_) => Piece(Colour::Red(kind)),
        }
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
