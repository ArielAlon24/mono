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

    pub fn evaluate(&mut self, program: Box<Node>) -> Result<Value, Error> {
        match *program {
            Node::Atom { value } => Ok(Value::from(value)),
            Node::BinaryOp {
                right,
                operator,
                left,
            } => {
                let right_value = self.evaluate(right)?;
                let left_value = self.evaluate(left)?;
                Ok(left_value.binary_operation(right_value, operator)?)
            }
            Node::UnaryOp { operator, value } => {
                let value = self.evaluate(value)?;
                Ok(value.unary_operation(operator)?)
            }
            Node::Assignment { identifier, value } => {
                let value = self.evaluate(value)?;
                if let TokenKind::Identifier(name) = identifier.kind {
                    self.symbol_table.insert(name, value.clone());
                }
                Ok(Value::None)
            }
            Node::Access { identifier } => {
                if let TokenKind::Identifier(name) = &identifier.kind {
                    if let Some(value) = self.symbol_table.get(name) {
                        return Ok(value);
                    }
                    return Err(Error::runtime(Runtime::UnknownIdentifier {
                        identifier: identifier,
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
            Node::If {
                condition,
                block,
                else_block,
            } => {
                let result = self.evaluate(condition)?;
                match result {
                    Value::Boolean(true) => return Ok(self.evaluate(block)?),
                    Value::Boolean(false) => {}
                    _ => todo!(),
                }

                if let Some(some_else_block) = else_block {
                    return Ok(self.evaluate(some_else_block)?);
                }
                Ok(Value::None)
            }
            Node::While { condition, block } => {
                while let Value::Boolean(true) = self.evaluate(condition.clone())? {
                    self.evaluate(block.clone())?;
                }
                Ok(Value::None)
            }
        }
    }
}
