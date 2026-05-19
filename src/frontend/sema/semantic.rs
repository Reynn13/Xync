use crate::{
    Diagnostic, DiagnosticBuilder, ScopeArena, SemanticBound, SemanticScope, Statement, Type,
    Value, Variable, XynError,
};

enum SemanticError {
    UndeclaredVariable,
    ConflictingVariable,
    _AbsoluteInvalidType,
    _NegateInvalidType,
}

impl XynError for SemanticError {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn message(&self) -> &'static str {
        use SemanticError::*;
        match self {
            UndeclaredVariable => "Undeclared variable:",
            ConflictingVariable => "Conflicting variable:",
            _AbsoluteInvalidType => "Expected integers for absolutes (+), found:",
            _NegateInvalidType => "Expected integers for negations (-), found:",
        }
    }
}

#[derive(Debug)]
pub struct Semantic {
    envs: Vec<SemanticScope>,
    // TODO: Handle unbound resolve
    unbound_counter: usize,
}

impl Semantic {
    pub fn new(arena_size: usize) -> Self {
        let mut envs = Vec::with_capacity(arena_size);
        for _ in 0..arena_size {
            envs.push(SemanticScope::default());
        }
        Self {
            envs,
            unbound_counter: 0,
        }
    }

    pub fn analyze(&mut self, arena: ScopeArena) -> Result<(), Diagnostic> {
        self.analyze_scope(0, &arena)
    }

    fn get_variable_recursive(
        &self,
        mut scope_id: usize,
        varname: &String,
    ) -> Result<(usize, &Variable), Diagnostic> {
        while self.envs.get(scope_id) != None {
            if let Some(v) = self.envs[scope_id].get_variable(&varname) {
                return Ok((scope_id, v));
            }
            scope_id -= 1;
        }

        Err(DiagnosticBuilder::default()
            .with_error_code(SemanticError::UndeclaredVariable)
            .build())
    }

    fn get_mut_bounds_recursive(
        &mut self,
        mut scope_id: usize,
        unbound_id: usize,
    ) -> (usize, &mut SemanticBound) {
        loop {
            if self.envs[scope_id].get_mut_bounds(unbound_id).is_some() {
                return (
                    scope_id,
                    self.envs[scope_id].get_mut_bounds(unbound_id).unwrap(),
                );
            }

            if scope_id == 0 {
                panic!("No bound found");
            }

            // climbing
            scope_id -= 1;
        }
    }

    fn analyze_scope(&mut self, scope_id: usize, arena: &ScopeArena) -> Result<(), Diagnostic> {
        let scope = arena.get_scope(scope_id).unwrap();

        // debug
        // println!("{}", scope.get_statements().len());
        for stmt in scope.get_statements() {
            self.analyze_statement(scope_id, stmt.clone(), arena)?;
        }

        for child_id in scope.get_children_scopes() {
            self.analyze_scope(*child_id, arena)?;
        }
        Ok(())
    }

    fn analyze_statement(
        &mut self,
        scope_id: usize,
        stmt: Statement,
        arena: &ScopeArena,
    ) -> Result<(), Diagnostic> {
        match stmt {
            Statement::Variable(mut v) => {
                // ? Variable already declared
                // ?NOTE: We only check for the variable in this scope, allowing user to make a shadowing variable
                if let Some(_v) = self.envs[scope_id].get_variable(&v.ident.lexeme) {
                    return Err(DiagnosticBuilder::default()
                        .with_error_code(SemanticError::ConflictingVariable)
                        .build());
                }

                // ? Get type
                let v_ty = if v.ty == None {
                    if let Some(ref mut val) = v.value {
                        self.analyze_expr(&v.ident.lexeme, scope_id, val, arena)?
                    } else {
                        let bound_id = self.unbound_counter;
                        self.unbound_counter += 1;
                        self.envs[scope_id]
                            .add_bounds(bound_id, SemanticBound::new(v.ident.lexeme.clone()));
                        Type::Unbound(bound_id)
                    }
                } else {
                    v.ty.unwrap()
                };
                self.envs[scope_id].add_variable(Variable::new(
                    v.ident,
                    Some(v_ty),
                    v.value,
                    v.desc,
                ));
                Ok(())
            }
        }
    }

    // ?NOTE: The unification happens if the semantic knows an unbound variable real type based by a context
    fn unify_unbound_to(
        &mut self,
        scope_id: usize,
        unbound_id: usize,
        arena: &ScopeArena,
        ty: Type,
    ) -> Result<(), Diagnostic> {
        let target_var;
        let relations;
        let found_at_scope;

        let (scope_id, bounds) = self.get_mut_bounds_recursive(scope_id, unbound_id);
        found_at_scope = scope_id;
        if bounds.resolved {
            // TODO LATER: Handle unification checking
            return Ok(());
        }
        bounds.resolved = true;
        target_var = bounds.target_var.clone();
        relations = bounds.relations.clone();

        // ? Update the main variable it's point to
        if let Some(var) = self.envs[found_at_scope].get_mut_variable(&target_var) {
            var.ty = Some(ty.clone());

            // ? Unbound variable value must also be evaluated to unbound, so we need to re-evaluate it
            if let Some(mut val) = var.value.clone() {
                self.analyze_expr(&target_var, found_at_scope, &mut val, arena)?;
                self.envs[found_at_scope]
                    .get_mut_variable(&target_var)
                    .unwrap()
                    .value = Some(val);
            }
        }

        // ? Then update it's relations
        for bound_id in relations {
            self.unify_unbound_to(scope_id, bound_id, arena, ty)?;
        }
        Ok(())
    }

    fn analyze_expr(
        &mut self,
        varname: &String,
        scope_id: usize,
        val: &mut Value,
        arena: &ScopeArena,
    ) -> Result<Type, Diagnostic> {
        match val {
            // TODO LATER: Handle Integer bounds & overflows
            Value::Integer(_) => Ok(Type::I32), // ? Default type for integers,
            Value::Identifier(s) => match self.get_variable_recursive(scope_id, s) {
                Ok((_, v)) => Ok(v.ty.unwrap()),
                Err(e) => Err(e),
            },
            Value::Absolute(v) => match self.analyze_expr(varname, scope_id, v, arena)? {
                Type::I32 => Ok(Type::I32),

                Type::Unbound(id) => {
                    // ? Only integers valid in `+x` context, so `x` must be also integer
                    self.unify_unbound_to(scope_id, id, arena, Type::I32)?;
                    Ok(Type::I32)
                }
            },
            Value::Negate(v) => match self.analyze_expr(varname, scope_id, v, arena)? {
                Type::I32 => Ok(Type::I32),

                Type::Unbound(id) => {
                    // ? Only integers valid in `-x` context, so `x` must be also integer
                    self.unify_unbound_to(scope_id, id, arena, Type::I32)?;
                    Ok(Type::I32)
                }
            },
            Value::BinaryExpr {
                left,
                op: _,
                right,
                evaluated_ty,
            } => {
                if let Some(t) = evaluated_ty {
                    match t {
                        // ? Allowing re-evaluation
                        Type::Unbound(_) => (),
                        t => return Ok(*t),
                    }
                }
                let left_t = self.analyze_expr(varname, scope_id, left, arena)?;
                let right_t = self.analyze_expr(varname, scope_id, right, arena)?;
                let t = self.promote_ty(varname.clone(), scope_id, arena, left_t, right_t)?;
                *evaluated_ty = Some(t);
                Ok(t)
            }
        }
    }

    fn promote_ty(
        &mut self,
        varname: String,
        scope_id: usize,
        arena: &ScopeArena,
        left_t: Type,
        right_t: Type,
    ) -> Result<Type, Diagnostic> {
        match (left_t, right_t) {
            (Type::I32, Type::I32) => Ok(Type::I32),

            // ?NOTE: An unbound variable meet with another unbound variable, assuming their type is the same
            (Type::Unbound(id1), Type::Unbound(id2)) => {
                let (_, bounds) = self.get_mut_bounds_recursive(scope_id, id1);
                // ? id1 -> id2
                bounds.add_relation(id2);

                // ? this variable new unbound id
                let new_bound_id = self.unbound_counter;
                self.unbound_counter += 1; // skip

                let (_, bounds) = self.get_mut_bounds_recursive(scope_id, id2);
                // ? id2 -> id3
                bounds.add_relation(id1);

                // ? id2 -> 
                bounds.add_relation(new_bound_id);

                // ? promote_ty() used in variables context, so we need add another semantic bound that targets it
                self.envs[scope_id].add_bounds(new_bound_id, SemanticBound::new(varname));
                Ok(Type::Unbound(new_bound_id))
            }

            // ?NOTE: An unbound variable meet with a value, assuming that variable type is also the same as the value
            (t, Type::Unbound(id)) | (Type::Unbound(id), t) => {
                self.unify_unbound_to(scope_id, id, arena, t)?;
                Ok(t)
            }
        }
    }
}
