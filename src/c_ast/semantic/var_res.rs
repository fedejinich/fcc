use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::{debug, trace};

use crate::{
    c_ast::ast::{Declaration, Expression, Identifier},
    common::folder::FolderC,
};

pub type VarValue = String;
pub type VarName = String;

#[derive(Default)]
pub struct VariableResolver {
    variable_map: HashMap<VarName, VarValue>,
}

impl VariableResolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wether a variable is already named with the given name
    fn is_var_named(&self, var_name: &Identifier) -> bool {
        let Some(_) = self.get_var(var_name) else {
            return false;
        };
        return true;
    }

    /// Wether a variable is already declared with the given name
    fn is_var_declared(&self, _var_name: &Identifier) -> bool {
        todo!()
    }

    pub fn get_var(&self, var_name: &Identifier) -> Option<VarValue> {
        self.variable_map.get(&var_name.value).cloned()
    }
}

impl FolderC for VariableResolver {
    fn fold_declaration(&mut self, declaration: Declaration) -> Result<Declaration, String> {
        trace!("resolving declaration: {declaration:?}");

        if self.is_var_named(&declaration.name) &&
            self.is_var_declared(&declaration.name) {
            debug!("variable: {declaration}");
            return Err("variable already declared".to_string());
        }

        let unique_name: String = temporary_name(&declaration.name.value);
        self.variable_map
            .insert(declaration.name.value.clone(), unique_name.clone());

        let init = declaration
            .initializer
            .map(|e| self.fold_expression(e))
            .transpose()?;

        Ok(Declaration::new(Identifier::new(unique_name), init))
    }

    fn fold_expression(&mut self, expr: Expression) -> Result<Expression, String> {
        trace!("resolving expression: {expr:?}");

        let res = match expr {
            Expression::Assignment(left, right) => match *left {
                Expression::Var(_) => Expression::Assignment(
                    Box::new(self.fold_expression(*left)?),
                    Box::new(self.fold_expression(*right)?),
                ),
                _ => {
                    return Err("invalid lvalue".to_string());
                }
            },
            Expression::Var(ref id) => {
                if let Some(v) = self.get_var(&id.value) {
                    Expression::Var(Identifier::new(v))
                } else {
                    debug!("undeclared variable: {expr}");

                    return Err("undeclared variable".to_string());
                }
            }
            Expression::Unary(op, expr) => {
                Expression::Unary(op, Box::new(self.fold_expression(*expr)?))
            }
            Expression::Binary(op, left, right) => Expression::Binary(
                op,
                Box::new(self.fold_expression(*left)?),
                Box::new(self.fold_expression(*right)?),
            ),
            Expression::Constant(c) => Expression::Constant(c),
            Expression::Conditional(cond, then, el) => Expression::Conditional(
                Box::new(self.fold_expression(*cond)?),
                Box::new(self.fold_expression(*then)?),
                Box::new(self.fold_expression(*el)?),
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
