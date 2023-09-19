use crate::models::position::Position;

#[derive(Debug, PartialEq)]
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
    Float(f32),
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
    start: Position,
    end: Option<Position>,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(start: Position, end: Option<Position>, kind: TokenKind) -> Self {
        Self { start, end, kind }
    }
}
