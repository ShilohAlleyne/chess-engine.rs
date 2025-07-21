use itertools::chain;

use super::{attack_tables::AttackTables, movement::Action};
use crate::{
    board::{
        bitboard::Bitboard,
        chessboard::{get_piece_at_pos, is_attacked, Chessboard},
        pieces::{Colour, Kind, Piece},
        position::{CastlingRights, Position},
    },
    engine::movement::Detail,
};

pub fn generate_moves<'a>(
    chessboard: &'a Chessboard,
    attks: &'a AttackTables,
) -> impl Iterator<Item = Action> + 'a {
    // Compose our lazy eval generated moves together
    chain!(
        generate_pawn_moves(
            chessboard.material_layer
                [Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn)],
            chessboard,
            attks
        ),
        generate_castle_moves(chessboard, attks),
    )
}

// === Indivdual piece move gen ===
pub fn generate_pawn_moves<'a>(
    board: Bitboard,
    chessboard: &'a Chessboard,
    attks: &'a AttackTables,
) -> impl Iterator<Item = Action> + 'a {
    board.flat_map(move |source_square| {
        chain!(
            generate_pawn_pushes(source_square, chessboard),
            generate_pawn_pushes2(source_square, chessboard),
            generate_pawn_captures(source_square, chessboard, attks),
            generate_enpassant(source_square, chessboard, attks)
        )
        .flatten()
    })
}

fn generate_pawn_targets(
    source_square: Position,
    chessboard: &Chessboard,
) -> (Option<Position>, Option<Position>) {
    let (forward_one, forward_two) = match chessboard.side_to_move {
        Colour::Red(()) => (-1, -2),
        Colour::White(()) => (1, 2),
    };

    let target_one = source_square
        .change_rank(forward_one)
        .filter(|sq| !chessboard.occpancy_layer.get_both().is_occupied(sq));

    let is_start_rank = matches!(
        (source_square.rank(), chessboard.side_to_move),
        (6, Colour::Red(())) | (1, Colour::White(()))
    );

    let target_two = source_square.change_rank(forward_two).filter(|sq| {
        !chessboard.occpancy_layer.get_both().is_occupied(sq)
            && target_one.is_some()
            && is_start_rank
    });

    (target_one, target_two)
}

fn generate_pawn_pushes(source_square: Position, chessboard: &Chessboard) -> Option<Action> {
    let (target_one, _target_two) = generate_pawn_targets(source_square, chessboard);

    if let Some(tgt1) = target_one {
        let action = match source_square.rank() {
            6 if chessboard.side_to_move == Colour::White(()) => Action::Promotion(Detail {
                piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                source: source_square,
                target: tgt1,
            }),
            1 if chessboard.side_to_move == Colour::Red(()) => Action::Promotion(Detail {
                piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                source: source_square,
                target: tgt1,
            }),
            _ => Action::Push(Detail {
                piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                source: source_square,
                target: tgt1,
            }),
        };
        return Some(action);
    }

    None
}

fn generate_pawn_pushes2(source_square: Position, chessboard: &Chessboard) -> Option<Action> {
    let (_target_one, target_two) = generate_pawn_targets(source_square, chessboard);

    let is_red_start = source_square.rank() == 6 && chessboard.side_to_move == Colour::Red(());
    let is_white_start = source_square.rank() == 1 && chessboard.side_to_move == Colour::White(());

    if is_red_start || is_white_start {
        if let Some(tgt2) = target_two {
            return Some(Action::Push(Detail {
                piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                source: source_square,
                target: tgt2,
            }));
        }
    }

    None
}

fn generate_enpassant(
    source_square: Position,
    chessboard: &Chessboard,
    attks: &AttackTables,
) -> Option<Action> {
    if let Some(en) = chessboard.en_passant {
        let en_attacks =
            attks.pawn_attacks[chessboard.side_to_move][source_square] & (1u64 << en as u64);

        if !en_attacks.is_empty() {
            if let Some(target) = en_attacks.get_ls1b() {
                return Some(Action::Enpassant {
                    detail: Detail {
                        piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                        source: source_square,
                        target,
                    },
                    captures: Piece::from_colour_kind(&chessboard.side_to_move.opp(), Kind::Pawn),
                });
            }
        }
    }

    None
}

fn generate_pawn_captures<'a>(
    source_square: Position,
    chessboard: &'a Chessboard,
    attks: &'a AttackTables,
) -> impl Iterator<Item = Action> + 'a {
    let targets = attks.pawn_attacks[chessboard.side_to_move][source_square]
        & chessboard.occpancy_layer.0[chessboard.side_to_move.opp()];

    targets
        .into_iter()
        .filter_map(move |target| match source_square.rank() {
            6 if chessboard.side_to_move == Colour::White(()) => {
                get_piece_at_pos(chessboard, target).map(|capture| Action::CapturePromotion {
                    detail: Detail {
                        piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                        source: source_square,
                        target,
                    },
                    captures: capture,
                })
            }
            1 if chessboard.side_to_move == Colour::Red(()) => get_piece_at_pos(chessboard, target)
                .map(|capture| Action::CapturePromotion {
                    detail: Detail {
                        piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                        source: source_square,
                        target,
                    },
                    captures: capture,
                }),
            _ => get_piece_at_pos(chessboard, target).map(|capture| Action::Capture {
                detail: Detail {
                    piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                    source: source_square,
                    target,
                },
                captures: capture,
            }),
        })
}

// === Castle moves ===
fn generate_castle_moves<'a>(
    chessboard: &'a Chessboard,
    attks: &'a AttackTables,
) -> impl Iterator<Item = Action> + 'a {
    let occ = chessboard.occpancy_layer.get_both();
    chessboard
        .castling_rights_from_bits()
        .into_iter()
        .flat_map(move |cr| {
            match cr {
                CastlingRights::WK if chessboard.side_to_move == Colour::White(()) => {
                    if !is_attacked(chessboard, Position::E1, attks)
                        && !is_attacked(chessboard, Position::G1, attks)
                        && !occ.is_occupied(Position::F1)
                        && !occ.is_occupied(Position::G1)
                    {
                        return Some(Action::Castle(CastlingRights::WK));
                    }
                    None
                }
                CastlingRights::WQ if chessboard.side_to_move == Colour::White(()) => {
                    if !is_attacked(chessboard, Position::E1, attks)
                        && !is_attacked(chessboard, Position::C1, attks)
                        && !occ.is_occupied(Position::D1)
                        && !occ.is_occupied(Position::C1)
                        && !occ.is_occupied(Position::B1)
                    {
                        return Some(Action::Castle(CastlingRights::WQ));
                    }
                    None
                }
                CastlingRights::RK if chessboard.side_to_move == Colour::Red(()) => {
                    if !is_attacked(chessboard, Position::E8, attks)
                        && !is_attacked(chessboard, Position::G8, attks)
                        && !occ.is_occupied(Position::F8)
                        && !occ.is_occupied(Position::G8)
                    {
                        return Some(Action::Castle(CastlingRights::RK));
                    }
                    None
                }
                CastlingRights::RQ if chessboard.side_to_move == Colour::Red(()) => {
                    if !is_attacked(chessboard, Position::E8, attks)
                        && !is_attacked(chessboard, Position::C8, attks)
                        && !occ.is_occupied(Position::D8)
                        && !occ.is_occupied(Position::C8)
                        && !occ.is_occupied(Position::B8)
                    {
                        return Some(Action::Castle(CastlingRights::RQ));
                    }
                    None
                }
                _ => None,
            }
        })
}

// === Knight moves ===
fn generate_knight_moves<'a>(
    board: Bitboard,
    chessboard: &'a Chessboard,
    attks: &'a AttackTables,
) -> impl Iterator<Item = Action> + 'a {
    todo!("Generate knight moves using precomputed attack tables");
    std::iter::empty()
}

// // === Bishop moves ===
// fn generate_bishop_moves<'a>(
//     board: Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate bishop sliding moves using diagonal ray attacks")
// }
//
// // === Rook moves ===
// fn generate_rook_moves<'a>(
//     board: Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate rook sliding moves using rank and file ray attacks")
// }
//
// // === Queen moves ===
// fn generate_queen_moves<'a>(
//     board: Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate queen moves by combining bishop and rook rays")
// }
//
// // === King moves (excluding castling) ===
// fn generate_king_moves<'a>(
//     board: Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate non-castling king moves using adjacent square masks")
// }
