use chess::{
    engine::{
        move_gen::{generate_moves, generate_pawn_moves}
    },
    gamestate::boardstate as BOARDSTATE,
    parsers::error::ParserError,
    effects::static_attack_provider::StaticAttackProvider,
};

fn main() -> Result<(), ParserError> {
    let board: BOARDSTATE::State = BOARDSTATE::State::try_from_fen(
        "r3k2r/p1ppqp2/bn2pnp1/3PN3/Pp2P3/2b2Q1p/1PB1BPpP/R3K2R b KQkq - 0 1",
    )?;

    println!("{}", board);
    // println!("{}", board.occpancy_layer.0[1]);
    // println!("{}", board.occpancy_layer.0[0]);

    let lookup = StaticAttackProvider;
    let moves = generate_moves(&board, lookup);
    for m in moves {
        println!("{:?}", m);
    }

    Ok(())
}
