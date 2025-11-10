use std::collections::HashMap;
use std::process;

pub struct SemanticAnalyzer {
    // Stack of scope maps using...HASHMAPS :D
    scopes: Vec<HashMap<String, String>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer { scopes: vec![HashMap::new()] } // global scope
    }

    /// Define a variable in the current scope
    pub fn define_var(&mut self, name: &str, value: &str) {
        if let Some(current) = self.scopes.last_mut() {
            current.insert(name.to_string(), value.to_string());
        }
    }

    /// Use a variable (look up its value in any enclosing scope)
    pub fn use_var(&self, name: &str) -> Result<String, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        Err(format!("Static Semantic Error: variable `{}` used before definition", name))
    }

    // Enter a new nested scope (i.e. inside a block)
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Leave the most recent scope
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}
