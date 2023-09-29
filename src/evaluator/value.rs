use crate::models::error::Error;
use crate::models::error::InvalidSyntax;
use crate::tokenizer::token::TokenKind;

macro_rules! invalid_operation {
    ($operator:expr, $right:expr, $left:expr) => {
        Err(Error::invalid_syntax(InvalidSyntax::InvalidOperation {
            operator: $operator,
            right: $right,
            left: $left,
        }))
    };
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

type Operation = Result<Value, Error>;

impl Value {
    pub fn add(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (right, left) => invalid_operation!(TokenKind::Add, Some(right), left),
        }
    }

    pub fn uadd(self) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(a)),
            Value::Float(a) => Ok(Value::Float(a)),
            left => invalid_operation!(TokenKind::Add, None, left),
        }
    }

    pub fn sub(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (right, left) => invalid_operation!(TokenKind::Sub, Some(right), left),
        }
    }

    pub fn usub(self) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            left => invalid_operation!(TokenKind::Sub, None, left),
        }
    }

    pub fn mul(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (right, left) => invalid_operation!(TokenKind::Mul, Some(right), left),
        }
    }

    pub fn div(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => todo!(),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(_), Value::Float(b)) if b == 0.0 => todo!(),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (right, left) => invalid_operation!(TokenKind::Div, Some(right), left),
        }
    }

    pub fn modulo(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (right, left) => invalid_operation!(TokenKind::Mod, Some(right), left),
        }
    }

    pub fn pow(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::Integer((a as f64).powi(b as i32) as i32))
            }
            (Value::Integer(_), Value::Integer(_)) => todo!(),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            (right, left) => invalid_operation!(TokenKind::Pow, Some(right), left),
        }
    }

    pub fn not(self) -> Operation {
        match self {
            Value::Boolean(a) => Ok(Value::Boolean(!a)),
            left => invalid_operation!(TokenKind::Not, None, left),
        }
    }

    pub fn and(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            (right, left) => invalid_operation!(TokenKind::And, Some(right), left),
        }
    }

    pub fn or(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a || b)),
            (right, left) => invalid_operation!(TokenKind::Or, Some(right), left),
        }
    }

    pub fn equals(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (right, left) => invalid_operation!(TokenKind::Equals, Some(right), left),
        }
    }

    pub fn not_equals(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a ^ b)),
            (right, left) => invalid_operation!(TokenKind::NotEquals, Some(right), left),
        }
    }

    pub fn greater(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (right, left) => invalid_operation!(TokenKind::Greater, Some(right), left),
        }
    }

    pub fn greater_eq(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a >= b)),
            (right, left) => invalid_operation!(TokenKind::GreaterEq, Some(right), left),
        }
    }

    pub fn less_than(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (right, left) => invalid_operation!(TokenKind::LessThan, Some(right), left),
        }
    }

    pub fn less_than_eq(self, other: Self) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (right, left) => invalid_operation!(TokenKind::LessThanEq, Some(right), left),
        }
    }
}
