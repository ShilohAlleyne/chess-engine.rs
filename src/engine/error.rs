use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Decode(String),
    Encode(String)
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Decode(s) => writeln!(f, "{}", s)?,
            Error::Encode(s) => writeln!(f, "{}", s)?,
        }

        Ok(())
    }
}


