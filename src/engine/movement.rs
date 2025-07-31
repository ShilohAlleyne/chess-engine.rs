use crate::board::{castling as CR, pieces as PIECE, position as POSITION};

#[derive(Debug)]
pub enum Action {
    Push(Detail),
    Promotion(Detail),
    Capture {
        detail: Detail,
        captures: PIECE::Piece,
    },
    CapturePromotion {
        detail: Detail,
        captures: PIECE::Piece,
    },
    Enpassant {
        detail: Detail,
        captures: PIECE::Piece,
    },
    Castle(CR::CastlingRights),
    Reposition(Detail),
    Check(Detail),
    CaptureWithCheck { detail: Detail, captures: PIECE::Piece }
}

// Iterator impl for lazy actions
impl IntoIterator for Action {
    type Item = Action;
    type IntoIter = std::iter::Once<Action>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

#[derive(Debug)]
pub struct Detail {
    pub piece: PIECE::Piece,
    pub source: POSITION::Position,
    pub target: POSITION::Position,
}
