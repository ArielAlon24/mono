use crate::evaluator::value::Value;
use crate::models::error::Error;
use crate::parser::node::Node::{self, Atom, BinaryOp, UnaryOp};

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(ast: Box<Node>) -> Result<Value, Error> {
        match *ast {
            Atom(token) => Ok(Value::from(token)),
            BinaryOp(right, operation, left) => {
                let right_value = Self::evaluate(right)?;
                let left_value = Self::evaluate(left)?;
                Ok(right_value.binary_operation(left_value, operation)?)
            }
            UnaryOp(operation, node) => {
                let value = Self::evaluate(node)?;
                Ok(value.unary_operation(operation)?)
            }
        }
    }
}
