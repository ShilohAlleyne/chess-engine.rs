use crate::board::{pieces::Piece, position::Position};

#[derive(Debug)]
pub enum Action {
    Push(Detail),
    Promotion(Detail),
    Capture { detail: Detail, captures: Piece },
    CapturePromotion {detail: Detail, captures: Piece },
    Enpassant {detail: Detail, captures: Piece},
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
    pub piece: Piece,
    pub source: Position,
    pub target: Position,
}
