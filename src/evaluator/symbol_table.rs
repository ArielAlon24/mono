use crate::evaluator::builtins;
use crate::evaluator::builtins::builtin;
use crate::evaluator::value::Value;
use std::collections::HashMap;

pub struct SymbolTable<'a> {
    symbol_table: HashMap<String, Value>,
    parent: Option<&'a Box<SymbolTable<'a>>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new(parent: Option<&'a Box<SymbolTable>>) -> Self {
        Self {
            symbol_table: HashMap::new(),
            parent: parent,
        }
    }

    pub fn insert(&mut self, identifier: String, value: Value) {
        self.symbol_table.insert(identifier, value);
    }

    pub fn insert_tuple(&mut self, (identifier, value): (String, Value)) {
        self.symbol_table.insert(identifier, value);
    }

    pub fn get(&self, identifier: &str) -> Option<Value> {
        match self.symbol_table.get(identifier) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(identifier),
                None => None,
            },
        }
    }

    pub fn contains(&mut self, identifier: &str) -> bool {
        return self.get(identifier) != None;
    }

    pub fn add_builtins(&mut self) {
        self.insert_tuple(builtin("println", vec!["x"], builtins::println));
        self.insert_tuple(builtin("print", vec!["x"], builtins::print));
        self.insert_tuple(builtin("exit", vec!["exit_code"], builtins::exit));
        self.insert_tuple(builtin("input", Vec::new(), builtins::input));
        self.insert_tuple(builtin("integer", vec!["string"], builtins::integer));
        self.insert_tuple(builtin("string", vec!["value"], builtins::string));
    }
}
