use chess::{
    board::{
        bitboard::Bitboard,
        chessboard::{current_attacks, is_attacked, Chessboard},
        pieces::{Colour, Kind, Piece},
        position::Position,
    },
    consts as CONS,
    engine::{
        attack_tables::AttackTables,
        magic_numbers::find_magic,
        move_gen::{generate_moves, generate_pawn_moves},
    },
    parsers::error::ParserError,
};

fn main() -> Result<(), ParserError> {
    let attks = AttackTables::new();

    let board: Chessboard = Chessboard::try_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPpP/R3K2R b KQkq a3 0 1")?;

    println!("{}", board);
    println!("{}", board.occpancy_layer.0[1]);
    println!("{}", board.occpancy_layer.0[0]);

    let moves = generate_pawn_moves(board.material_layer[Piece(Colour::Red(Kind::Pawn))], &board, &attks);
    for m in moves.iter() {
        println!("{:?}", m);
    }

    Ok(())
}
