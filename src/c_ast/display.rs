use std::fmt;

use crate::{
    c_ast::ast::{
        BinaryOperator, Block, BlockItem, Declaration, Expression, FunctionDefinition, Program,
        Statement, UnaryOperator,
    },
    common::util::indent,
};

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
                        .0
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

#[allow(unused)]
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("to be implemented");
    }
}

impl fmt::Display for BlockItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockItem::S(s) => write!(f, "S({s})"),
            BlockItem::D(d) => write!(f, "D({d})"),
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Declaration(")?;
        writeln!(
            f,
            "{}",
            indent(&format!("name=\"{}\",", self.name.value), 4)
        )?;
        if let Some(v) = &self.initializer {
            writeln!(f, "{}", indent(&format!("value=\"{}\",", v.clone()), 4))?;
        }
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
            Statement::Expression(expr) => {
                writeln!(f, "Expression(")?;
                write!(f, "{}\n)", indent(&expr.to_string(), 4))
            }
            Statement::If(cond, then, else_) => {
                writeln!(f, "If(")?;
                if let Some(e) = else_ {
                    write!(
                        f,
                        "{}\n)",
                        indent(&format!("cond={cond}, then={then}, else={e}"), 4)
                    )?;
                } else {
                    write!(f, "{}\n)", indent(&format!("cond={cond}, then={then}"), 4))?;
                }
                writeln!(f, ")")
            }
            Statement::Compound(b) => write!(f, "Compound(\n{b}\n)"),
            Statement::Null => writeln!(f, "Null"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(c) => write!(f, "Constant({c})"),
            Expression::Unary(u, e) => write!(f, "Unary({u}, {e})"),
            Expression::Binary(op, exp_1, exp_2) => {
                write!(f, "Binary({op}, {exp_1}, {exp_2})")
            }
            Expression::Assignment(left, right) => write!(f, "Assignment({left}, {right})"),
            Expression::Var(id) => write!(f, "Var({})", id.value),
            Expression::Conditional(cond, then, el) => {
                write!(f, "Conditional({cond}, {then}, {el})")
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
