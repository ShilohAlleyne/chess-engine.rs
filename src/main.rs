use chess::{
    board::{bitboard::Bitboard, position::Position},
    engine::{attack_tables::AttackTables, magic_numbers::find_magic},
    parsers::error::ParserError,
    consts as CONS,
};
use strum::IntoEnumIterator;

fn main() -> Result<(), ParserError> {
    let attks = AttackTables::new();

    let occ = Bitboard::new()
        .set_bit(Position::B6)
        .set_bit(Position::D6)
        .set_bit(Position::F6);

    println!("{}", occ);
    println!("{}", attks.get_queen_attacks(Position::D4, occ));

    Ok(())
}
