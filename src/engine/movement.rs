use crate::board::{pieces::Piece, position::Position};

#[derive(Debug)]
pub enum Action {
    Push(Detail),
    Promotion(Detail),
    Capture { detail: Detail, captures: Piece },
    CapturePromotion {detail: Detail, captures: Piece },
    Enpassant {detail: Detail, captures: Piece},
}

#[derive(Debug)]
pub struct Detail {
    pub piece: Piece,
    pub source: Position,
    pub target: Position,
}
