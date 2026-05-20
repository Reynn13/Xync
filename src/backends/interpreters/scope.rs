use std::collections::HashMap;

use crate::RuntimeVariable;

#[derive(Default, PartialEq)]
pub struct RuntimeScope {
    variables: HashMap<String, RuntimeVariable>,
    children: Vec<RuntimeScope>
}

impl RuntimeScope {
    pub fn add_variable(&mut self, var: RuntimeVariable) {
        self.variables.insert(var.name().clone(), var);
    }
    pub fn get_variable(&self, varname: impl AsRef<str>) -> Option<&RuntimeVariable> {
        self.variables.get(varname.as_ref())
    }
}