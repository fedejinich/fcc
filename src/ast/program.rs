use std::fmt;

use crate::util::indent;

// todo(fede) disable warnings for 'variables can be used directly in the `format!` string'

pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Vec<Statement>,
}

pub struct Identifier {
    pub value: String, //  todo(fede) this is still weird as fuck
}

#[derive(Clone)]
pub enum Statement {
    Return(Expression),
}

#[derive(Clone)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
}

#[derive(Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program(")?;
        write!(f, "{}\n)", indent(&self.function_definition.to_string(), 4))
    }
}

impl fmt::Display for FunctionDefinition {
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr) => {
                writeln!(f, "Return(")?;
                write!(f, "{}\n)", indent(&expr.to_string(), 4))
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(c) => write!(f, "Constant({})", c),
            Expression::Unary(u, e) => write!(f, "Unary({}, {})", u, e),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Complement => write!(f, "Complement"),
            UnaryOperator::Negate => write!(f, "Negate"),
        }
    }
}
