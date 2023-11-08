use super::position::Position;
use crate::evaluator::value::Value;
use crate::tokenizer::token::{Token, TokenKind};
use std::fmt;

pub trait MonoError: fmt::Display {
    fn kind(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub enum Syntax {
    InvalidIntegerSize {
        start: Position,
        end: Position,
    },
    InvalidFloatSize {
        start: Position,
        end: Position,
    },
    UnclosedCharDelimeter {
        start: Position,
        end: Position,
        found: Option<char>,
    },
    UnclosedStringDelimeter {
        start: Position,
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
    UnexpectedToken {
        token: Token,
        expected: Vec<TokenKind>,
    },
    UnexpectedEOF,
    MultipleExpressions {
        position: Position,
    },
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIntegerSize {
                start,
                end,
            } => write!(f, "Invalid integer size at {} until {}. Integer i must be in the range of {} >= i >= {}.", start, end, i32::MAX, i32::MIN),
            Self::InvalidFloatSize {
                start,
                end,
            } => write!(f, "Invalid float size at {} until {}. Float f must be in the range of {} >= f >= {}.", start, end, f32::MAX, f32::MIN),
            Self::UnclosedCharDelimeter {
                start,
                end,
                found: Some(c),
            } => {
                write!(f, "Encountered unclosed Character delimiter `'`. Character deceleration starts at {} and expected closing delimiter at {} but found `{}`.", start, end, c)
            }
            Self::UnclosedCharDelimeter {
                start,
                end: _,
                found: None,
            } => {
                write!(f, "Encountered unclosed Character delimiter `'`. Character deceleration starts at {} but a closing delimiter was not found.", start)
            }
            Self::UnclosedStringDelimeter { start } => {
                write!(f, "Encountered unclosed String delimiter `\"`. String deceleration starts at {} but a closing delimiter was not found.", start)
            }
            Self::UnclosedTokenDelimeter {
                start,
                found: Some(token),
                delimiter,
            } => {
                write!(f, "Encountered unclosed token delimiter '{:?}' that starts at {}. Expected matching closing token but found `{}` at {}.", delimiter, start.start, token, token.start)
            }
            Self::UnclosedTokenDelimeter {
                start,
                found: None,
                delimiter,
            } => {
                write!(f, "Encountered unclosed token delimiter '{:?}' that starts at {}. No matching closing token was found.", delimiter, start.start)
            }
            Self::UnexpectedChar { position, c } => {
                write!(f, "Encountered unexpected character '{}' at position {}. Please check your input.", c, position)
            }
            Self::MultipleFloatingPoints { start, end } => {
                write!(f, "Multiple floating points detected between {} and {}. A number can only contain one decimal point.", start, end)
            }
            Self::UnrecognizedChar { position, c } => {
                write!(f, "Encountered unrecognized character '{}' at position {}. Ensure your input only contains valid characters.", c, position)
            }
            Self::UnexpectedToken { token, expected } => {
                write!(
                    f,
                    "Encountered unexpected token `{:?}` at position {}, expected one of the following: {}.",
                    token.kind, 
                    token.start, 
                    expected.iter()
                            .map(|kind| kind.to_kind())
                            .collect::<Vec<_>>()
                            .join(", ")
                )
            }
            Self::UnexpectedEOF => {
                write!(
                    f,
                    "Unexpected end of input. The expression might be incomplete."
                )
            }
            Self::MultipleExpressions { position } => {
                write!(f, "Detected multiple expressions at {}. Ensure you're providing a single, valid expression.", position)
            }
        }
    }
}

impl MonoError for Syntax {
    fn kind(&self) -> &str {
        "SyntaxError"
    }
}

#[derive(Debug, PartialEq)]
pub enum Runtime {
    DivisionByZero {
        division: Token,
    },
    InvalidOperation {
        operator: Token,
        right: Option<Value>,
        left: Value,
    },
    UnknownIdentifier {
        identifier: Token,
    },
    IncorrectParameters {
        name: String,
        call: Token,
        expected: Vec<String>,
        found: Vec<Value>,
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero { division } => {
                write!(f, "Division by zero at position {}.", division.start)
            }
            Self::InvalidOperation {
                operator,
                right,
                left,
            } => {
                if let Some(right) = right {
                    write!(f, "Invalid binary operation detected. Operator `{:?}` was used with left value `{:?}` and right value `{:?}`.", operator.kind, left, right)
                } else {
                    write!(f, "Invalid unary operation detected. Operator `{:?}` was used with value `{:?}`.", operator.kind, left)
                }
            }
            Self::UnknownIdentifier { identifier } => {
                write!(f, "Unknown identifier `{}` detected.", identifier)
            }
            Self::IncorrectParameters { expected, found, name, call } => {
                write!(f, "Incorrect parameters: ({}) for function '{}' at {}, expected: ({}).", 
                    found.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", "),
                    name,
                    call.start,
                    expected.iter().map(|p| format!("{}", p)).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}

impl MonoError for Runtime {
    fn kind(&self) -> &str {
        "RuntimeError"
    }
}
