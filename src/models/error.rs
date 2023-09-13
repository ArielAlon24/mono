use super::position::Position;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidSyntax(Vec<char>, Option<char>),
}

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    start: Position,
    end: Option<Position>,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, start: Position, end: Position, message: String) -> Self {
        Self {
            kind,
            start,
            end: Some(end),
            message,
        }
    }

    pub fn new_char(kind: ErrorKind, position: Position, message: String) -> Self {
        Self {
            kind,
            start: position,
            end: None,
            message,
        }
    }
}
