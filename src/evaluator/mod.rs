pub mod builtins;
pub mod symbol_table;
pub mod value;

use crate::evaluator::symbol_table::SymbolTable;
use crate::evaluator::value::Value;
use crate::internal_err;
use crate::models::error::MonoError;
use crate::models::error::Runtime;
use crate::parser::node::Node;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Evaluator {
    symbol_table: SymbolTable,
}

pub type EvaluatorItem = Result<Value, Box<dyn MonoError>>;

impl Evaluator {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_builtins();
        Self {
            symbol_table: symbol_table,
        }
    }

    pub fn from(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table: symbol_table,
        }
    }

    pub fn evaluate(&mut self, program: &Node) -> EvaluatorItem {
        match program {
            Node::Atom { value } => self.eval_atom(&value),
            Node::List { values } => self.eval_list(&values),
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
            Node::ListAssignment {
                identifier,
                index,
                value,
            } => self.eval_list_assignment(identifier, index, value),
            Node::Access { identifier } => self.eval_access(identifier),
            Node::Index { identifier, index } => self.eval_index(identifier, index),
            Node::Program { statements } => self.eval_program(statements),
            Node::If {
                condition,
                block,
                else_block,
            } => self.eval_if(condition, block, else_block),
            Node::While { condition, block } => self.eval_while(condition, block),
            Node::FuncDeclearion {
                identifier,
                arguments,
                body,
            } => self.eval_func_declaration(identifier, arguments, body),
            Node::FuncCall {
                identifier,
                parameters,
            } => self.eval_func_call(identifier, parameters),
            Node::Return { value } => self.eval_return(value),
        }
    }

    fn eval_atom(&mut self, value: &Token) -> EvaluatorItem {
        Ok(Value::from(value))
    }

    fn eval_list(&mut self, nodes: &Vec<Box<Node>>) -> EvaluatorItem {
        let mut list = Vec::new();
        for node in nodes.iter() {
            list.push(self.evaluate(node)?);
        }
        Ok(Value::List(Rc::new(RefCell::new(list))))
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
        let TokenKind::Identifier(name) = &identifier.kind else {
            internal_err!("Token must be of type Indetifier.")
        };

        if *is_declaration {
            self.symbol_table.insert(name.to_string(), value);
        } else if let Some(old) = self.symbol_table.get_mut(name) {
            *old = value;
        } else {
            return Runtime::UnknownIdentifier {
                identifier: identifier.clone(),
            }
            .into();
        }

        Ok(Value::None)
    }

    fn eval_list_assignment(
        &mut self,
        identifier: &Token,
        index: &Node,
        value: &Node,
    ) -> EvaluatorItem {
        let value = self.evaluate(value)?;
        let index = self.evaluate(index)?;
        let TokenKind::Identifier(name) = &identifier.kind else {
            internal_err!("Token must be of kind Identifier");
        };

        if let Some(list) = self.symbol_table.get(name) {
            return Ok(list.list_assign(index, value, identifier)?);
        }
        return Runtime::UnknownIdentifier {
            identifier: identifier.clone(),
        }
        .into();
    }

    fn eval_access(&mut self, identifier: &Token) -> EvaluatorItem {
        let TokenKind::Identifier(name) = &identifier.kind else {
            internal_err!("Token must be of kind Identifier.")
        };
        if let Some(value) = self.symbol_table.get(name) {
            return Ok(value);
        }
        return Runtime::UnknownIdentifier {
            identifier: identifier.clone(),
        }
        .into();
    }

    fn eval_index(&mut self, identifier: &Token, index: &Box<Node>) -> EvaluatorItem {
        let index = self.evaluate(index)?;
        let TokenKind::Identifier(name) = &identifier.kind else {
            internal_err!("Token must be of kind Identifier.");
        };
        match self.symbol_table.get(name) {
            Some(value) => Ok(value.index(index, identifier)?),
            None => Runtime::UnknownIdentifier {
                identifier: identifier.clone(),
            }
            .into(),
        }
    }

    fn eval_program(&mut self, statements: &Vec<Box<Node>>) -> EvaluatorItem {
        let mut value = Value::None;
        for statement in statements {
            value = self.evaluate(&statement)?;
            if value != Value::None {
                break;
            }
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
            _ => {
                return Runtime::InvalidValue {
                    expected: Value::Boolean(false),
                    found: result,
                }
                .into()
            }
        }

        if let Some(some_else_block) = else_block {
            return Ok(self.evaluate(&some_else_block)?);
        }
        Ok(Value::None)
    }

    fn eval_while(&mut self, condition: &Node, block: &Node) -> EvaluatorItem {
        let mut value = Value::None;
        while let Value::Boolean(true) = self.evaluate(&condition)? {
            value = self.evaluate(&block)?;
            if value != Value::None {
                break;
            }
        }
        Ok(value)
    }

    fn eval_func_declaration(
        &mut self,
        identifier: &Token,
        arguments: &[Token],
        body: &Box<Node>,
    ) -> EvaluatorItem {
        let TokenKind::Identifier(n) = &identifier.kind else {
            internal_err!("Token must be of type Identifier.");
        };
        let string_arguments = arguments
            .iter()
            .map(|arg| {
                if let TokenKind::Identifier(name) = &arg.kind {
                    name.to_string()
                } else {
                    panic!("Expected identifier in function arguments");
                }
            })
            .collect::<Vec<String>>();

        let function = Value::Function {
            name: n.to_string(),
            arguments: string_arguments,
            body: body.clone(),
        };
        self.symbol_table.insert(n.to_string(), function);

        Ok(Value::None)
    }

    fn eval_func_call(&mut self, identifier: &Token, parameters: &Vec<Box<Node>>) -> EvaluatorItem {
        let mut values = Vec::new();
        for parameter in parameters {
            values.push(self.evaluate(parameter)?);
        }
        let TokenKind::Identifier(name) = &identifier.kind else {
            internal_err!("Token must be of type Identifier.");
        };

        return match self.symbol_table.get(&name) {
            Some(Value::Function {
                name,
                arguments,
                body,
            }) => {
                if arguments.len() != parameters.len() {
                    return Runtime::IncorrectParameters {
                        name: name,
                        call: identifier.clone(),
                        expected: arguments,
                        found: values,
                    }
                    .into();
                }

                self.symbol_table.scope();
                for (arg, val) in arguments.into_iter().zip(values.into_iter()) {
                    self.symbol_table.insert(arg, val);
                }
                let result = self.evaluate(&body);
                self.symbol_table.unscope();
                result
            }
            Some(Value::BuiltInFunction {
                name,
                arguments,
                function,
            }) => match arguments.len() != parameters.len() {
                true => Runtime::IncorrectParameters {
                    name: name,
                    call: identifier.clone(),
                    expected: arguments,
                    found: values,
                }
                .into(),
                false => Ok(function(values)),
            },
            _ => Runtime::UnknownIdentifier {
                identifier: identifier.clone(),
            }
            .into(),
        };
    }

    fn eval_return(&mut self, value: &Box<Node>) -> EvaluatorItem {
        self.evaluate(value)
    }
}
