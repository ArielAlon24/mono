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
}
