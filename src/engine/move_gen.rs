use crate::{
    board::{
        bitboard as BITBOARD, castling as CR, colour as COLOUR, pieces as PIECES,
        position as POSITION,
    },
    engine::movement as MOVE,
    gamestate::{boardstate as BOARDSTATE, occupancy_layer as OCCUPANCY},
    traits::static_lookup as PRECOMP,
};
use itertools::chain;

pub fn generate_moves<A>(
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
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
        generate_major_piece_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Knight)],
            chessboard,
            lookup,
            PIECES::Kind::Knight,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Rook)],
            chessboard,
            lookup,
            PIECES::Kind::Rook,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Bishop)],
            chessboard,
            lookup,
            PIECES::Kind::Bishop,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Queen)],
            chessboard,
            lookup,
            PIECES::Kind::Queen,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::King)],
            chessboard,
            lookup,
            PIECES::Kind::King,
        ),
    )
}

// === Individual piece move gen ===
pub fn generate_pawn_moves<A>(
    board: BITBOARD::Bitboard,
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    board.flat_map(move |source_square| {
        chain!(
            generate_pawn_pushes(source_square, chessboard),
            generate_pawn_pushes2(source_square, chessboard),
            generate_pawn_captures(source_square, chessboard, lookup),
            generate_enpassant(source_square, chessboard, lookup),
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
) -> Option<MOVE::Move> {
    let (target_one, _target_two) = generate_pawn_targets(source_square, chessboard);

    let is_promotion_rank = match chessboard.side_to_move {
        COLOUR::Colour::White(()) => source_square.rank() == 6,
        COLOUR::Colour::Red(()) => source_square.rank() == 1,
    };

    if let Some(tgt1) = target_one {
        let mv = MOVE::MoveBuilder::new()
            .set_piece(PIECES::Piece::from_colour_kind(
                &chessboard.side_to_move,
                PIECES::Kind::Pawn,
            ))
            .set_source(source_square)
            .set_target(tgt1)
            .set_traits(if is_promotion_rank {
                &[MOVE::MoveTrait::Promotion, MOVE::MoveTrait::Quiet]
            } else {
                &[MOVE::MoveTrait::Quiet]
            })
            .build();
        return Some(mv);
    }

    None
}

fn generate_pawn_pushes2(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
) -> Option<MOVE::Move> {
    let (_target_one, target_two) = generate_pawn_targets(source_square, chessboard);

    let is_red_start =
        source_square.rank() == 6 && chessboard.side_to_move == COLOUR::Colour::Red(());
    let is_white_start =
        source_square.rank() == 1 && chessboard.side_to_move == COLOUR::Colour::White(());

    if is_red_start || is_white_start {
        if let Some(tgt2) = target_two {
            return Some(
                MOVE::MoveBuilder::new()
                    .set_traits(&[MOVE::MoveTrait::Quiet])
                    .set_piece(PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ))
                    .set_source(source_square)
                    .set_target(tgt2)
                    .build(),
            );
        }
    }

    None
}

fn generate_enpassant<A: PRECOMP::StaticAttack + Copy>(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> Option<MOVE::Move> {
    if let Some(en) = chessboard.en_passant {
        let en_attacks = lookup.pawn(source_square, chessboard.side_to_move) & (1u64 << en as u64);

        if !en_attacks.is_empty() {
            if let Some(target) = en_attacks.get_ls1b() {
                let detail = MOVE::Detail {
                    piece: PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ),
                    source: source_square,
                    target,
                };

                let mv = MOVE::MoveBuilder::new()
                    .set_traits(if into_check(&detail, chessboard, lookup) {
                        // The mythical en_passant check!
                        &[MOVE::MoveTrait::Enpassant, MOVE::MoveTrait::Check]
                    } else {
                        &[MOVE::MoveTrait::Enpassant]
                    })
                    .set_piece(PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        PIECES::Kind::Pawn,
                    ))
                    .set_source(source_square)
                    .set_target(target)
                    .captures(PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move.opp(),
                        PIECES::Kind::Pawn,
                    ))
                    .build();

                return Some(mv);
            }
        }
    }

    None
}

fn generate_pawn_captures<A>(
    source_square: POSITION::Position,
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    let targets = lookup.pawn(source_square, chessboard.side_to_move)
        & chessboard.occpancy_layer.0[chessboard.side_to_move.opp()];

    targets.into_iter().filter_map(move |target| {
        let piece = PIECES::Piece::from_colour_kind(&chessboard.side_to_move, PIECES::Kind::Pawn);
        let checks = into_check(
            &MOVE::Detail {
                piece,
                source: source_square,
                target,
            },
            chessboard,
            lookup,
        );

        BOARDSTATE::get_piece_at_pos(chessboard, target).map(|capture| {
            MOVE::MoveBuilder::new()
                .set_traits(
                    if (chessboard.side_to_move == COLOUR::Colour::White(())
                        && source_square.rank() == 6)
                        || (chessboard.side_to_move == COLOUR::Colour::Red(())
                            && source_square.rank() == 1)
                    {
                        &[MOVE::MoveTrait::Capture, MOVE::MoveTrait::Promotion]
                    } else if checks {
                        &[MOVE::MoveTrait::Capture, MOVE::MoveTrait::Check]
                    } else {
                        &[MOVE::MoveTrait::Capture]
                    },
                )
                .set_piece(piece)
                .set_source(source_square)
                .set_target(target)
                .captures(capture)
                .build()
        })
    })
}

// === Castle moves ===
// Add castling into check
pub fn generate_castle_moves<A>(
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
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
                    return Some((POSITION::Position::E1, POSITION::Position::G1));
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
                    return Some((POSITION::Position::E1, POSITION::Position::C1));
                }
                None
            }
            CR::CastlingRights::RK if chessboard.side_to_move == COLOUR::Colour::Red(()) => {
                if !BOARDSTATE::is_attacked(chessboard, POSITION::Position::E8, lookup)
                    && !BOARDSTATE::is_attacked(chessboard, POSITION::Position::G8, lookup)
                    && !occ.is_occupied(POSITION::Position::F8)
                    && !occ.is_occupied(POSITION::Position::G8)
                {
                    return Some((POSITION::Position::E8, POSITION::Position::G8));
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
                    return Some((POSITION::Position::E8, POSITION::Position::C8));
                }
                None
            }
            _ => None,
        })
        .map(|(king_from, king_to)| {
            MOVE::MoveBuilder::new()
                .set_traits(&[MOVE::MoveTrait::Castle])
                .set_source(king_from)
                .set_target(king_to)
                .build()
        })
}

// === Major piece moves ===
fn generate_major_piece_moves<A>(
    board: BITBOARD::Bitboard,
    chessboard: &BOARDSTATE::State,
    lookup: A,
    piece: PIECES::Kind,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    board.into_iter().flat_map(move |source_square| {
        // Lookup relavent attacks
        let raw_attacks = match piece {
            PIECES::Kind::Bishop => lookup.bishop(
                source_square,
                OCCUPANCY::get_both(&chessboard.occpancy_layer),
            ),
            PIECES::Kind::Knight => lookup.knight(source_square),
            PIECES::Kind::Rook => lookup.rook(
                source_square,
                OCCUPANCY::get_both(&chessboard.occpancy_layer),
            ),
            PIECES::Kind::Queen => lookup.queen(
                source_square,
                OCCUPANCY::get_both(&chessboard.occpancy_layer),
            ),
            PIECES::Kind::King => lookup.king(source_square),
            // Minor piece
            PIECES::Kind::Pawn => unreachable!(),
        };
        // Init attacks
        let attacks = raw_attacks & !chessboard.occpancy_layer[chessboard.side_to_move];

        attacks.into_iter().map(move |trgt| {
            let detail = MOVE::Detail {
                piece: PIECES::Piece::from_colour_kind(&chessboard.side_to_move, piece),
                source: source_square,
                target: trgt,
            };

            if let Some(capture) = BOARDSTATE::get_piece_at_pos(chessboard, trgt) {
                return MOVE::MoveBuilder::new()
                    .set_traits(if into_check(&detail, chessboard, lookup) {
                        &[MOVE::MoveTrait::Check, MOVE::MoveTrait::Capture]
                    } else {
                        &[MOVE::MoveTrait::Capture]
                    })
                    .set_piece(PIECES::Piece::from_colour_kind(
                        &chessboard.side_to_move,
                        piece,
                    ))
                    .set_source(source_square)
                    .set_target(trgt)
                    .captures(capture)
                    .build();
            }
            // Check if we move into check or just make a quite move
            MOVE::MoveBuilder::new()
                .set_traits(if into_check(&detail, chessboard, lookup) {
                    &[MOVE::MoveTrait::Check]
                } else {
                    &[MOVE::MoveTrait::Quiet]
                })
                .set_piece(PIECES::Piece::from_colour_kind(
                    &chessboard.side_to_move,
                    piece,
                ))
                .set_source(source_square)
                .set_target(trgt)
                .build()
        })
    })
}

fn into_check<A: PRECOMP::StaticAttack>(
    detail: &MOVE::Detail,
    chessboard: &BOARDSTATE::State,
    lookup: A,
) -> bool {
    let raw_attacks = match PIECES::get_kind(&detail.piece) {
        PIECES::Kind::Bishop => lookup.bishop(
            detail.target,
            OCCUPANCY::get_both(&chessboard.occpancy_layer),
        ),
        PIECES::Kind::Knight => lookup.knight(detail.target),
        PIECES::Kind::Rook => lookup.rook(
            detail.target,
            OCCUPANCY::get_both(&chessboard.occpancy_layer),
        ),
        PIECES::Kind::Queen => lookup.queen(
            detail.target,
            OCCUPANCY::get_both(&chessboard.occpancy_layer),
        ),
        PIECES::Kind::King => lookup.king(detail.target),
        PIECES::Kind::Pawn => lookup.pawn(detail.target, chessboard.side_to_move),
    };

    // We Not here as we want to know if there IS a king attack
    let king_bb = chessboard.material_layer
        [PIECES::Piece::from_colour_kind(&chessboard.side_to_move.opp(), PIECES::Kind::King)];

    !(raw_attacks & king_bb).is_empty()
}
