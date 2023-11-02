use crate::evaluator::symbol_table::SymbolTable;
use crate::evaluator::value::Value;
use crate::models::error::Error;
use crate::models::error::Runtime;
use crate::parser::node::Node;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;

pub struct Evaluator {
    symbol_table: SymbolTable,
}

type EvaluatorItem = Result<Value, Error>;

impl Evaluator {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(None),
        }
    }

    fn eval_atom(&mut self, value: &Token) -> EvaluatorItem {
        Ok(Value::from(value))
    }

    fn eval_binary_op(&mut self, right: &Node, operator: &Token, left: &Node) -> EvaluatorItem {
        let right_value = self.evaluate(right)?;
        let left_value = self.evaluate(left)?;
        Ok(left_value.binary_operation(right_value, operator)?)
    }

    fn eval_unary_op(&mut self, operator: &Token, value: &Node) -> EvaluatorItem {
        let value = self.evaluate(value)?;
        Ok(value.unary_operation(operator)?)
    }

    fn eval_assignment(
        &mut self,
        identifier: &Token,
        value: &Node,
        is_declaration: &bool,
    ) -> EvaluatorItem {
        let value = self.evaluate(value)?;

        if let TokenKind::Identifier(name) = &identifier.kind {
            if *is_declaration || self.symbol_table.contains(name) {
                self.symbol_table.insert(name.to_string(), value);
            } else {
                return Err(Runtime::UnknownIdentifier {
                    identifier: identifier.clone(),
                }
                .into());
            }
        }

        Ok(Value::None)
    }

    fn eval_access(&mut self, identifier: &Token) -> EvaluatorItem {
        if let TokenKind::Identifier(name) = &identifier.kind {
            if let Some(value) = self.symbol_table.get(name) {
                return Ok(value);
            }
            return Err(Runtime::UnknownIdentifier {
                identifier: identifier.clone(),
            }
            .into());
        }
        unreachable!()
    }

    fn eval_program(&mut self, statements: &Vec<Box<Node>>) -> EvaluatorItem {
        let mut value = Value::None;
        for statement in statements {
            value = self.evaluate(&statement)?;
        }
        Ok(value)
    }

    fn eval_if(
        &mut self,
        condition: &Node,
        block: &Node,
        else_block: &Option<Box<Node>>,
    ) -> EvaluatorItem {
        let result = self.evaluate(&condition)?;
        match result {
            Value::Boolean(true) => return Ok(self.evaluate(&block)?),
            Value::Boolean(false) => {}
            _ => todo!(),
        }

        if let Some(some_else_block) = else_block {
            return Ok(self.evaluate(&some_else_block)?);
        }
        Ok(Value::None)
    }

    fn eval_while(&mut self, condition: &Node, block: &Node) -> EvaluatorItem {
        while let Value::Boolean(true) = self.evaluate(&condition)? {
            self.evaluate(&block)?;
        }
        Ok(Value::None)
    }

    pub fn evaluate(&mut self, program: &Node) -> EvaluatorItem {
        match program {
            Node::Atom { value } => self.eval_atom(&value),
            Node::BinaryOp {
                right,
                operator,
                left,
            } => self.eval_binary_op(&right, &operator, &left),
            Node::UnaryOp { operator, value } => self.eval_unary_op(operator, value),
            Node::Assignment {
                identifier,
                value,
                is_declaration,
            } => self.eval_assignment(identifier, value, is_declaration),
            Node::Access { identifier } => self.eval_access(identifier),
            Node::Program { statements } => self.eval_program(statements),
            Node::If {
                condition,
                block,
                else_block,
            } => self.eval_if(condition, block, else_block),
            Node::While { condition, block } => self.eval_while(condition, block),
            Node::FuncDeclearion { .. } => todo!(),
            &Node::FuncCall { .. } => todo!(),
        }
    }
}
