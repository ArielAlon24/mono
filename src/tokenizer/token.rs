use crate::models::position::Position;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenKind {
    Identifier(String),

    // Keywords
    None,
    Not,
    And,
    Or,
    Let,
    If,
    Else,
    While,
    Def,

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
    Comma,
    NewLine,
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl TokenKind {
    pub fn from_str(identifier: &str) -> Option<Self> {
        match identifier {
            "True" => Some(Self::Boolean(true)),
            "False" => Some(Self::Boolean(false)),
            "None" => Some(Self::None),
            "not" => Some(Self::Not),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "let" => Some(Self::Let),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "while" => Some(Self::While),
            "def" => Some(Self::Def),
            _ => None,
        }
    }

    pub fn to_kind(&self) -> String {
        match self {
            Self::Identifier(_) => String::from("Identifier"),
            Self::Character(_) => String::from("Character"),
            Self::String(_) => String::from("String"),
            Self::Integer(_) => String::from("Integer"),
            Self::Float(_) => String::from("Float"),
            Self::Boolean(_) => String::from("Boolean"),
            _ => format!("{:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
