use crate::models::error::{Error, Runtime};
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;

macro_rules! invalid_operation {
    ($operator:expr, $right:expr, $left:expr) => {
        Err(Error::runtime(Runtime::InvalidOperation {
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
    pub fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Integer(value) => Self::Integer(value),
            TokenKind::Float(value) => Self::Float(value),
            TokenKind::Boolean(value) => Self::Boolean(value),
            _ => unreachable!(),
        }
    }

    pub fn binary_operation(self, other: Self, operator: Token) -> Operation {
        match operator.kind {
            TokenKind::Add => self.add(other, operator),
            TokenKind::Sub => self.sub(other, operator),
            TokenKind::Mul => self.mul(other, operator),
            TokenKind::Div => self.div(other, operator),
            TokenKind::Mod => self.modulo(other, operator),
            TokenKind::Pow => self.pow(other, operator),
            TokenKind::And => self.and(other, operator),
            TokenKind::Or => self.or(other, operator),
            TokenKind::Equals => self.equals(other, operator),
            TokenKind::NotEquals => self.not_equals(other, operator),
            TokenKind::Greater => self.greater(other, operator),
            TokenKind::GreaterEq => self.greater_eq(other, operator),
            TokenKind::LessThan => self.less_than(other, operator),
            TokenKind::LessThanEq => self.less_than_eq(other, operator),
            _ => unreachable!(),
        }
    }

    pub fn unary_operation(self, operator: Token) -> Operation {
        match operator.kind {
            TokenKind::Add => self.uadd(operator),
            TokenKind::Sub => self.usub(operator),
            TokenKind::Not => self.not(operator),
            _ => unreachable!(),
        }
    }

    fn add(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn uadd(self, operator: Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(a)),
            Value::Float(a) => Ok(Value::Float(a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn sub(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn usub(self, operator: Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn mul(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn div(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => {
                Err(Error::runtime(Runtime::DivisionByZero {
                    division: operator,
                }))
            }
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(_), Value::Float(b)) if b == 0.0 => {
                Err(Error::runtime(Runtime::DivisionByZero {
                    division: operator,
                }))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn modulo(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn pow(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::Integer((a as f64).powi(b as i32) as i32))
            }
            (Value::Integer(_), Value::Integer(_)) => todo!(),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn not(self, operator: Token) -> Operation {
        match self {
            Value::Boolean(a) => Ok(Value::Boolean(!a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn and(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn or(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a || b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn equals(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn not_equals(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a ^ b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn greater(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn greater_eq(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a >= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn less_than(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn less_than_eq(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }
}
