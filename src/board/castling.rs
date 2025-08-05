use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub fn new() -> Self {
        Self(0)
    }

    // === Castling Rights bitmask operations ===
    pub fn set_castling_rights(&mut self, rights: &[Castling]) {
        self.0 = rights.iter().fold(0, |acc, r| acc | r.get_castlings_bits())
    }

    pub fn add_castling_right(&mut self, right: Castling) {
        self.0 |= right.get_castlings_bits();
    }

    pub fn toggle_castling_rights(&mut self, right: Castling) {
        self.0 ^= right.get_castlings_bits()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self::new()
    }
}

pub fn castling_rights_from_bits(cr: CastlingRights) -> impl Iterator<Item = Castling> {
    [
        Castling::WK,
        Castling::WQ,
        Castling::RK,
        Castling::RQ,
    ]
    .iter()
    .copied()
    .filter(move |r| cr.0 & r.get_castlings_bits() != 0)
}

// Castling bit binary representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castling {
    None,
    WK,
    WQ,
    RK,
    RQ,
}

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Castling::None => writeln!(f, "None"),
            Castling::WK => write!(f, "White King side"),
            Castling::WQ => write!(f, "White Queen side"),
            Castling::RK => write!(f, "Red King side"),
            Castling::RQ => write!(f, "Red Queen side"),
        }
    }
}

impl Castling {
    pub fn get_castlings_bits(&self) -> u8 {
        match self {
            Castling::None => 0,
            Castling::WK => 1,
            Castling::WQ => 2,
            Castling::RK => 4,
            Castling::RQ => 8,
        }
    }

    // Option bool egh my eyes!
    pub(crate) fn is_kingside(&self) -> Option<bool> {
        match self {
            Castling::WK => Some(true),
            Castling::WQ => Some(false),
            Castling::RK => Some(true),
            Castling::RQ => Some(false),
            Castling::None => None,
        }
    }
}
