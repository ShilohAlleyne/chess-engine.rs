use itertools::Itertools;

use crate::{
    board::{castling, colour, pieces, position},
    engine::movement,
    gamestate::{material_layer, occupancy_layer},
    traits::lens,
};

use super::{boardstate, delta};

pub struct History;

// Lens trait for composing moves
impl lens::MoveLens<boardstate::State, movement::Move, delta::Delta> for History {
    fn apply_move(
        &self,
        state: boardstate::State,
        mv: movement::Move,
    ) -> (boardstate::State, super::delta::Delta) {
        // 0. Check peice type
        // - If king moves remove CR rights
        // - If Rook moves remove CR rights for that side
        // 1.Pop bit at source on side occ and piece material_layer
        // 2.If capture pop bit at target on opp side occ and capture piece material_layer
        // 3.Set bit at target on side occ and piece material_layer

        let piece = movement::piece(mv).expect("There will always be a piece moving");
        let source = movement::source(mv);
        let target = movement::target(mv);
        let capture = movement::capture(mv);

        // === New State ===
        let new_cr: Option<Vec<castling::Castling>> = match pieces::get_kind(&piece) {
            pieces::Kind::King => {
                Some(castling::castling_rights_from_bits(state.castling).collect())
            }
            pieces::Kind::Rook => match source {
                position::Position::H8 | position::Position::H1 => {
                    let rights = castling::castling_rights_from_bits(state.castling)
                        .filter_map(|c| castling::Castling::is_kingside(&c).map(|b| (c, b)))
                        .filter(|(_, kingside)| !*kingside)
                        .map(|(c, _)| c)
                        .collect();
                    Some(rights)
                }
                position::Position::A8 | position::Position::A1 => {
                    let rights = castling::castling_rights_from_bits(state.castling)
                        .filter_map(|c| castling::Castling::is_kingside(&c).map(|b| (c, b)))
                        .filter(|(_, kingside)| *kingside)
                        .map(|(c, _)| c)
                        .collect();
                    Some(rights)
                }
                _ => None,
            },
            _ => None,
        };

        let new_castling_rights = match new_cr{
            Some(cr) => castling::CastlingRights::from_rights(&cr),
            None => castling::CastlingRights(0),
        };

        let new_mat_layer = material_layer::MaterialLayer(match capture {
            Some(cap) => {
                let new_mat = material_layer::MaterialLayer(material_layer::move_piece(
                    state.material_layer,
                    piece,
                    source,
                    target,
                ));
                material_layer::capture_piece(new_mat, cap, target)
            }
            None => material_layer::move_piece(state.material_layer, piece, source, target),
        });

        let new_half_move = if (pieces::get_kind(&piece) == pieces::Kind::Pawn)
            || movement::traits(mv).contains(&movement::MoveTrait::Capture)
        {
            state.half_moves
        } else {
            state.half_moves + 1
        };

        let new_state = boardstate::State {
            material_layer: new_mat_layer,
            occupancy_layer: occupancy_layer::generate_occ(new_mat_layer),
            en_passant: state.en_passant, // Need to check how to set this
            side_to_move: state.side_to_move.opp(),
            castling: new_castling_rights,
            half_moves: new_half_move,
            full_moves: match state.side_to_move {
                colour::Colour::White(()) => state.full_moves,
                colour::Colour::Black(()) => state.full_moves + 1,
            },
        };

        // === Delta from new State === 
        let delta = delta::DeltaBuilder::new()
            .castling(new_castling_rights)
            .set_movement_from_move(mv)
            .build();

        (new_state, delta)
    }

    fn apply_delta(
        &self,
        state: boardstate::State,
        delta: &super::delta::Delta,
    ) -> boardstate::State {
        unimplemented!("If a move is forward, then delta is inhertly backwards")
    }

    fn invert(&self, delta: &super::delta::Delta) -> super::delta::Delta {
        unimplemented!("This might not need to exist")
    }

    fn undo(&self, state: boardstate::State, delta: &super::delta::Delta) -> boardstate::State {
        unimplemented!("Undo state using delta")
    }
}
