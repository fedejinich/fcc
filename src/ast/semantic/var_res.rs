use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::{debug, trace};

use crate::ast::program::{Declaration, Expression, Identifier, Statement};

pub fn resolve_declaration(
    declaration: &Declaration,
    variable_map: &mut HashMap<String, String>,
) -> Result<Declaration, String> {
    trace!("resolving declaration: {declaration:?}");

    if variable_map.contains_key(&declaration.name.value) {
        debug!("variable: {}", declaration);
        return Err("variable already declared".to_string());
    }

    let unique_name: String = temporary_name(&declaration.name.value);
    variable_map.insert(declaration.name.value.clone(), unique_name.clone());

    let init = declaration
        .initializer
        .as_ref()
        .map(|e| resolve_expression(e, variable_map)) // option result expr
        .transpose()?; // result expr

    // variable_map.insert(declaration.name.value.clone(), unique_name.clone());
    Ok(Declaration::new(Identifier::new(unique_name), init))
}

pub fn resolve_statement(
    statement: &Statement,
    variable_map: &HashMap<String, String>,
) -> Result<Statement, String> {
    use Statement::*;

    trace!("resolving statement: {statement:?}");

    let res = match statement {
        Return(expr) => Return(resolve_expression(expr, variable_map)?),
        Expression(expr) => Expression(resolve_expression(expr, variable_map)?),
        Null => Null,
    };

    Ok(res)
}

fn resolve_expression(
    expr: &Expression,
    variable_map: &HashMap<String, String>,
) -> Result<Expression, String> {
    use Expression::*;

    trace!("resolving expression: {expr:?}");

    let res = match expr {
        Assignment(left, right) => match **left {
            Var(_) => Assignment(
                Box::new(resolve_expression(left, variable_map)?),
                Box::new(resolve_expression(right, variable_map)?),
            ),
            _ => {
                return Err("invalid lvalue".to_string());
            }
        },
        Var(id) => {
            // debug!("variable_map: {:?}", variable_map);
            if let Some(v) = variable_map.get(&id.value) {
                Var(Identifier::new(v.clone()))
            } else {
                debug!("undeclared variable: {}", expr);
                return Err("undeclared variable".to_string());
            }
        }
        Unary(op, expr) => Unary(
            op.clone(),
            Box::new(resolve_expression(expr, variable_map)?),
        ),
        Binary(op, left, right) => Binary(
            op.clone(),
            Box::new(resolve_expression(left, variable_map)?),
            Box::new(resolve_expression(right, variable_map)?),
        ),
        Constant(c) => Constant(*c),
    };

    Ok(res)
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn temporary_name(name: &str) -> String {
    let id = next_id();
    format!("{name}.{id}")
}
