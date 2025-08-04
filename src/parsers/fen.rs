use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use itertools::Itertools;

use crate::{
    board::castling as CR, board::colour as COLOUR, board::pieces as PIECE,
    board::position as POSITION, gamestate::boardstate as BOARDSTATE,
};

#[derive(Debug)]
enum Token {
    Material(PIECE::Piece),
    EmptySquares(u32),
    ActiveColour(COLOUR::Colour<()>),
    Castling(CR::CastlingRights),
    Enpassant(Option<POSITION::Position>),
    HalfMove(u32),
    FullMove(u32),
    NextRank,
    NextRegion,
    Err(usize, char),
}

#[derive(Debug, Default)]
enum Region {
    #[default]
    Boardstate,
    ActiveColour,
    CastlingRights,
    Enpassant,
    HalfMove,
    FullMove,
}

impl Region {
    pub fn advance(&mut self) {
        *self = match self {
            Region::Boardstate => Region::ActiveColour,
            Region::ActiveColour => Region::CastlingRights,
            Region::CastlingRights => Region::Enpassant,
            Region::Enpassant => Region::HalfMove,
            Region::HalfMove => Region::FullMove,
            Region::FullMove => Region::FullMove, // or wrap to Boardstate if cyclic
        };
    }
}

// This will need to be a result
pub(crate) fn parse(input: &str) -> Result<BOARDSTATE::State, crate::parsers::error::Error> {
    // Init chessboard
    let mut board: BOARDSTATE::State = BOARDSTATE::State::default();

    // Init x & y postions
    // The file counter gets reset each encountered
    // NextRank token
    let mut file: u32 = 0;
    // Increaments with each encountered NextRank token
    let mut rank: u32 = 0;

    let tokens: Vec<Token> = tokenize(input);

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
            Token::Err(i, c) => {
                return Err(crate::parsers::error::Error::Deserialization {
                    input: input.to_owned(),
                    invalid_char: c,
                    pos: i,
                })
            }
        }
    }

    // Set a matching occpancy_layer layer
    for (i, bb) in board.material_layer.0.iter().enumerate() {
        if i < 6 {
            board.occpancy_layer.0[0] |= *bb
        } else if i > 5 {
            board.occpancy_layer.0[1] |= *bb
        }
    }

    Ok(board)
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().enumerate().peekable();
    let mut region = Region::default();
    let mut tokens = Vec::new();

    while let Some((i, character)) = chars.next() {
        let token = match region {
            Region::Boardstate => match character {
                '0'..='8' => Token::EmptySquares(character.to_digit(10).unwrap_or_default()),
                '/' => Token::NextRank,
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => match PIECE::Piece::try_from(&character) {
                    Ok(p) => Token::Material(p),
                    Err(_) => Token::Err(i, character),
                },
            },

            Region::ActiveColour => match character {
                'w' => Token::ActiveColour(COLOUR::Colour::White(())),
                'b' => Token::ActiveColour(COLOUR::Colour::Red(())),
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(i, character),
            },

            Region::CastlingRights => match character {
                'K' => Token::Castling(CR::CastlingRights::WK),
                'Q' => Token::Castling(CR::CastlingRights::WQ),
                'k' => Token::Castling(CR::CastlingRights::RK),
                'q' => Token::Castling(CR::CastlingRights::RQ),
                '-' => Token::Castling(CR::CastlingRights::None),
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(i, character),
            },

            Region::Enpassant => match character {
                '-' => Token::Enpassant(None),
                'a'..='h' => {
                    let Some(next_char) = chars.peek().copied() else {
                        return vec![Token::Err(i, character)];
                    };

                    if let '1'..='8' = next_char.1 {
                        chars.next();
                        let pos = POSITION::Position::from_chars(character, next_char.1);
                        match pos {
                            Some(p) => Token::Enpassant(Some(p)),
                            None => Token::Err(i, character),
                        }
                    } else {
                        Token::Err(i, character)
                    }
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(i, character),
            },

            Region::HalfMove => match character {
                '0'..='9' => {
                    let value = parse_number(character, &mut chars);
                    Token::HalfMove(value)
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(i, character),
            },

            Region::FullMove => match character {
                '0'..='9' => {
                    let value = parse_number(character, &mut chars);
                    Token::FullMove(value)
                }
                ' ' => {
                    region.advance();
                    Token::NextRegion
                }
                _ => Token::Err(i, character),
            },
        };

        tokens.push(token);
    }

    tokens
}

fn parse_number(start: char, chars: &mut Peekable<Enumerate<Chars>>) -> u32 {
    let mut num = String::new();
    num.push(start);
    while let Some(&(_, c)) = chars.peek() {
        if c.is_ascii_digit() {
            num.push(c);
            chars.next();
        } else {
            break;
        }
    }
    num.parse().unwrap_or(0)
}

// serialize state to fen
pub fn serialize(state: BOARDSTATE::State) -> Result<String, crate::parsers::error::Error> {
    let rows = [0..8, 8..16, 16..24, 24..32, 32..40, 40..48, 48..56, 56..64];

    let fen_rows = rows
        .into_iter()
        .map(|row| {
            // Get piece at pos
            row.map(|sq| -> Result<Option<PIECE::Piece>, super::error::Error> {
                let pos = POSITION::Position::from_u32(sq as u32).ok_or_else(|| {
                    super::error::Error::Serialization(
                        "Invalid boardstate for serialization".to_owned(),
                    )
                })?;
                Ok(BOARDSTATE::get_piece_at_pos(&state, pos))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            // Convert peice to char and space to placeholder '1'
            .map(|piece| match piece {
                Some(p) => char::from(p),
                None => '1',
            })
            // Fold our row chars into fen row string
            .try_fold(
                (String::new(), 0),
                |(mut fen, space), c| -> Result<(String, u32), super::error::Error> {
                    match c {
                        '1' => Ok((fen, space + 1)),
                        _p => {
                            if space > 0 {
                                let spc = char::from_u32('0' as u32 + space).ok_or_else(|| {
                                    super::error::Error::Serialization(
                                        "Invalid boardstate for serialization".to_owned(),
                                    )
                                })?;
                                fen.push(spc);
                            }
                            fen.push(_p);
                            Ok((fen, 0))
                        }
                    }
                },
            )
            // Handle trailing spaces
            .and_then(|(mut fen, space)| -> Result<String, super::error::Error> {
                if space > 0 {
                    let char = char::from_u32('0' as u32 + space).ok_or_else(|| {
                        super::error::Error::Serialization(
                            "Invalid boardstate for serialization".to_owned(),
                        )
                    })?;
                    fen.push(char);
                }

                Ok(fen)
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let side_to_move = match state.side_to_move {
        COLOUR::Colour::White(()) => "w",
        COLOUR::Colour::Red(()) => "b",
    }
    .to_owned();

    let castling = BOARDSTATE::castling_rights_from_bits(&state)
        .map(|cr| match cr {
            CR::CastlingRights::None => "-",
            CR::CastlingRights::WK => "K",
            CR::CastlingRights::WQ => "Q",
            CR::CastlingRights::RK => "k",
            CR::CastlingRights::RQ => "q",
        })
        .join("");

    let enpassant = match state.en_passant {
        Some(e) => POSITION::to_string(e),
        None => "-".to_owned(),
    };

    Ok([
        fen_rows.join("/"),
        side_to_move,
        castling,
        enpassant,
        format!("{}", state.half_moves),
        format!("{}", state.full_moves),
    ]
    .join(" "))
}
