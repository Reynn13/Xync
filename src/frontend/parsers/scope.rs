
use crate::{Statement};

pub type ScopeId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeType {
    Local, // file
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    ty: ScopeType,
    children: Vec<ScopeId>,

    statements: Vec<Statement>,
}

impl Scope {
    pub fn new(ty: ScopeType) -> Self {
        Self {
            parent: None,
            ty,
            children: Vec::new(),
            statements: Vec::new()
        }
    }
    pub fn with_parent(parent: Option<ScopeId>, ty: ScopeType) -> Self {
        Self {
            parent,
            ty,
            children: Vec::new(),
            statements: Vec::new()
        }
    }

    pub fn get_parent(&self) -> Option<ScopeId> {
        self.parent
    }
    pub fn get_scope_type(&self) -> ScopeType {
        self.ty
    }
    pub fn add_child_scope(&mut self, scope_id: ScopeId) {
        self.children.push(scope_id);
    }
    pub fn get_children_scopes(&self) -> &Vec<ScopeId> {
        &self.children
    }
    pub fn add_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

#[derive(Debug)]
pub struct ScopeArena {
    // file_scope at idx 0
    scopes: Vec<Scope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }
    pub fn add_scope(&mut self, scope: Scope) {
        let id = self.scopes.len();
        let parent = scope.get_parent();
        self.scopes.push(scope);

        if let Some(parent_id) = parent {
            self.scopes[parent_id].add_child_scope(id);
        }
    }
    pub fn add_statement_to(&mut self, id: ScopeId, stmt: Statement) {
        self.scopes.get_mut(id).expect("Invalid scope id").add_statement(stmt);
    } 
    pub fn get_scope(&self, id: ScopeId) -> Option<&Scope> {
        self.scopes.get(id)
    }
    pub fn get_file_scope(&self) -> Option<&Scope> {
        self.scopes.first()
    }
    pub fn size(&self) -> usize {
        self.scopes.len()
    }
}
