use itertools::chain;

use super::movement::Action;
use crate::{
    board::{
        bitboard as BITBOARD, castling as CR, colour as COLOUR, pieces as PIECES,
        position as POSITION,
    },
    gamestate::{boardstate as BOARDSTATE, occupancy_layer as OCCUPANCY},
    engine::lazy_statics as STATIC,
    engine::movement as MOVE,
    traits::static_lookup as PRECOMP,
};

pub fn generate_moves<'a, A>(
    chessboard: &'a BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = Action> + 'a
where
    A: PRECOMP::StaticAttack + Copy + 'a,
{
    // Compose our lazy eval generated moves together
    chain!(
        generate_pawn_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Pawn)],
            chessboard,
            lookup,
        ),
        generate_castle_moves(chessboard, lookup),
    )
}

// === Individual piece move gen ===
pub fn generate_pawn_moves<'a, A>(
    board: BITBOARD::Bitboard,
    chessboard: &'a BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = Action> + 'a
where
    A: PRECOMP::StaticAttack + Copy + 'a,
{
    board.flat_map(move |source_square| {
        chain!(
            generate_pawn_pushes(source_square, chessboard),
            generate_pawn_pushes2(source_square, chessboard),
            generate_pawn_captures(source_square, chessboard),
            generate_enpassant(source_square, chessboard, lookup)
        )
        .flatten()
    })
}

fn generate_pawn_targets(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
) -> (Option<POSITION::Position>, Option<POSITION::Position>) {
    let (forward_one, forward_two) = match chessboard.side_to_move {
        COLOUR::Colour::Red(()) => (-1, -2),
        COLOUR::Colour::White(()) => (1, 2),
    };

    let target_one = source_square
        .change_rank(forward_one)
        .filter(|sq| !OCCUPANCY::get_both(&chessboard.occpancy_layer).is_occupied(sq));

    let is_start_rank = matches!(
        (source_square.rank(), chessboard.side_to_move),
        (6, COLOUR::Colour::Red(())) | (1, COLOUR::Colour::White(()))
    );

    let target_two = source_square.change_rank(forward_two).filter(|sq| {
        !OCCUPANCY::get_both(&chessboard.occpancy_layer).is_occupied(sq)
            && target_one.is_some()
            && is_start_rank
    });

    (target_one, target_two)
}

fn generate_pawn_pushes(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
) -> Option<Action> {
    let (target_one, _target_two) = generate_pawn_targets(source_square, chessboard);

    if let Some(tgt1) = target_one {
        let action = match source_square.rank() {
            6 if chessboard.side_to_move == COLOUR::Colour::White(()) => {
                Action::Promotion(MOVE::Detail {
                    piece: PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ),
                    source: source_square,
                    target: tgt1,
                })
            }
            1 if chessboard.side_to_move == COLOUR::Colour::Red(()) => {
                Action::Promotion(MOVE::Detail {
                    piece: PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ),
                    source: source_square,
                    target: tgt1,
                })
            }
            _ => Action::Push(MOVE::Detail {
                piece: PIECES::Piece::from_colour_kind(
                    &chessboard.side_to_move,
                    PIECES::Kind::Pawn,
                ),
                source: source_square,
                target: tgt1,
            }),
        };
        return Some(action);
    }

    None
}

fn generate_pawn_pushes2(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
) -> Option<Action> {
    let (_target_one, target_two) = generate_pawn_targets(source_square, chessboard);

    let is_red_start =
        source_square.rank() == 6 && chessboard.side_to_move == COLOUR::Colour::Red(());
    let is_white_start =
        source_square.rank() == 1 && chessboard.side_to_move == COLOUR::Colour::White(());

    if is_red_start || is_white_start {
        if let Some(tgt2) = target_two {
            return Some(Action::Push(MOVE::Detail {
                piece: PIECES::Piece::from_colour_kind(
                    &chessboard.side_to_move,
                    PIECES::Kind::Pawn,
                ),
                source: source_square,
                target: tgt2,
            }));
        }
    }

    None
}

fn generate_enpassant<A: PRECOMP::StaticAttack>(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> Option<Action> {
    if let Some(en) = chessboard.en_passant {
        let en_attacks =
            lookup.pawn(source_square, chessboard.side_to_move) & (1u64 << en as u64);

        if !en_attacks.is_empty() {
            if let Some(target) = en_attacks.get_ls1b() {
                return Some(Action::Enpassant {
                    detail: MOVE::Detail {
                        piece: PIECES::Piece::from_colour_kind(
                            &chessboard.side_to_move,
                            PIECES::Kind::Pawn,
                        ),
                        source: source_square,
                        target,
                    },
                    captures: PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move.opp(),
                        PIECES::Kind::Pawn,
                    ),
                });
            }
        }
    }

    None
}

fn generate_pawn_captures(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
) -> impl Iterator<Item = Action> + '_ {
    let targets = STATIC::PAWN_ATTACKS[chessboard.side_to_move][source_square]
        & chessboard.occpancy_layer.0[chessboard.side_to_move.opp()];

    targets
        .into_iter()
        .filter_map(move |target| match source_square.rank() {
            6 if chessboard.side_to_move == COLOUR::Colour::White(()) => {
                BOARDSTATE::get_piece_at_pos(chessboard, target).map(|capture| Action::CapturePromotion {
                    detail: MOVE::Detail {
                        piece: PIECES::Piece::from_colour_kind(
                            &chessboard.side_to_move,
                            PIECES::Kind::Pawn,
                        ),
                        source: source_square,
                        target,
                    },
                    captures: capture,
                })
            }
            1 if chessboard.side_to_move == COLOUR::Colour::Red(()) => {
                BOARDSTATE::get_piece_at_pos(chessboard, target).map(|capture| Action::CapturePromotion {
                    detail: MOVE::Detail {
                        piece: PIECES::Piece::from_colour_kind(
                            &chessboard.side_to_move,
                            PIECES::Kind::Pawn,
                        ),
                        source: source_square,
                        target,
                    },
                    captures: capture,
                })
            }
            _ => BOARDSTATE::get_piece_at_pos(chessboard, target).map(|capture| Action::Capture {
                detail: MOVE::Detail {
                    piece: PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ),
                    source: source_square,
                    target,
                },
                captures: capture,
            }),
        })
}

// === Castle moves ===
pub fn generate_castle_moves<'a, A>(
    chessboard: &'a BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = Action> + 'a
where
    A: PRECOMP::StaticAttack + Copy + 'a,
{
    let occ = OCCUPANCY::get_both(&chessboard.occpancy_layer);
    BOARDSTATE::castling_rights_from_bits(chessboard)
        .flat_map(move |cr| match cr {
            CR::CastlingRights::WK if chessboard.side_to_move == COLOUR::Colour::White(()) => {
                if !BOARDSTATE::is_attacked(chessboard, POSITION::Position::E1, lookup)
                    && !BOARDSTATE::is_attacked(chessboard, POSITION::Position::G1, lookup)
                    && !occ.is_occupied(POSITION::Position::F1)
                    && !occ.is_occupied(POSITION::Position::G1)
                {
                    return Some(Action::Castle(CR::CastlingRights::WK));
                }
                None
            }
            CR::CastlingRights::WQ if chessboard.side_to_move == COLOUR::Colour::White(()) => {
                if !BOARDSTATE::is_attacked(chessboard, POSITION::Position::E1, lookup)
                    && !BOARDSTATE::is_attacked(chessboard, POSITION::Position::C1, lookup)
                    && !occ.is_occupied(POSITION::Position::D1)
                    && !occ.is_occupied(POSITION::Position::C1)
                    && !occ.is_occupied(POSITION::Position::B1)
                {
                    return Some(Action::Castle(CR::CastlingRights::WQ));
                }
                None
            }
            CR::CastlingRights::RK if chessboard.side_to_move == COLOUR::Colour::Red(()) => {
                if !BOARDSTATE::is_attacked(chessboard, POSITION::Position::E8, lookup)
                    && !BOARDSTATE::is_attacked(chessboard, POSITION::Position::G8, lookup)
                    && !occ.is_occupied(POSITION::Position::F8)
                    && !occ.is_occupied(POSITION::Position::G8)
                {
                    return Some(Action::Castle(CR::CastlingRights::RK));
                }
                None
            }
            CR::CastlingRights::RQ if chessboard.side_to_move == COLOUR::Colour::Red(()) => {
                if !BOARDSTATE::is_attacked(chessboard, POSITION::Position::E8, lookup)
                    && !BOARDSTATE::is_attacked(chessboard, POSITION::Position::C8, lookup)
                    && !occ.is_occupied(POSITION::Position::D8)
                    && !occ.is_occupied(POSITION::Position::C8)
                    && !occ.is_occupied(POSITION::Position::B8)
                {
                    return Some(Action::Castle(CR::CastlingRights::RQ));
                }
                None
            }
            _ => None,
        })
}

// === Knight moves ===
fn generate_knight_moves(
    board: BITBOARD::Bitboard,
    chessboard: &BOARDSTATE::State,
) -> impl Iterator<Item = Action> + '_ {
    todo!("Generate knight moves using precomputed attack tables");
    std::iter::empty()
}

// // === Bishop moves ===
// fn generate_bishop_moves<'a>(
//     board: BITBOARD::Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate bishop sliding moves using diagonal ray attacks")
// }
//
// // === Rook moves ===
// fn generate_rook_moves<'a>(
//     board: BITBOARD::Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate rook sliding moves using rank and file ray attacks")
// }
//
// // === Queen moves ===
// fn generate_queen_moves<'a>(
//     board: BITBOARD::Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate queen moves by combining bishop and rook rays")
// }
//
// // === King moves (excluding castling) ===
// fn generate_king_moves<'a>(
//     board: BITBOARD::Bitboard,
//     chessboard: &'a Chessboard,
//     attks: &'a AttackTables,
// ) -> impl Iterator<Item = Action> + 'a {
//     todo!("Generate non-castling king moves using adjacent square masks")
// }
