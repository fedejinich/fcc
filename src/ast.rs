use log::{debug, trace};
use std::{fmt, slice::Iter};

use crate::{lexer::Token, util::indent};

// todo(fede) disable warnings for 'variables can be used directly in the `format!` string'

pub struct CProgram {
    pub function_definition: CFunctionDefinition,
}

pub struct CFunctionDefinition {
    pub name: CIdentifier,
    pub body: Vec<CStatement>,
}

pub struct CIdentifier {
    pub value: String, //  todo(fede) this is still weird as fuck
}

#[derive(Clone)]
pub enum CStatement {
    Return(CExpression),
}

#[derive(Clone)]
pub enum CExpression {
    Constant(i32),
    Unary(CUnaryOperator, Box<CExpression>),
}

#[derive(Clone)]
pub enum CUnaryOperator {
    Complement,
    Negate,
}

impl fmt::Display for CProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program(")?;
        write!(f, "{}\n)", indent(&self.function_definition.to_string(), 4))
    }
}

impl fmt::Display for CFunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function(")?;
        writeln!(
            f,
            "{}",
            indent(&format!("name=\"{}\",", self.name.value), 4)
        )?;
        write!(
            f,
            "{}",
            indent(
                &format!(
                    "body={}",
                    self.body
                        .clone()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                4
            )
        )?;
        Ok(())
    }
}

impl fmt::Display for CStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CStatement::Return(expr) => {
                writeln!(f, "Return(")?;
                write!(f, "{}\n)", indent(&expr.to_string(), 4))
            }
        }
    }
}

impl fmt::Display for CExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CExpression::Constant(c) => write!(f, "Constant({})", c),
            CExpression::Unary(u, e) => write!(f, "Unary({}, {})", u, e),
        }
    }
}

impl fmt::Display for CUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CUnaryOperator::Complement => write!(f, "Complement"),
            CUnaryOperator::Negate => write!(f, "Negate"),
        }
    }
}
