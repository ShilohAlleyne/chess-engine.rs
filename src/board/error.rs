#[derive(Debug)]
pub enum Error {
    Serilzation(String),
    Deserialization(String),
    TypeCoversiton(String),
} 
