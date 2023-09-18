use super::position::Position;
use crate::models::token::Token;
use crate::models::token::TokenKind;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSyntax(InvalidSyntax),
}

#[derive(Debug, PartialEq)]
pub enum InvalidSyntax {
    UnclosedCharDelimeter(Position, Position, char, Option<char>),
    UnclosedTokenDelimeter(Token, Option<Token>, TokenKind),
    UnexpectedChar(Position, char),
    MultipleFloatingPoints(Position, Position),
    UnrecognizedChar(Position, char),
    InvalidToken {
        expected: Vec<TokenKind>,
        found: Option<Token>,
    },
    MultipleExpressions,
}

impl Error {
    pub fn invalid_syntax(kind: InvalidSyntax) -> Self {
        Error::InvalidSyntax(kind)
    }
}
