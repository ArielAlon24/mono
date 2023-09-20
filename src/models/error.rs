use super::position::Position;
use crate::tokenizer::token::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSyntax(InvalidSyntax),
}

#[derive(Debug, PartialEq)]
pub enum InvalidSyntax {
    UnclosedCharDelimeter {
        start: Position,
        end: Position,
        delimiter: char,
        found: Option<char>,
    },
    UnclosedTokenDelimeter {
        start: Token,
        found: Option<Token>,
        delimiter: TokenKind,
    },
    UnexpectedChar {
        position: Position,
        c: char,
    },
    MultipleFloatingPoints {
        start: Position,
        end: Position,
    },
    UnrecognizedChar {
        position: Position,
        c: char,
    },
    InvalidToken {
        expected: Vec<TokenKind>,
        found: Option<Token>,
    },
    MultipleExpressions {
        position: Position,
    },
}

impl Error {
    pub fn invalid_syntax(kind: InvalidSyntax) -> Self {
        Error::InvalidSyntax(kind)
    }
}
