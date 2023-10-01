use crate::models::position::Position;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum TokenKind {
    Identifier(String),

    // Keywords
    None,
    Not,
    And,
    Or,

    // Builtin types
    Character(char),
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),

    // Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Assignment,
    Equals,
    NotEquals,
    Greater,
    GreaterEq,
    LessThan,
    LessThanEq,

    // Brackets
    RightParen,
    LeftParen,
    RightCurly,
    LeftCurly,
    RightBracket,
    LeftBracket,

    // Arrows
    Arrow,
    DoubleArrow,

    // Other
    NewLine,
}

impl TokenKind {
    pub fn str_to_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "true" => Some(Self::Boolean(true)),
            "false" => Some(Self::Boolean(false)),
            "none" => Some(Self::None),
            "not" => Some(Self::Not),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub start: Position,
    pub end: Option<Position>,
    pub kind: TokenKind,
}

impl Token {
    pub const COMPERATORS: [TokenKind; 6] = [
        TokenKind::Equals,
        TokenKind::NotEquals,
        TokenKind::Greater,
        TokenKind::GreaterEq,
        TokenKind::LessThan,
        TokenKind::LessThanEq,
    ];

    pub fn new(start: Position, end: Option<Position>, kind: TokenKind) -> Self {
        Self { start, end, kind }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.end {
            Some(end) => write!(f, "<{}:{} {:?}>", self.start, end, self.kind),
            None => write!(f, "<{} {:?}>", self.start, self.kind),
        }
    }
}
