use std::{iter::Peekable, str::Chars};

use crate::{
    bitboard::Bitboard,
    chessboard::Chessboard,
    material_layer::MaterialLayer,
    pieces::{Colour, Piece},
    position::{CastlingRights, Position},
};

#[derive(Debug)]
pub(crate) enum Token {
    Material(Piece),
    EmptySquares(u32),
    ActiveColour(Colour<()>),
    Castling(CastlingRights),
    Enpassant(Option<Position>),
    HalfMove(u32),
    FullMove(u32),
    NextRank,
    NextRegion,
    Err(char),
}

#[derive(Debug)]
pub(crate) enum ParserError {
    InvalidCharacter(char)
}

#[derive(Debug, Default)]
enum ParserRegion {
    #[default]
    Boardstate,
    ActiveColour,
    CastlingRights,
    Enpassant,
    HalfMove,
    FullMove,
}

impl ParserRegion {
    pub fn advance(&mut self) {
        *self = match self {
            ParserRegion::Boardstate => ParserRegion::ActiveColour,
            ParserRegion::ActiveColour => ParserRegion::CastlingRights,
            ParserRegion::CastlingRights => ParserRegion::Enpassant,
            ParserRegion::Enpassant => ParserRegion::HalfMove,
            ParserRegion::HalfMove => ParserRegion::FullMove,
            ParserRegion::FullMove => ParserRegion::FullMove, // or wrap to Boardstate if cyclic
        };
    }
}

// This will need to be a result
pub(crate) fn generate_board(input: &str) -> Result<Chessboard, ParserError> {
    // Init chessboard
    let mut board: Chessboard = Chessboard::new();
    board.material_layer = MaterialLayer([Bitboard::new(); 12]);

    // Init x & y postions
    // The file counter gets reset each encountered
    // NextRank token
    let mut file: u32 = 0;
    // Increaments with each encountered NextRank token
    let mut rank: u32 = 0;

    let tokens: Vec<Token> = parse(input);

    // === Loop over tokens and configure chessboard ===
    for token in tokens {
        match token {
            Token::Material(p) => {
                board.material_layer[p].mutate_set_bit(rank * 8 + file);
                file += 1;
            }
            Token::EmptySquares(sq) => file += sq,
            Token::ActiveColour(colour) => board.side_to_move = colour,
            Token::Castling(cr) => board.add_castling_right(cr),
            Token::Enpassant(en) => board.en_passant = en,
            Token::HalfMove(count) => board.half_moves = count,
            Token::FullMove(count) => board.full_moves = count,
            Token::NextRank => {
                rank += 1;
                file = 0;
            }
            Token::NextRegion => {}
            Token::Err(c) => return Err(ParserError::InvalidCharacter(c))
        }
    }

    Ok(board)
}

pub(crate) fn parse(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut region = ParserRegion::default();
    let mut tokens = Vec::new();

    while let Some(character) = chars.next() {
        let token = match region {
            ParserRegion::Boardstate => match character {
                '0'..='8' => Token::EmptySquares(character.to_digit(10).unwrap_or_default()),
                '/' => Token::NextRank,
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => match Piece::try_from(&character) {
                    Ok(p) => Token::Material(p),
                    Err(_) => Token::Err(character),
                },
            },

            ParserRegion::ActiveColour => match character {
                'w' => Token::ActiveColour(Colour::White(())),
                'b' => Token::ActiveColour(Colour::Red(())),
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(character),
            },

            ParserRegion::CastlingRights => match character {
                'K' => Token::Castling(CastlingRights::WK),
                'Q' => Token::Castling(CastlingRights::WQ),
                'k' => Token::Castling(CastlingRights::RK),
                'q' => Token::Castling(CastlingRights::RQ),
                '-' => {
                    Token::Castling(CastlingRights::None)
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(character),
            },

            ParserRegion::Enpassant => match character {
                '-' => {
                    Token::Enpassant(None)
                }
                'a'..='h' => {
                    let Some(next_char) = chars.peek().copied() else {
                        return vec![Token::Err(character)];
                    };

                    if let '1'..='8' = next_char {
                        chars.next();
                        let pos = Position::new(character, next_char);
                        match pos {
                            Some(p) => Token::Enpassant(Some(p)),
                            None => Token::Err(character),
                        }
                    } else {
                        Token::Err(character)
                    }
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(character),
            },

            ParserRegion::HalfMove => match character {
                '0'..='9' => {
                    let value = parse_number(character, &mut chars);
                    Token::HalfMove(value)
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(character),
            },

            ParserRegion::FullMove => match character {
                '0'..='9' => {
                    let value = parse_number(character, &mut chars);
                    Token::FullMove(value)
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(character),
            },
        };

        println!("{:?} | {:?}", &region, &token);
        tokens.push(token);
    }

    tokens
}

fn parse_number(start: char, chars: &mut Peekable<Chars>) -> u32 {
    let mut num = String::new();
    num.push(start);
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num.push(c);
            chars.next();
        } else {
            break;
        }
    }
    num.parse().unwrap_or(0)
}
