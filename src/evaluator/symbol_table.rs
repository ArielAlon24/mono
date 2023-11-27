use crate::evaluator::builtins;
use crate::evaluator::builtins::builtin;
use crate::evaluator::value::Value;
use std::collections::HashMap;

// TODO: Make the symbol table, stack based.
pub struct SymbolTable {
    tables: Vec<HashMap<String, Value>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            tables: vec![HashMap::new()],
        }
    }

    pub fn insert(&mut self, identifier: String, value: Value) {
        if self.tables.is_empty() {
            panic!("Internal Error: Symbol Table dropped.")
        }
        self.tables.first_mut().unwrap().insert(identifier, value);
    }

    pub fn insert_tuple(&mut self, (identifier, value): (String, Value)) {
        self.insert(identifier, value);
    }

    pub fn get(&self, identifier: &str) -> Option<Value> {
        for table in &self.tables {
            if let Some(value) = table.get(identifier) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn get_mut(&mut self, identifier: &str) -> Option<&mut Value> {
        for table in &mut self.tables {
            if let Some(value) = table.get_mut(identifier) {
                return Some(value);
            }
        }
        None
    }

    pub fn contains(&mut self, identifier: &str) -> bool {
        return self.get(identifier) != None;
    }

    pub fn scope(&mut self) {
        self.tables.push(HashMap::new());
    }

    pub fn unscope(&mut self) {
        if self.tables.len() <= 1 {
            panic!("Internal Error: Tried to drop the main symbol table.")
        }
        self.tables.remove(self.tables.len() - 1);
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
