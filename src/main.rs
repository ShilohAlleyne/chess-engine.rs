use chess::{
    board::{colour::Colour, pieces::{Kind, Piece}, position::Position},
    effects::static_attack_provider::StaticAttackProvider,
    engine::{
        move_gen::{generate_moves, generate_pawn_moves},
        movement::{Move, MoveBuilder, MoveTrait},
    },
    gamestate::boardstate as BOARDSTATE,
    parsers::error::ParserError,
};

fn main() -> Result<(), ParserError> {
    //C27 Vienna Game: Frankenstein-Dracula
    let board: BOARDSTATE::State = BOARDSTATE::State::try_from_fen(
        "rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1",
    )?;

    println!("{}", board);
    // println!("{}", board.occpancy_layer.0[1]);
    // println!("{}", board.occpancy_layer.0[0]);

    let lookup = StaticAttackProvider;
    let moves = generate_moves(&board, lookup);
    for m in moves {
        print!("{}", m);
    }

    Ok(())
}
