use crate::evaluator::symbol_table::SymbolTable;
use crate::evaluator::value::Value;
use crate::models::error::Error;
use crate::models::error::Runtime;
use crate::parser::node::Node;
use crate::tokenizer::token::TokenKind;

pub struct Evaluator {
    symbol_table: SymbolTable,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(None),
        }
    }

    pub fn evaluate(&mut self, statement: Box<Node>) -> Result<Value, Error> {
        match *statement {
            Node::Atom(token) => Ok(Value::from(token)),
            Node::BinaryOp(right, operation, left) => {
                let right_value = self.evaluate(right)?;
                let left_value = self.evaluate(left)?;
                Ok(right_value.binary_operation(left_value, operation)?)
            }
            Node::UnaryOp(operation, node) => {
                let value = self.evaluate(node)?;
                Ok(value.unary_operation(operation)?)
            }
            Node::Assignment(token, expression) => {
                let value = self.evaluate(expression)?;
                if let TokenKind::Identifier(name) = token.kind {
                    self.symbol_table.insert(name, value.clone());
                }
                Ok(Value::None)
            }
            Node::Access(token) => {
                if let TokenKind::Identifier(name) = &token.kind {
                    if let Some(value) = self.symbol_table.get(name) {
                        return Ok(value);
                    }
                    return Err(Error::runtime(Runtime::UnknownIdentifier {
                        identifier: token,
                    }));
                }
                unreachable!()
            }
            Node::Program { statements } => {
                let mut value = Value::None;
                for statement in statements {
                    value = self.evaluate(statement)?;
                }
                Ok(value)
            }
            Node::If { condition, block } => {
                if self.evaluate(condition)? == Value::Boolean(true) {
                    return Ok(self.evaluate(block)?);
                }
                Ok(Value::None)
            }
        }
    }
}
