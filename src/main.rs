use chess::{
    engine::
        movement::Move, gamestate::{
        boardstate::State,
        move_lens::History, occupancy_layer::{self, OccupancyLayer},
    }, parsers::error::Error, traits::lens::MoveLens
};

fn main() -> Result<(), Error> {
    //C27 Vienna Game: Frankenstein-Dracula
    // let board: boardstate::State = boardstate::try_from_fen(
    //     "rnbqkb1r/ppp2ppp/3p4/4p3/2B1n3/2NP4/PPP2PPP/R1BQK1NR w KQkq - 0 1",
    // )?;


    // Note that each element must be instantiated as a Move struct.
    let vienna = [
        Move(0x0011D240),
        Move(0x001931C0),
        Move(0x0012E6A0),
        Move(0x001A1950),
        Move(0x0013F620),
        Move(0x001B15A0),
        Move(0x0011CEB0),
        Move(0x00192920),
        Move(0x0012FAD0),
        Move(0x00192D30)
    ];

    let board = State::new();
    let lens = History;
    let (res, deltas) = apply_and_collect_deltas(&lens, board, vienna);

    println!("{}", board);
    for d in deltas {
        print!("{}", d);
    }
    println!("{}", res);

    Ok(())
}

fn apply_all_moves<S: Copy, M, D, L: MoveLens<S, M, D>>(
    lens: &L,
    initial: S,
    moves: impl IntoIterator<Item = M>,
) -> S {
    moves.into_iter().fold(initial, |state, mv| {
        let (new_state, _) = lens.apply_move(state, mv);
        new_state
    })
}

fn apply_and_collect_deltas<S: Copy, M, D, L: MoveLens<S, M, D>>(
    lens: &L,
    initial: S,
    moves: impl IntoIterator<Item = M>,
) -> (S, Vec<D>) {
    moves
        .into_iter()
        .fold((initial, Vec::new()), |(state, mut deltas), mv| {
            let (new_state, delta) = lens.apply_move(state, mv);
            deltas.push(delta);
            (new_state, deltas)
        })
}
