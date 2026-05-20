use crate::{OpKind, RuntimeScope, RuntimeValue, RuntimeVariable, SemanticScope, Value};

pub struct Interpreter {
    runtime_env: Vec<RuntimeScope>
}

impl Interpreter {
    pub fn new() -> Self {
        Self { runtime_env: Vec::new() }
    }

    pub fn run(&mut self, env: Vec<SemanticScope>) {
        self.eval_scope(0, &env);
    }

    fn get_variable_recursive(
        &self,
        mut scope_id: usize,
        varname: &String,
    ) -> (usize, &RuntimeVariable) {
        while self.runtime_env.get(scope_id) != None {
            if let Some(v) = self.runtime_env[scope_id].get_variable(&varname) {
                return (scope_id, v);
            }
            scope_id -= 1;
        }
        unreachable!()
    }

    fn eval_scope(&mut self, scope_id: usize, env: &Vec<SemanticScope>) {
        self.runtime_env.push(RuntimeScope::default()); 
        let ref scope = env[scope_id];
        for var in scope.get_variables() {
            if let Some(ref value) = var.value {
                let value = self.eval_value(0, scope, value);
                self.runtime_env[scope_id].add_variable(

                    RuntimeVariable::new(var.ident.lexeme.clone(), var.ty.unwrap(), value)
                );
            }
        }
        for child_id in scope.get_children() {
            self.eval_scope(*child_id, env);
        }
    }

    fn eval_value(&self, scope_id: usize, scope: &SemanticScope, value: &Value) -> RuntimeValue {
        match value {
            Value::Integer(n) => RuntimeValue::I32(n.parse().unwrap()),
            Value::Absolute(value) => match self.eval_value(scope_id, scope, value) {
                RuntimeValue::I32(n) => RuntimeValue::I32(n.abs()),
            },
            Value::Negate(value) => match self.eval_value(scope_id, scope, value) {
                RuntimeValue::I32(n) => RuntimeValue::I32(-n),
            },
            Value::BinaryExpr {
                left,
                op,
                right,
                evaluated_ty: _,
            } => {
                let left = self.eval_value(scope_id,scope, left);
                let right = self.eval_value(scope_id, scope, right);
                match (left, right) {
                    (RuntimeValue::I32(n1), RuntimeValue::I32(n2)) => match op {
                        OpKind::Addition => RuntimeValue::I32(n1 + n2),
                        OpKind::Subtraction => RuntimeValue::I32(n1 - n2),
                        OpKind::Multiplication => RuntimeValue::I32(n1 * n2),
                        OpKind::Division => RuntimeValue::I32(n1 / n2),
                    },
                }
            }
            Value::Identifier(s) => self.get_variable_recursive(scope_id, s).1.value().clone() 
        }
    }
}
