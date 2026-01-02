use std::{collections::HashMap, sync::atomic::AtomicUsize};

use log::{debug, error, trace};

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
    fn name(&self) -> &'static str {
        "var_res"
    }

    fn fold_decl(&mut self, declaration: Declaration) -> Result<Declaration, String> {
        trace!("[semantic] <declaration> {}", declaration.name().value());

        if self.is_var_named(declaration.name()) && self.is_var_declred(declaration.name()) {
            error!(
                "[semantic] duplicate variable: {}",
                declaration.name().value()
            );

            return Err("duplicate variable declaration".to_string());
        }

        let unique_name = temporary_name(declaration.name().value(), &VAR_RES_COUNT);

        debug!(
            "[semantic] {} -> {}",
            declaration.name().value(),
            unique_name
        );

        self.track_variable(declaration.name().clone(), unique_name.clone());
        let init = declaration
            .initializer()
            .cloned()
            .map(|e| self.fold_expr(e))
            .transpose()?;

        Ok(Declaration::new(Identifier::new(unique_name), init))
    }

    fn fold_st(&mut self, statement: Statement) -> Result<Statement, String> {
        let res = match statement {
            Statement::Compound(block) => {
                trace!("[semantic] <statement> compound (new scope)");

                let new_var_map = self.copy_variable_map();
                let mut new_resolver = Self::new_with(new_var_map);

                Statement::Compound(Box::new(new_resolver.fold_block(*block)?))
            }
            Statement::For(for_init, cond, post, body, id) => {
                let new_var_map = self.copy_variable_map();
                let mut new_resolver = Self::new_with(new_var_map);

                new_resolver.default_fold_st_for(for_init, cond, post, body, id)?
            }
            _ => self.default_fold_st(statement)?,
        };

        Ok(res)
    }

    fn fold_expr(&mut self, expr: Expression) -> Result<Expression, String> {
        match expr {
            Expression::Assignment(left, right) => match *left {
                Expression::Var(_) => Ok(Expression::Assignment(
                    Box::new(self.fold_expr(*left)?),
                    Box::new(self.fold_expr(*right)?),
                )),
                _ => {
                    error!("[semantic] invalid lvalue in assignment");

                    Err("invalid lvalue".to_string())
                }
            },
            Expression::Var(ref id) => {
                let Some((unique_name, _)) = self.get_var(id) else {
                    error!("[semantic] undeclared variable: {}", id.value());

                    return Err("undeclared variable".to_string());
                };

                Ok(Expression::Var(Identifier::new(unique_name)))
            }
            Expression::Unary(op, e) => Ok(Expression::Unary(op, Box::new(self.fold_expr(*e)?))),
            Expression::Binary(op, l, r) => Ok(Expression::Binary(
                op,
                Box::new(self.fold_expr(*l)?),
                Box::new(self.fold_expr(*r)?),
            )),
            Expression::Constant(c) => Ok(Expression::Constant(c)),
            Expression::Conditional(c, t, e) => Ok(Expression::Conditional(
                Box::new(self.fold_expr(*c)?),
                Box::new(self.fold_expr(*t)?),
                Box::new(self.fold_expr(*e)?),
            )),
        }
    }
}
