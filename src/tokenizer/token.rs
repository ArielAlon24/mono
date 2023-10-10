use crate::models::position::Position;
use std::fmt;

/*
--- TokenKind (enum) ---

The TokenKind enum contains every kind of token that can be
fount inside Mono code.
Note: builtin and identifier types include an attribute
attached to them.
*/
#[derive(Debug, PartialEq, Clone)]
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
    /*
    The from_str method takes an identifier str and checks if it
    is a builtin keyword. If it is, it returns the corresponding
    keyword wrapped in Some option. Otherwise it returns None.
    */
    pub fn from_str(identifier: &str) -> Option<Self> {
        match identifier {
            "True" => Some(Self::Boolean(true)),
            "False" => Some(Self::Boolean(false)),
            "None" => Some(Self::None),
            "not" => Some(Self::Not),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            _ => None,
        }
    }
}

/*
--- Token (struct) ---

The Token struct holds all information regarding a token,
his kind, start and end position. If the token is of length
1 (a single char) his end is marked as None. otherwise with
the position wrapped in Some option.
*/
#[derive(Debug, PartialEq)]
pub struct Token {
    pub start: Position,
    pub end: Option<Position>,
    pub kind: TokenKind,
}

impl Token {
    // COMPERTAORS is an array of all comparison token kinds
    pub const COMPERATORS: [TokenKind; 6] = [
        TokenKind::Equals,
        TokenKind::NotEquals,
        TokenKind::Greater,
        TokenKind::GreaterEq,
        TokenKind::LessThan,
        TokenKind::LessThanEq,
    ];

    /*
    The new function is a constructor for a Token, it takes the
    start and end positions and the token kind. Then returns
    a Token struct made out of these arguments.
    */
    pub fn new(start: Position, end: Option<Position>, kind: TokenKind) -> Self {
        Self { start, end, kind }
    }
}

impl fmt::Display for Token {
    /*
    The fmt (toString) method takes the default formatter and
    formats the self Token for a more readable and user
    friendly representation.
    */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.end {
            Some(end) => write!(f, "<{}:{} {:?}>", self.start, end, self.kind),
            None => write!(f, "<{} {:?}>", self.start, self.kind),
        }
    }
}
