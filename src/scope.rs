//! Contains the scope module for tracking variable scopes
//! Scopes in Palladium are lexical

use std::collections::HashMap;

pub struct Scope {
    variables: HashMap<String, String>
}

impl Scope {
    /// Creates and returns a new Scope
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new()
        }
    }

    /// Checks if a Scope has a specific variable
    pub fn has_variable(&self, variable: &str) -> bool {
        self.variables.contains_key(variable)
    }
}