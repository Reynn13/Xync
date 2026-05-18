use crate::{Token, Type, VariableDescriptor};

#[derive(Debug, PartialEq, Clone)]
pub enum OpKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(String),
    Identifier(String),
    BinaryExpr {
        left: Box<Value>,
        op: OpKind,
        right: Box<Value>,
        evaluated_ty: Option<Type>
    },

    Absolute(Box<Value>),
    Negate(Box<Value>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mutability {
    /// Immutable, Runtime
    Immutable,

    /// Immutable, constants
    Const,

    /// Mutable, visible inside the file where the program first started
    Mutable,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Variable(Variable),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub ident: Token,
    pub ty: Option<Type>,
    pub value: Option<Value>,

    pub desc: VariableDescriptor
}

impl Variable {
    pub fn new(ident: Token, ty: Option<Type>, value: Option<Value>, desc: VariableDescriptor) -> Self {
        Self {
            ident,
            ty,
            value,
            desc,
        }
    }
}
