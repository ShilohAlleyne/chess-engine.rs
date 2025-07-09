mod attack_tables;
mod bitboard;
mod position;

use attack_tables::AttackTables;
use bitboard::Bitboard;
use position::Position;


fn main() {
    let attacks = AttackTables::new();
    if let Some(mask) = attacks.knight.white {
        let occupancy = Bitboard::new()
            .set_occupancy(4095, &mask[Position::D4 as usize]);
        println!("{}", occupancy);
    };
}
