mod attack_tables;
mod bitboard;
mod position;
mod consts;
mod xor_rand;
mod pieces;
mod chessboard;
mod material_layer;
mod fen_parser;


use attack_tables::AttackTables;
use chessboard::Chessboard;
use fen_parser::{generate_board, ParserError};
use position::CastlingRights;

fn main() -> Result<(), ParserError>{

    // New display trait
    let board = generate_board("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9")?;

    println!("{}", board);

    Ok(())
}
