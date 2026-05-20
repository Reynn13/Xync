use crate::{RuntimeValue, Type};

#[derive(PartialEq)]
pub struct RuntimeVariable {
    name: String,
    ty: Type,
    value: RuntimeValue
}

impl RuntimeVariable {
    pub fn new(name: String, ty: Type, value: RuntimeValue) -> Self {
        Self {
            name, ty, value
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn value(&self) -> &RuntimeValue {
        &self.value
    }
}