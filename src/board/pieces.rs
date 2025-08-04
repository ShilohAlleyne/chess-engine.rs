use crate::board::colour as COLOUR;
use colored::*;
use std::fmt::{self};

#[derive(Debug, Clone, Copy)]
pub struct Piece(pub COLOUR::Colour<Kind>);

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            COLOUR::Colour::Black(kind) => write!(f, "{}", kind.to_string().red()),
            COLOUR::Colour::White(kind) => write!(f, "{}", kind.to_string().white()),
        }
    }
}

impl TryFrom<&char> for Piece {
    type Error = crate::board::error::Error;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match *c {
            'P' => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            'B' => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            'K' => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            'p' => Ok(Piece(COLOUR::Colour::Black(Kind::Pawn))),
            'n' => Ok(Piece(COLOUR::Colour::Black(Kind::Knight))),
            'b' => Ok(Piece(COLOUR::Colour::Black(Kind::Bishop))),
            'r' => Ok(Piece(COLOUR::Colour::Black(Kind::Rook))),
            'q' => Ok(Piece(COLOUR::Colour::Black(Kind::Queen))),
            'k' => Ok(Piece(COLOUR::Colour::Black(Kind::King))),
            _bad_char => Err(super::error::Error::Deserialization(format!(
                "The character: {} cannot be de-serialised to type Piece.",
                _bad_char
            ))),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = crate::board::error::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            'N' => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            'B' => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            'R' => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            'Q' => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            'K' => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            'p' => Ok(Piece(COLOUR::Colour::Black(Kind::Pawn))),
            'n' => Ok(Piece(COLOUR::Colour::Black(Kind::Knight))),
            'b' => Ok(Piece(COLOUR::Colour::Black(Kind::Bishop))),
            'r' => Ok(Piece(COLOUR::Colour::Black(Kind::Rook))),
            'q' => Ok(Piece(COLOUR::Colour::Black(Kind::Queen))),
            'k' => Ok(Piece(COLOUR::Colour::Black(Kind::King))),
            _bad_char => Err(super::error::Error::Deserialization(format!(
                "The character: {} cannot be de-serialised to type Piece.",
                _bad_char
            ))),
        }
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        match piece.0 {
            COLOUR::Colour::White(Kind::Pawn)   => 'P',
            COLOUR::Colour::White(Kind::Knight) => 'N',
            COLOUR::Colour::White(Kind::Bishop) => 'B',
            COLOUR::Colour::White(Kind::Rook)   => 'R',
            COLOUR::Colour::White(Kind::Queen)  => 'Q',
            COLOUR::Colour::White(Kind::King)   => 'K',
            COLOUR::Colour::Black(Kind::Pawn)     => 'p',
            COLOUR::Colour::Black(Kind::Knight)   => 'n',
            COLOUR::Colour::Black(Kind::Bishop)   => 'b',
            COLOUR::Colour::Black(Kind::Rook)     => 'r',
            COLOUR::Colour::Black(Kind::Queen)    => 'q',
            COLOUR::Colour::Black(Kind::King)     => 'k',
        }
    }
}

impl TryFrom<usize> for Piece {
    type Error = crate::board::error::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece(COLOUR::Colour::White(Kind::Pawn))),
            1 => Ok(Piece(COLOUR::Colour::White(Kind::Knight))),
            2 => Ok(Piece(COLOUR::Colour::White(Kind::Bishop))),
            3 => Ok(Piece(COLOUR::Colour::White(Kind::Rook))),
            4 => Ok(Piece(COLOUR::Colour::White(Kind::Queen))),
            5 => Ok(Piece(COLOUR::Colour::White(Kind::King))),
            6 => Ok(Piece(COLOUR::Colour::Black(Kind::Pawn))),
            7 => Ok(Piece(COLOUR::Colour::Black(Kind::Knight))),
            8 => Ok(Piece(COLOUR::Colour::Black(Kind::Bishop))),
            9 => Ok(Piece(COLOUR::Colour::Black(Kind::Rook))),
            10 => Ok(Piece(COLOUR::Colour::Black(Kind::Queen))),
            11 => Ok(Piece(COLOUR::Colour::Black(Kind::King))),
            _bad_num => Err(super::error::Error::TypeCoversiton(format!(
                "The value: {}, cannot be converted to type Piece",
                _bad_num
            ))),
        }
    }
}

impl From<Piece> for u8 {
    fn from(piece: Piece) -> Self {
        use COLOUR::Colour;

        // Destructure to get the colour and kind
        let (kind, colour_bit) = match piece.0 {
            Colour::White(kind) => (kind, 0),
            Colour::Black(kind) => (kind, 1),
        };

        let kind_id = match kind {
            Kind::Pawn => 1,
            Kind::Knight => 2,
            Kind::Bishop => 3,
            Kind::Rook => 4,
            Kind::Queen => 5,
            Kind::King => 6,
        };

        (colour_bit << 3) | kind_id
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
            COLOUR::Colour::Black(kind) => match kind {
                Kind::Pawn => 6,
                Kind::Knight => 7,
                Kind::Bishop => 8,
                Kind::Rook => 9,
                Kind::Queen => 10,
                Kind::King => 11,
            },
        }
    }
}

pub(crate) fn from_colour_kind(colour: &COLOUR::Colour<()>, kind: Kind) -> Piece {
    match colour {
        COLOUR::Colour::White(_) => Piece(COLOUR::Colour::White(kind)),
        COLOUR::Colour::Black(_) => Piece(COLOUR::Colour::Black(kind)),
    }
}

pub fn try_from_u8(value: u8) -> Result<Option<Piece>, crate::engine::error::Error> {
    if value == 0 {
        return Ok(None); // 0b0000 means "no piece"
    }

    let kind_id = value & 0b0111; // bits 0ÔÇô2
    let colour_flag = (value >> 3) & 0b1; // bit 3

    let kind = match kind_id {
        1 => Kind::Pawn,
        2 => Kind::Knight,
        3 => Kind::Bishop,
        4 => Kind::Rook,
        5 => Kind::Queen,
        6 => Kind::King,
        _int => {
            return Err(crate::engine::error::Error::Encode(format!(
                "Invalid Piece encoding: {}",
                _int
            )))
        } // 0 or >6 is invalid
    };

    let colour = match colour_flag {
        0 => COLOUR::Colour::White(kind),
        1 => COLOUR::Colour::Black(kind),
        _ => unreachable!(),
    };

    Ok(Some(Piece(colour)))
}

pub(crate) fn get_kind(piece: &Piece) -> Kind {
    match piece.0 {
        COLOUR::Colour::White(k) => k,
        COLOUR::Colour::Black(k) => k,
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
