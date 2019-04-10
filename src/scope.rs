//! Contains the scope module for tracking variable scopes
//! Scopes in Palladium are lexical

use std::collections::HashMap;

pub struct Scope {
    pub variables: HashMap<String, u8>,
    return_registers: Vec<u8>,
    pub used_registers: Vec<u8>,
    bp_offset: i32,
}

impl Scope {
    /// Creates and returns a new Scope
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
            return_registers: vec![],
            used_registers: vec![],
            bp_offset: 0,
        }
    }

    /// Checks if a Scope has a specific variable
    pub fn has_variable(&self, variable: &str) -> bool {
        self.variables.contains_key(variable)
    }

    pub fn new_variable(&mut self, identifier: &str, register: u8) {
        self.variables.insert(identifier.to_owned(), register);
    }

    pub fn get_variable(&self, variable: &str) -> Option<u8> {
        if let Some(register) = self.variables.get(variable) {
            return Some(register.to_owned());
        }
        None
    }

    /// Gets all the registers used in this scope
    pub fn get_registers(&self) -> Vec<u8> {
        let mut variables = vec![];
        for register in self.variables.values() {
            variables.push(register.to_owned());
        }
        variables
    }

    pub fn add_return_register(&mut self, register: u8) {
        self.return_registers.push(register)
    }

    pub fn pop_return_register(&mut self) -> Option<u8> {
        self.return_registers.pop()
    }

    pub fn clone_all_return_registers(&mut self) -> Vec<u8> {
        self.return_registers.clone()
    }
}