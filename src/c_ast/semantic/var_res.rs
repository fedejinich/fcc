use std::{collections::HashMap, sync::atomic::AtomicUsize};

use log::{debug, trace};

use crate::{
    c_ast::ast::{Declaration, Expression, Identifier, Statement},
    common::{folder::FolderC, util::temporary_name},
};

pub type UniqueName = String;
pub type FromCurrentBlock = bool;
/// A tuple containing the variable unique name and whether it is declared for the current block
pub type VarValue = (UniqueName, FromCurrentBlock);
pub type VarName = String;

static VAR_RES_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
pub struct VariableResolver(HashMap<VarName, VarValue>);

impl VariableResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(var_map: HashMap<VarName, VarValue>) -> Self {
        Self(var_map)
    }

    /// Wether a variable is already named with the given name
    fn is_var_named(&self, var_name: &Identifier) -> bool {
        let Some(_) = self.get_var(var_name) else {
            return false;
        };
        return true;
    }

    /// Wether a variable is already declared with the given name
    fn is_var_declred(&self, var_name: &Identifier) -> bool {
        let Some(var_value) = self.get_var(var_name) else {
            return false;
        };
        return var_value.1;
    }

    pub fn get_var(&self, var_name: &Identifier) -> Option<VarValue> {
        self.0.get(var_name.value()).cloned()
    }

    fn track_variable(&mut self, var_name: Identifier, unique_name: String) {
        self.0
            .insert(var_name.value().to_string(), (unique_name, true));
    }

    /// returns a copy of the variables map with all the variables marked as undeclared for the current block
    fn copy_variable_map(&self) -> HashMap<VarName, VarValue> {
        self.0
            .clone() // copy variables map
            .iter()
            .map(|(k, v)| {
                let mut var_name = v.clone();
                var_name.1 = false;

                (k.clone(), var_name)
            })
            .collect()
    }
}

impl FolderC for VariableResolver {
    fn fold_decl(&mut self, declaration: Declaration) -> Result<Declaration, String> {
        trace!("resolving declaration: {declaration:?}");

        if self.is_var_named(&declaration.name) && self.is_var_declred(&declaration.name) {
            debug!("variable: {declaration}");
            return Err("duplicate variable declaration".to_string());
        }

        let unique_name: String = temporary_name(declaration.name.value(), &VAR_RES_COUNT);

        self.track_variable(declaration.name.clone(), unique_name.clone());

        let init = declaration
            .initializer
            .map(|e| self.fold_expr(e))
            .transpose()?;

        Ok(Declaration::new(Identifier::new(unique_name), init))
    }

    fn fold_st(&mut self, statement: Statement) -> Result<Statement, String> {
        match statement {
            Statement::Compound(block) => {
                trace!("resolving compound statement");

                let new_variable_map = self.copy_variable_map();
                let mut new_var_resolver = Self::new_with(new_variable_map);
                let resolved_block = new_var_resolver.fold_block(*block)?;

                return Ok(Statement::Compound(Box::new(resolved_block)));
            }
            _ => {
                trace!("resolving statement");

                self.default_fold_st(statement)
            }
        }
    }

    fn fold_expr(&mut self, expr: Expression) -> Result<Expression, String> {
        trace!("resolving expression: {expr:?}");

        let res = match expr {
            Expression::Assignment(left, right) => match *left {
                Expression::Var(_) => Expression::Assignment(
                    Box::new(self.fold_expr(*left)?),
                    Box::new(self.fold_expr(*right)?),
                ),
                _ => {
                    return Err("invalid lvalue".to_string());
                }
            },
            Expression::Var(ref id) => {
                if let Some((var_unique_name, _)) = self.get_var(&id) {
                    Expression::Var(Identifier::new(var_unique_name))
                } else {
                    debug!("undeclared variable: {expr}");

                    return Err("undeclared variable".to_string());
                }
            }
            Expression::Unary(op, expr) => Expression::Unary(op, Box::new(self.fold_expr(*expr)?)),
            Expression::Binary(op, left, right) => Expression::Binary(
                op,
                Box::new(self.fold_expr(*left)?),
                Box::new(self.fold_expr(*right)?),
            ),
            Expression::Constant(c) => Expression::Constant(c),
            Expression::Conditional(cond, then, el) => Expression::Conditional(
                Box::new(self.fold_expr(*cond)?),
                Box::new(self.fold_expr(*then)?),
                Box::new(self.fold_expr(*el)?),
            ),
        };

        Ok(res)
    }
}
