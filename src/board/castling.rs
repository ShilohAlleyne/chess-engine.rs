use std::fmt;

// Castling bit binaary representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CastlingRights {
    None,
    WK,
    WQ,
    RK,
    RQ,
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastlingRights::None => writeln!(f, "None"),
            CastlingRights::WK => write!(f, "White King side"),
            CastlingRights::WQ => write!(f, "White Queen side"),
            CastlingRights::RK => write!(f, "Red King side"),
            CastlingRights::RQ => write!(f, "Red Queen side"),
        }
    }
}

impl CastlingRights {
    pub fn get_castlings_bits(&self) -> u8 {
        match self {
            CastlingRights::None => 0,
            CastlingRights::WK => 1,
            CastlingRights::WQ => 2,
            CastlingRights::RK => 4,
            CastlingRights::RQ => 8,
        }
    }

    // Option bool egh my eyes!
    pub(crate) fn is_kingside(&self) -> Option<bool> {
        match self {
            CastlingRights::WK => Some(true),
            CastlingRights::WQ => Some(false),
            CastlingRights::RK => Some(true),
            CastlingRights::RQ => Some(false),
            CastlingRights::None => None,
        }
    }
}
