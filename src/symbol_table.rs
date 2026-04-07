//! # Symbol Table Module
//!
//! Implements variable storage and lookup with support for nested scopes.
//! Manages the mapping between identifiers and their runtime values
//! during program execution.

use crate::values::Value;
use std::collections::HashMap;

/// Symbol table for variable storage with parent scoping
#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Value>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    /// Creates a new symbol table
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a new symbol table with a parent for scoping
    pub fn with_parent(parent: SymbolTable) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Gets a value from the symbol table (searching parents)
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.symbols.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    /// Sets a value in the current symbol table
    pub fn set(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value);
    }

    /// Removes a value from the current symbol table
    pub fn remove(&mut self, name: &str) -> Option<Value> {
        self.symbols.remove(name)
    }

    /// Updates a variable in the scope it was originally defined, or sets in current scope if not found
    pub fn set_existing(&mut self, name: String, value: Value) {
        if self.symbols.contains_key(&name) {
            self.symbols.insert(name, value);
        } else if let Some(parent) = &mut self.parent {
            parent.set_existing(name, value);
        } else {
            self.symbols.insert(name, value);
        }
    }

    pub fn set_local(&mut self, name: String, value: Value) {
        // Explicitly set ONLY in this table's own HashMap, never in parent
        self.symbols.insert(name, value);
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
