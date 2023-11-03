use crate::evaluator::builtin_functions;
use crate::evaluator::symbol_table::SymbolTable;
use crate::evaluator::value::Value;
use crate::models::error::Error;
use crate::models::error::Runtime;
use crate::parser::node::Node;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;

pub struct Evaluator<'a> {
    symbol_table: Box<SymbolTable<'a>>,
}

type EvaluatorItem = Result<Value, Error>;

impl<'a> Evaluator<'a> {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new(None);
        symbol_table.insert(
            "print".to_string(),
            Value::BuiltInFunction {
                name: "print".to_string(),
                arguments: vec!["x".to_string()],
                function: builtin_functions::print,
            },
        );
        Self {
            symbol_table: Box::new(symbol_table),
        }
    }

    pub fn from(symbol_table: SymbolTable<'a>) -> Self {
        Self {
            symbol_table: Box::new(symbol_table),
        }
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
            _ => todo!(),
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
        if let TokenKind::Identifier(n) = &identifier.kind {
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
        } else {
            panic!("Expected identifier for function name");
        }

        Ok(Value::None)
    }

    fn eval_func_call(&mut self, identifier: &Token, parameters: &Vec<Box<Node>>) -> EvaluatorItem {
        // TODO: Check corresponding arguments and parameters
        if let TokenKind::Identifier(name) = &identifier.kind {
            match self.symbol_table.get(&name) {
                Some(Value::Function {
                    name: _,
                    arguments,
                    body,
                }) => {
                    let mut pairs = Vec::new();
                    for (name, parameter) in arguments.iter().zip(parameters.iter()) {
                        pairs.push((name.to_string(), self.evaluate(parameter)?));
                    }
                    let mut child_table = SymbolTable::new(Some(&self.symbol_table));
                    for (name, value) in pairs.into_iter() {
                        child_table.insert(name, value);
                    }
                    let mut inner_evaluator = Evaluator::from(child_table);
                    return inner_evaluator.evaluate(&body);
                }
                Some(Value::BuiltInFunction {
                    name: _,
                    arguments: _,
                    function,
                }) => {
                    let mut values = Vec::new();
                    for parameter in parameters.into_iter() {
                        values.push(self.evaluate(parameter)?);
                    }
                    return Ok(function(values));
                }
                _ => {
                    return Err(Runtime::UnknownIdentifier {
                        identifier: identifier.clone(),
                    }
                    .into())
                }
            }
        }
        unreachable!();
    }

    fn eval_return(&mut self, value: &Box<Node>) -> EvaluatorItem {
        self.evaluate(value)
    }
}
