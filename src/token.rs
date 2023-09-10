#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Kind {
    Identifier(String),

    // Keywords
    True,
    False,
    None,
    And,
    Or,

    // Builtin types
    Character(char),
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),

    // Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Power,
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

    // Arrows
    Arrow,
    DoubleArrow,

    // Other
    NewLine,
}

impl Kind {
    pub fn str_to_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "true" => Some(Self::Boolean(true)),
            "false" => Some(Self::Boolean(false)),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: Kind,
    row: usize,
    column: usize,
}

impl Token {
    pub fn new(kind: Kind, row: usize, column: usize) -> Self {
        return Self { kind, row, column };
    }
}
