use crate::{Mutability, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDescriptor {
    mutability: Mutability,
    evaluated_type: Option<Type>
}

impl VariableDescriptor {
    pub fn default(mutability: Mutability) -> Self {
        Self {
            mutability,
            evaluated_type: None
        }
    }
    pub fn mutability(&self) -> Mutability {
        self.mutability
    }
    pub fn evaluated_type(&self) -> Option<Type> {
        self.evaluated_type
    }
}