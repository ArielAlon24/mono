use crate::models::error::MonoError;
use crate::models::error::Runtime;
use crate::parser::node::Node;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;
use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;

macro_rules! invalid_operation {
    ($operator:expr, $right:expr, $left:expr) => {
        Err(Box::new(Runtime::InvalidOperation {
            operator: $operator.clone(),
            right: $right,
            left: $left,
        }))
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
    Character(char),
    List(Rc<RefCell<Vec<Value>>>),
    Function {
        name: String,
        arguments: Vec<String>,
        body: Box<Node>,
    },
    BuiltInFunction {
        name: String,
        arguments: Vec<String>,
        function: fn(Vec<Value>) -> Value,
    },
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(value) => write!(f, "{value}"),
            Value::Float(value) => write!(f, "{value}"),
            Value::Boolean(true) => write!(f, "True"),
            Value::Boolean(false) => write!(f, "False"),
            Value::String(value) => write!(f, "{value}"),
            Value::Character(value) => write!(f, "{value}"),
            Value::List(list) => {
                let format = list
                    .borrow()
                    .iter()
                    .map(|value| format!("{}", value))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{format}]")
            }
            Value::Function { name, .. } => write!(f, "<Function: {}>", name),
            Value::BuiltInFunction { name, .. } => write!(f, "<Function: {}>", name),
            Value::None => write!(f, "None"),
        }
    }
}

type Operation = Result<Value, Box<dyn MonoError>>;

impl Value {
    pub fn binary_operation(self, other: Self, operator: &Token) -> Operation {
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

    pub fn unary_operation(self, operator: &Token) -> Operation {
        match operator.kind {
            TokenKind::Add => self.pos(operator),
            TokenKind::Sub => self.neg(operator),
            TokenKind::Not => self.not(operator),
            _ => unreachable!(),
        }
    }

    pub fn index(self, index: Self) -> Operation {
        match (self, index) {
            (Value::String(string), Value::Integer(i)) => {
                if i >= 0 && i < string.len() as i32 {
                    Ok(Value::Character(string.chars().nth(i as usize).unwrap()))
                } else {
                    todo!()
                }
            }
            (Value::List(list), Value::Integer(i)) => {
                let borrow_list = list.borrow();
                if i >= 0 && i < borrow_list.len() as i32 {
                    Ok(borrow_list[i as usize].clone())
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    pub fn list_assign(self, index: Self, value: Self) -> Operation {
        match (self, index) {
            (Value::List(list), Value::Integer(i)) => {
                let mut mut_list = list.borrow_mut();
                if i >= 0 && i < mut_list.len() as i32 {
                    mut_list[i as usize] = value;
                    Ok(Value::None)
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn add(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
            (Value::Character(a), Value::Character(b)) => Ok(Value::String(format!("{a}{b}"))),
            (Value::String(a), Value::Character(b)) => Ok(Value::String(format!("{a}{b}"))),
            (Value::Character(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn pos(self, operator: &Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(a)),
            Value::Float(a) => Ok(Value::Float(a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn sub(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn neg(self, operator: &Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn mul(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::String(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::String(a.repeat(b as usize)))
            }
            (Value::String(_), Value::Integer(b)) if b < 0 => todo!(),
            (Value::Character(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::String(a.to_string().repeat(b as usize)))
            }
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn div(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => Err(Box::new(Runtime::DivisionByZero {
                division: operator.clone(),
            })),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(_), Value::Float(b)) if b == 0.0 => {
                Err(Box::new(Runtime::DivisionByZero {
                    division: operator.clone(),
                }))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn modulo(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn pow(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::Integer((a as f64).powi(b as i32) as i32))
            }
            (Value::Integer(_), Value::Integer(_)) => todo!(),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f32).powf(b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(b as f32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn not(self, operator: &Token) -> Operation {
        match self {
            Value::Boolean(a) => Ok(Value::Boolean(!a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    fn and(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn or(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a || b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn equals(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a == b)),
            (Value::None, Value::None) => Ok(Value::Boolean(true)),
            (_, Value::None) | (Value::None, _) => Ok(Value::Boolean(false)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn not_equals(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a ^ b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a != b)),
            (Value::None, Value::None) => Ok(Value::Boolean(false)),
            (_, Value::None) | (Value::None, _) => Ok(Value::Boolean(true)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn greater(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a > b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a > b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn greater_eq(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a >= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a >= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn less_than(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a < b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a < b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    fn less_than_eq(self, other: Self, operator: &Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Character(a), Value::Character(b)) => Ok(Value::Boolean(a <= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }
}

impl From<&Token> for Value {
    fn from(value: &Token) -> Self {
        match &value.kind {
            TokenKind::Integer(value) => Self::Integer(*value),
            TokenKind::Float(value) => Self::Float(*value),
            TokenKind::Boolean(value) => Self::Boolean(*value),
            TokenKind::String(value) => Self::String(value.to_string()),
            TokenKind::Character(value) => Self::Character(*value),
            TokenKind::None => Self::None,
            _ => unreachable!(),
        }
    }
}
