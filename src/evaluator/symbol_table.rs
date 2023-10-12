use crate::evaluator::value::Value;

use std::collections::HashMap;

pub struct SymbolTable {
    symbol_table: HashMap<String, Value>,
    _parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        Self {
            symbol_table: HashMap::new(),
            _parent: parent,
        }
    }

    pub fn insert(&mut self, identifier: String, value: Value) {
        self.symbol_table.insert(identifier, value);
    }

    pub fn get(&mut self, identifier: &str) -> Option<Value> {
        self.symbol_table.get(identifier).cloned()
    }
}
