use crate::{
    board::{
        bitboard, castling, colour, pieces,
        position,
    },
    engine::movement as MOVE,
    gamestate::{boardstate, occupancy_layer},
    traits::static_lookup as PRECOMP,
};
use itertools::chain;

pub fn generate_moves<A>(
    chessboard: &boardstate::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    // Compose our lazy eval generated moves together
    chain!(
        generate_pawn_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Pawn)],
            chessboard,
            lookup,
        ),
        generate_castle_moves(chessboard, lookup),
        generate_major_piece_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Knight)],
            chessboard,
            lookup,
            pieces::Kind::Knight,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Rook)],
            chessboard,
            lookup,
            pieces::Kind::Rook,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Bishop)],
            chessboard,
            lookup,
            pieces::Kind::Bishop,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Queen)],
            chessboard,
            lookup,
            pieces::Kind::Queen,
        ),
        generate_major_piece_moves(
            chessboard.material_layer
                [pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::King)],
            chessboard,
            lookup,
            pieces::Kind::King,
        ),
    )
}

// === Individual piece move gen ===
pub fn generate_pawn_moves<A>(
    board: bitboard::Bitboard,
    chessboard: &boardstate::State,
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
    source_square: position::Position,
    chessboard: &boardstate::State,
) -> (Option<position::Position>, Option<position::Position>) {
    let (forward_one, forward_two) = match chessboard.side_to_move {
        colour::Colour::Black(()) => (-1, -2),
        colour::Colour::White(()) => (1, 2),
    };

    let target_one = source_square
        .change_rank(forward_one)
        .filter(|sq| !occupancy_layer::get_both(&chessboard.occupancy_layer).is_occupied(sq));

    let is_start_rank = matches!(
        (source_square.rank(), chessboard.side_to_move),
        (6, colour::Colour::Black(())) | (1, colour::Colour::White(()))
    );

    let target_two = source_square.change_rank(forward_two).filter(|sq| {
        !occupancy_layer::get_both(&chessboard.occupancy_layer).is_occupied(sq)
            && target_one.is_some()
            && is_start_rank
    });

    (target_one, target_two)
}

fn generate_pawn_pushes(
    source_square: position::Position,
    chessboard: &boardstate::State,
) -> Option<MOVE::Move> {
    let (target_one, _target_two) = generate_pawn_targets(source_square, chessboard);

    let is_promotion_rank = match chessboard.side_to_move {
        colour::Colour::White(()) => source_square.rank() == 6,
        colour::Colour::Black(()) => source_square.rank() == 1,
    };

    if let Some(tgt1) = target_one {
        let mv = MOVE::MoveBuilder::new()
            .set_piece(pieces::from_colour_kind(
                &chessboard.side_to_move,
                pieces::Kind::Pawn,
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
    source_square: position::Position,
    chessboard: &boardstate::State,
) -> Option<MOVE::Move> {
    let (_target_one, target_two) = generate_pawn_targets(source_square, chessboard);

    let is_red_start =
        source_square.rank() == 6 && chessboard.side_to_move == colour::Colour::Black(());
    let is_white_start =
        source_square.rank() == 1 && chessboard.side_to_move == colour::Colour::White(());

    if is_red_start || is_white_start {
        if let Some(tgt2) = target_two {
            return Some(
                MOVE::MoveBuilder::new()
                    .set_traits(&[MOVE::MoveTrait::Quiet])
                    .set_piece(pieces::from_colour_kind(
                        &chessboard.side_to_move,
                        pieces::Kind::Pawn,
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
    source_square: position::Position,
    chessboard: &boardstate::State,
    lookup: A,
) -> Option<MOVE::Move> {
    if let Some(en) = chessboard.en_passant {
        let en_attacks = lookup.pawn(source_square, chessboard.side_to_move) & (1u64 << en as u64);

        if !en_attacks.is_empty() {
            if let Some(target) = en_attacks.get_ls1b() {
                let detail = MOVE::Detail {
                    piece: pieces::from_colour_kind(
                        &chessboard.side_to_move,
                        pieces::Kind::Pawn,
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
                    .set_piece(pieces::from_colour_kind(
                        &chessboard.side_to_move,
                        pieces::Kind::Pawn,
                    ))
                    .set_source(source_square)
                    .set_target(target)
                    .captures(pieces::from_colour_kind(
                        &chessboard.side_to_move.opp(),
                        pieces::Kind::Pawn,
                    ))
                    .build();

                return Some(mv);
            }
        }
    }

    None
}

fn generate_pawn_captures<A>(
    source_square: position::Position,
    chessboard: &boardstate::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    let targets = lookup.pawn(source_square, chessboard.side_to_move)
        & chessboard.occupancy_layer.0[chessboard.side_to_move.opp()];

    targets.into_iter().filter_map(move |target| {
        let piece = pieces::from_colour_kind(&chessboard.side_to_move, pieces::Kind::Pawn);
        let checks = into_check(
            &MOVE::Detail {
                piece,
                source: source_square,
                target,
            },
            chessboard,
            lookup,
        );

        boardstate::get_piece_at_pos(chessboard, target).map(|capture| {
            MOVE::MoveBuilder::new()
                .set_traits(
                    if (chessboard.side_to_move == colour::Colour::White(())
                        && source_square.rank() == 6)
                        || (chessboard.side_to_move == colour::Colour::Black(())
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
    chessboard: &boardstate::State,
    lookup: A,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    let occ = occupancy_layer::get_both(&chessboard.occupancy_layer);
    castling::castling_rights_from_bits(chessboard.castling)
        .flat_map(move |cr| match cr {
            castling::Castling::WK if chessboard.side_to_move == colour::Colour::White(()) => {
                if !boardstate::is_attacked(chessboard, position::Position::E1, lookup)
                    && !boardstate::is_attacked(chessboard, position::Position::G1, lookup)
                    && !occ.is_occupied(position::Position::F1)
                    && !occ.is_occupied(position::Position::G1)
                {
                    return Some((position::Position::E1, position::Position::G1));
                }
                None
            }
            castling::Castling::WQ if chessboard.side_to_move == colour::Colour::White(()) => {
                if !boardstate::is_attacked(chessboard, position::Position::E1, lookup)
                    && !boardstate::is_attacked(chessboard, position::Position::C1, lookup)
                    && !occ.is_occupied(position::Position::D1)
                    && !occ.is_occupied(position::Position::C1)
                    && !occ.is_occupied(position::Position::B1)
                {
                    return Some((position::Position::E1, position::Position::C1));
                }
                None
            }
            castling::Castling::RK if chessboard.side_to_move == colour::Colour::Black(()) => {
                if !boardstate::is_attacked(chessboard, position::Position::E8, lookup)
                    && !boardstate::is_attacked(chessboard, position::Position::G8, lookup)
                    && !occ.is_occupied(position::Position::F8)
                    && !occ.is_occupied(position::Position::G8)
                {
                    return Some((position::Position::E8, position::Position::G8));
                }
                None
            }
            castling::Castling::RQ if chessboard.side_to_move == colour::Colour::Black(()) => {
                if !boardstate::is_attacked(chessboard, position::Position::E8, lookup)
                    && !boardstate::is_attacked(chessboard, position::Position::C8, lookup)
                    && !occ.is_occupied(position::Position::D8)
                    && !occ.is_occupied(position::Position::C8)
                    && !occ.is_occupied(position::Position::B8)
                {
                    return Some((position::Position::E8, position::Position::C8));
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
    board: bitboard::Bitboard,
    chessboard: &boardstate::State,
    lookup: A,
    piece: pieces::Kind,
) -> impl Iterator<Item = MOVE::Move> + '_
where
    A: PRECOMP::StaticAttack + Copy + 'static,
{
    board.into_iter().flat_map(move |source_square| {
        // Lookup relavent attacks
        let raw_attacks = match piece {
            pieces::Kind::Bishop => lookup.bishop(
                source_square,
                occupancy_layer::get_both(&chessboard.occupancy_layer),
            ),
            pieces::Kind::Knight => lookup.knight(source_square),
            pieces::Kind::Rook => lookup.rook(
                source_square,
                occupancy_layer::get_both(&chessboard.occupancy_layer),
            ),
            pieces::Kind::Queen => lookup.queen(
                source_square,
                occupancy_layer::get_both(&chessboard.occupancy_layer),
            ),
            pieces::Kind::King => lookup.king(source_square),
            // Minor piece
            pieces::Kind::Pawn => unreachable!(),
        };
        // Init attacks
        let attacks = raw_attacks & !chessboard.occupancy_layer[chessboard.side_to_move];

        attacks.into_iter().map(move |trgt| {
            let detail = MOVE::Detail {
                piece: pieces::from_colour_kind(&chessboard.side_to_move, piece),
                source: source_square,
                target: trgt,
            };

            if let Some(capture) = boardstate::get_piece_at_pos(chessboard, trgt) {
                return MOVE::MoveBuilder::new()
                    .set_traits(if into_check(&detail, chessboard, lookup) {
                        &[MOVE::MoveTrait::Check, MOVE::MoveTrait::Capture]
                    } else {
                        &[MOVE::MoveTrait::Capture]
                    })
                    .set_piece(pieces::from_colour_kind(
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
                .set_piece(pieces::from_colour_kind(
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
    chessboard: &boardstate::State,
    lookup: A,
) -> bool {
    let raw_attacks = match pieces::get_kind(&detail.piece) {
        pieces::Kind::Bishop => lookup.bishop(
            detail.target,
            occupancy_layer::get_both(&chessboard.occupancy_layer),
        ),
        pieces::Kind::Knight => lookup.knight(detail.target),
        pieces::Kind::Rook => lookup.rook(
            detail.target,
            occupancy_layer::get_both(&chessboard.occupancy_layer),
        ),
        pieces::Kind::Queen => lookup.queen(
            detail.target,
            occupancy_layer::get_both(&chessboard.occupancy_layer),
        ),
        pieces::Kind::King => lookup.king(detail.target),
        pieces::Kind::Pawn => lookup.pawn(detail.target, chessboard.side_to_move),
    };

    // We Not here as we want to know if there IS a king attack
    let king_bb = chessboard.material_layer
        [pieces::from_colour_kind(&chessboard.side_to_move.opp(), pieces::Kind::King)];

    !(raw_attacks & king_bb).is_empty()
}
