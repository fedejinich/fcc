use std::fmt;

use crate::util::indent;

// TODO: disable warnings for 'variables can be used directly in the `format!` string'

pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Vec<Statement>,
}

pub struct Identifier {
    pub value: String, //  TODO: this is still weird as fuck
}

#[derive(Clone, Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    // bitwise operators are binary operators as well
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // logical operators
    And,
    Or,

    // relational operators
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    // wip
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
            Expression::Binary(op, exp_1, exp_2) => {
                write!(f, "Binary({}, {}, {})", op, exp_1, exp_2)
            }
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Complement => write!(f, "Complement"),
            UnaryOperator::Negate => write!(f, "Negate"),
            UnaryOperator::Not => write!(f, "Not"),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "Add"),
            BinaryOperator::Subtract => write!(f, "Subtract"),
            BinaryOperator::Multiply => write!(f, "Multiply"),
            BinaryOperator::Divide => write!(f, "Divide"),
            BinaryOperator::Remainder => write!(f, "Remainder"),
            BinaryOperator::BitwiseAnd => write!(f, "BitwiseAnd"),
            BinaryOperator::BitwiseOr => write!(f, "BitwiseOr"),
            BinaryOperator::BitwiseXor => write!(f, "BitwiseXor"),
            BinaryOperator::LeftShift => write!(f, "LeftShift"),
            BinaryOperator::RightShift => write!(f, "RightShift"),
            BinaryOperator::And => write!(f, "And"),
            BinaryOperator::Or => write!(f, "Or"),
            BinaryOperator::Equal => write!(f, "Equal"),
            BinaryOperator::NotEqual => write!(f, "NotEqual"),
            BinaryOperator::GreaterThan => write!(f, "GreaterThan"),
            BinaryOperator::LessThan => write!(f, "LessThan"),
            BinaryOperator::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            BinaryOperator::LessThanOrEqual => write!(f, "LessThanOrEqual"),
        }
    }
}
