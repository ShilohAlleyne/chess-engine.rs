use chess::{
    gamestate::boardstate as BOARDSTATE,
    parsers::error::Error,
};

fn main() -> Result<(), Error> {
    //C27 Vienna Game: Frankenstein-Dracula
    let board: BOARDSTATE::State = BOARDSTATE::try_from_fen(
        "rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1",
    )?;

    println!("{}", board);
    println!("fen_input: rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1");
    println!("board fen: {}", BOARDSTATE::to_fen(board)?);

    Ok(())
}
