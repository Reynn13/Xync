use std::{collections::HashMap};

use crate::Variable;

#[derive(Debug, Default, PartialEq)]
pub struct SemanticBound {
    pub target_var: String,
    pub relations: Vec<usize>,
    pub resolved: bool,
}

impl SemanticBound {
    pub fn new(target_var: String) -> Self {
        Self {
            target_var,
            ..Default::default()
        }
    }
    pub fn add_relation(&mut self, id: usize) {
        if !self.relations.contains(&id) {
            self.relations.push(id);
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SemanticScope {
    variables: HashMap<String, Variable>,
    bounds: HashMap<usize, SemanticBound>, // (id, (var_name, relations))
}

impl SemanticScope {
    pub fn get_variable(&self, name: impl AsRef<str>) -> Option<&Variable>
    {
        self.variables.get(name.as_ref())
    }
    pub fn add_variable(&mut self, var: Variable) {
        self.variables.insert(var.ident.lexeme.clone(), var);
    }
    pub fn get_mut_variable(&mut self, name: impl AsRef<str>) -> Option<&mut Variable>
    {
        self.variables.get_mut(name.as_ref())
    }
    pub fn get_bounds(&self, id: usize) -> &SemanticBound {
        &self.bounds[&id]
    }
    pub fn get_mut_bounds(&mut self, id: usize) -> Option<&mut SemanticBound> {
        self.bounds.get_mut(&id)
    }
    pub fn add_bounds(&mut self, bound_id: usize, bound: SemanticBound) {
        self.bounds.insert(bound_id, bound);
    }
}
