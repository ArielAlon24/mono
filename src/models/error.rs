use super::position::Position;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    UnrecognizedChar(char),
    InvalidSyntax(Vec<char>, Option<char>),
    UnexpectedChar(char),
    UnclosedDelimeter(char),
}

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    start: Position,
    end: Option<Position>,
}

impl Error {
    pub fn new(kind: ErrorKind, start: Position, end: Position) -> Self {
        Self {
            kind,
            start,
            end: Some(end),
        }
    }

    pub fn new_char(kind: ErrorKind, position: Position) -> Self {
        Self {
            kind,
            start: position,
            end: None,
        }
    }
}
