use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::{debug, trace};

use crate::{
    ast::program::{Declaration, Expression, Identifier},
    common::folder::Folder,
};

#[derive(Default)]
pub struct VariableResolver {
    variable_map: HashMap<String, String>,
}

impl VariableResolver {
    pub fn with(&mut self, variable_map: &HashMap<String, String>) -> Self {
        Self {
            variable_map: variable_map.clone(),
        }
    }
}

impl Folder for VariableResolver {
    fn create() -> Self {
        Self::default()
    }

    fn fold_declaration(&mut self, declaration: &Declaration) -> Result<Declaration, String> {
        trace!("resolving declaration: {declaration:?}");

        if self.variable_map.contains_key(&declaration.name.value) {
            debug!("variable: {declaration}");
            return Err("variable already declared".to_string());
        }

        let unique_name: String = temporary_name(&declaration.name.value);
        self.variable_map
            .insert(declaration.name.value.clone(), unique_name.clone());

        let init = declaration
            .initializer
            .as_ref()
            .map(|e| self.fold_expression(e)) // option result expr
            .transpose()?; // result expr

        Ok(Declaration::new(Identifier::new(unique_name), init))
    }

    fn fold_expression(&mut self, expr: &Expression) -> Result<Expression, String> {
        use Expression::*;

        trace!("resolving expression: {expr:?}");

        let res = match expr {
            Assignment(left, right) => match **left {
                Var(_) => Assignment(
                    Box::new(self.fold_expression(left)?),
                    Box::new(self.fold_expression(right)?),
                ),
                _ => {
                    return Err("invalid lvalue".to_string());
                }
            },
            Var(id) => {
                if let Some(v) = self.variable_map.get(&id.value) {
                    Var(Identifier::new(v.clone()))
                } else {
                    debug!("undeclared variable: {expr}");
                    return Err("undeclared variable".to_string());
                }
            }
            // TODO: this should be refactored by a call to the super method
            Unary(op, expr) => Unary(op.clone(), Box::new(self.fold_expression(expr)?)),
            Binary(op, left, right) => Binary(
                op.clone(),
                Box::new(self.fold_expression(left)?),
                Box::new(self.fold_expression(right)?),
            ),
            Constant(c) => Constant(*c),
            Conditional(cond, then, el) => Conditional(
                Box::new(self.fold_expression(cond)?),
                Box::new(self.fold_expression(then)?),
                Box::new(self.fold_expression(el)?),
            ),
        };

        Ok(res)
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn temporary_name(name: &str) -> String {
    let id = next_id();
    format!("{name}.{id}")
}
