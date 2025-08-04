use chess::{
    board::{colour::Colour, pieces::{Kind, Piece}, position::Position},
    effects::static_attack_provider::StaticAttackProvider,
    engine::{
        move_gen::{generate_moves, generate_pawn_moves},
        movement::{traits, Move, MoveBuilder, MoveTrait},
    },
    gamestate::boardstate as BOARDSTATE,
    parsers::error::Error, traits,
};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    //C27 Vienna Game: Frankenstein-Dracula
    let board: BOARDSTATE::State = BOARDSTATE::try_from_fen(
        "rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1",
    )?;

    println!("{}", board);
    // println!("{}", board.occpancy_layer.0[1]);
    // println!("{}", board.occpancy_layer.0[0]);

    // let lookup = StaticAttackProvider;
    // let moves = generate_moves(&board, lookup)
    //     .filter(|m| traits(*m).contains(&MoveTrait::Check));
    // for m in moves {
    //     print!("{}", m);
    // }
    //
    
    println!("fen_input: rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1");
    println!("board fen: {}", BOARDSTATE::to_fen(board)?);

    Ok(())
}
