use crate::evaluator::value::Value;
use crate::models::error::Error;
use crate::parser::node::Node::{self, Atom, BinaryOp, UnaryOp};
use crate::tokenizer::token::TokenKind;

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(ast: Box<Node>) -> Result<Value, Error> {
        match *ast {
            Atom(token) => match token.kind {
                TokenKind::Integer(value) => Ok(Value::Integer(value)),
                TokenKind::Float(value) => Ok(Value::Float(value)),
                TokenKind::Boolean(value) => Ok(Value::Boolean(value)),
                _ => todo!(),
            },
            BinaryOp(right, op, left) => {
                let right_value = Self::evaluate(right)?;

                match op.kind {
                    TokenKind::Add => Ok(right_value.add(Self::evaluate(left)?)?),
                    TokenKind::Sub => Ok(right_value.sub(Self::evaluate(left)?)?),
                    TokenKind::Mul => Ok(right_value.mul(Self::evaluate(left)?)?),
                    TokenKind::Div => Ok(right_value.div(Self::evaluate(left)?)?),
                    TokenKind::Mod => Ok(right_value.modulo(Self::evaluate(left)?)?),
                    TokenKind::Pow => Ok(right_value.pow(Self::evaluate(left)?)?),
                    TokenKind::And => Ok(right_value.and(Self::evaluate(left)?)?),
                    TokenKind::Or => Ok(right_value.or(Self::evaluate(left)?)?),
                    TokenKind::Equals => Ok(right_value.equals(Self::evaluate(left)?)?),
                    TokenKind::NotEquals => Ok(right_value.not_equals(Self::evaluate(left)?)?),
                    TokenKind::Greater => Ok(right_value.greater(Self::evaluate(left)?)?),
                    TokenKind::GreaterEq => Ok(right_value.greater_eq(Self::evaluate(left)?)?),
                    TokenKind::LessThan => Ok(right_value.less_than(Self::evaluate(left)?)?),
                    TokenKind::LessThanEq => Ok(right_value.less_than_eq(Self::evaluate(left)?)?),
                    _ => todo!(),
                }
            }
            UnaryOp(op, node) => {
                let value = Self::evaluate(node)?;
                match op.kind {
                    TokenKind::Add => Ok(value.uadd()?),
                    TokenKind::Sub => Ok(value.usub()?),
                    TokenKind::Not => Ok(value.not()?),
                    _ => todo!(),
                }
            }
        }
    }
}
