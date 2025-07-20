use super::{attack_tables::AttackTables, movement::Action};
use crate::{
    board::{
        bitboard::Bitboard,
        chessboard::{get_piece_at_pos, Chessboard},
        pieces::{Colour, Kind, Piece},
    },
    consts as CONTS,
    engine::movement::Detail,
};

pub fn generate_moves(board: &Chessboard, attks: &AttackTables) -> Vec<Action> {
    let mut moves = Vec::new();
    for (i, bitboard) in board.material_layer.0.iter().enumerate() {
        let bb = *bitboard;

        let move_fn = CONTS::MOVE_GENERATORS[i];
        moves.extend(move_fn(bb, board, attks));
    }

    moves
}

// === Indivdual piece move gen ===
pub fn generate_pawn_moves(
    mut board: Bitboard,
    chessboard: &Chessboard,
    attks: &AttackTables,
) -> Vec<Action> {
    // Init our list of valid moves
    let mut moves: Vec<Action> = Vec::new();

    while !board.is_empty() {
        if let Some(source_square) = board.get_ls1b() {
            let (forward_one, forward_two) = match chessboard.side_to_move {
                Colour::Red(()) => (-1, -2),
                Colour::White(()) => (1, 2),
            };

            // One-square push (must be valid and unoccupied)
            let target_one = source_square
                .change_rank(forward_one)
                .filter(|sq| !chessboard.occpancy_layer.get_both().is_occupied(sq));

            // Two-square push (from starting rank only, must be valid and unoccupied)
            // Pawns can't jump so target 1 must also be valid
            let target_two = source_square.change_rank(forward_two).filter(|sq| {
                !chessboard.occpancy_layer.get_both().is_occupied(sq) && target_one.is_some()
            });

            // Check and emit two-square push
            let is_red_start =
                source_square.rank() == 6 && chessboard.side_to_move == Colour::Red(());
            let is_white_start =
                source_square.rank() == 1 && chessboard.side_to_move == Colour::White(());

            if is_red_start || is_white_start {
                if let Some(tgt2) = target_two {
                    moves.push(Action::Push(Detail {
                        piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                        source: source_square,
                        target: tgt2,
                    }));
                }
            }

            // Handle one-square push and promotions
            if let Some(tgt1) = target_one {
                match source_square.rank() {
                    6 if chessboard.side_to_move == Colour::White(()) => {
                        moves.push(Action::Promotion(Detail {
                            piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                            source: source_square,
                            target: tgt1,
                        }));
                    }
                    1 if chessboard.side_to_move == Colour::Red(()) => {
                        moves.push(Action::Promotion(Detail {
                            piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                            source: source_square,
                            target: tgt1,
                        }));
                    }
                    _ => {
                        moves.push(Action::Push(Detail {
                            piece: Piece::from_colour_kind(&chessboard.side_to_move, Kind::Pawn),
                            source: source_square,
                            target: tgt1,
                        }));
                    }
                }
            }

            // === Capturing Moves ===
            let mut pawn_attks = attks.pawn_attacks[chessboard.side_to_move][source_square]
                & chessboard.occpancy_layer.0[chessboard.side_to_move.opp()];

            while !pawn_attks.is_empty() {
                if let Some(target) = pawn_attks.get_ls1b() {
                    // Pawn promotion capture
                    match source_square.rank() {
                        6 if chessboard.side_to_move == Colour::White(()) => {
                            if let Some(capture) = get_piece_at_pos(chessboard, target) {
                                moves.push(Action::CapturePromotion {
                                    detail: Detail {
                                        piece: Piece::from_colour_kind(
                                            &chessboard.side_to_move,
                                            Kind::Pawn,
                                        ),
                                        source: source_square,
                                        target,
                                    },
                                    captures: capture,
                                });
                            }
                        }
                        1 if chessboard.side_to_move == Colour::Red(()) => {
                            if let Some(capture) = get_piece_at_pos(chessboard, target) {
                                moves.push(Action::CapturePromotion {
                                    detail: Detail {
                                        piece: Piece::from_colour_kind(
                                            &chessboard.side_to_move,
                                            Kind::Pawn,
                                        ),
                                        source: source_square,
                                        target,
                                    },
                                    captures: capture,
                                });
                            }
                        }
                        // Regular capture
                        _ => {
                            if let Some(capture) = get_piece_at_pos(chessboard, target) {
                                moves.push(Action::Capture {
                                    detail: Detail {
                                        piece: Piece::from_colour_kind(
                                            &chessboard.side_to_move,
                                            Kind::Pawn,
                                        ),
                                        source: source_square,
                                        target,
                                    },
                                    captures: capture,
                                });
                            }
                        }
                    }

                    pawn_attks.mutate_pop_bit(target);
                }
            }

            //=== En passant ===
            if let Some(en) = chessboard.en_passant {
                let en_attacks = attks.pawn_attacks[chessboard.side_to_move][source_square]
                    & (1u64 << en as u64);

                if !en_attacks.is_empty() {
                    if let Some(target) = en_attacks.get_ls1b() {
                        moves.push(Action::Enpassant {
                            detail: Detail {
                                piece: Piece::from_colour_kind(
                                    &chessboard.side_to_move,
                                    Kind::Pawn,
                                ),
                                source: source_square,
                                target,
                            },
                            captures: Piece::from_colour_kind(
                                &chessboard.side_to_move.opp(),
                                Kind::Pawn,
                            ),
                        });
                    }
                }
            }

            board.mutate_pop_bit(source_square);
        }
    }

    moves
}
