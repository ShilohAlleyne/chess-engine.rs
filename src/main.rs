use chess::{
    board::{
        chessboard::{is_attacked, Chessboard},
        pieces::{Colour, Kind, Piece}, position::Position,
    },
    engine::{
        attack_tables::AttackTables,
        move_gen::{generate_moves, generate_pawn_moves},
    },
    parsers::error::ParserError,
};

fn main() -> Result<(), ParserError> {
    let attks = AttackTables::new();

    let board: Chessboard = Chessboard::try_from_fen(
        "r3k2r/p1ppqp2/bn2pnp1/3PN3/Pp2P3/2b2Q1p/1PB1BPpP/R3K2R b KQkq - 0 1",
    )?;

    println!("{}", board);
    println!("{}", board.occpancy_layer.0[1]);
    println!("{}", board.occpancy_layer.0[0]);

    let moves = generate_moves(
        &board,
        &attks,
    );
    for m in moves {
        println!("{:?}", m);
    }

    Ok(())
}
