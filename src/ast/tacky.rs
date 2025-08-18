#![allow(warnings)]

use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    ast::c::{CFunctionDefinition, CIdentifier, CProgram, CStatement},
    util::indent,
};

use super::c::{CExpression, CUnaryOperator};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub struct TackyProgram {
    pub function_definition: TackyFunctionDefinition,
}

#[derive(Clone)]
pub struct TackyFunctionDefinition {
    pub name: TackyIdentifier,
    pub instructions: Vec<TackyInstruction>,
}

#[derive(Clone)]
pub enum TackyInstruction {
    Return(TackyValue),
    Unary(TackyUnaryOperator, TackyValue, TackyValue),
}

#[derive(Clone)]
pub struct TackyIdentifier {
    pub value: String,
}

impl TackyIdentifier {
    fn new(desc: &str) -> TackyIdentifier {
        TackyIdentifier {
            value: format!("{}.{}", desc, next_id()),
        }
    }
}

#[derive(Clone)]
pub enum TackyValue {
    Constant(i32),
    Var(TackyIdentifier),
}

#[derive(Clone)]
pub enum TackyUnaryOperator {
    Complement,
    Negate,
}

impl From<CProgram> for TackyProgram {
    fn from(program: CProgram) -> Self {
        TackyProgram {
            function_definition: TackyFunctionDefinition::from(program.function_definition),
        }
    }
}

impl From<CFunctionDefinition> for TackyFunctionDefinition {
    fn from(function_definition: CFunctionDefinition) -> Self {
        let instructions = function_definition
            .body
            .into_iter()
            .flat_map(|s| TackyInstruction::from(s))
            .collect();

        TackyFunctionDefinition {
            name: TackyIdentifier::from(function_definition.name),
            instructions,
        }
    }
}

impl From<CIdentifier> for TackyIdentifier {
    fn from(value: CIdentifier) -> Self {
        TackyIdentifier { value: value.value }
    }
}

impl TackyInstruction {
    fn from(statement: CStatement) -> Vec<TackyInstruction> {
        let mut instructions = vec![];
        match statement {
            CStatement::Return(expr) => {
                let _ = TackyInstruction::from_expr(expr, &mut instructions);
                instructions
            }
        }
    }

    fn from_expr(expr: CExpression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        match expr {
            CExpression::Constant(c) => TackyValue::Constant(c),
            CExpression::Unary(op, inner_exp) => {
                let src = TackyInstruction::from_expr(*inner_exp, instructions);
                // todo(fede) provide a more descriptive name
                let dst = TackyValue::Var(TackyIdentifier::new("tmp"));
                let unary_op = TackyUnaryOperator::from(op);

                // todo(fede) this is a clone(hack?)
                instructions.push(TackyInstruction::Unary(unary_op, src, dst.clone()));

                dst
            }
        }
    }
}

impl From<CUnaryOperator> for TackyUnaryOperator {
    fn from(op: CUnaryOperator) -> Self {
        match op {
            CUnaryOperator::Complement => TackyUnaryOperator::Complement,
            CUnaryOperator::Negate => TackyUnaryOperator::Negate,
        }
    }
}

impl fmt::Display for TackyProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TackyProgram(")?;
        writeln!(f, "{})", indent(&self.function_definition.to_string(), 4));
        writeln!(f, ")")
    }
}

impl fmt::Display for TackyFunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TackyFunction(")?;
        writeln!(
            f,
            "{}",
            indent(&format!("name=\"{}\",", self.name.value), 4)
        )?;
        writeln!(
            f,
            "{}",
            indent(
                &format!(
                    "instructions=[\n{}\n]",
                    self.instructions
                        .clone()
                        .into_iter()
                        .map(|s| indent(&s.to_string(), 4))
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                4
            )
        )?;
        writeln!(f, "")
    }
}

impl fmt::Display for TackyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TackyInstruction::Return(value) => {
                writeln!(f, "Return(")?;
                writeln!(f, "{}", indent(&value.to_string(), 4));
                writeln!(f, ")")
            }
            TackyInstruction::Unary(op, src, dst) => {
                writeln!(f, "Unary({}, {}, {})", op, src, dst)
                // writeln!(f, "Unary(")?;
                // writeln!(f, "{},", indent(&op.to_string(), 4))?;
                // writeln!(f, "{},", indent(&src.to_string(), 4))?;
                // writeln!(f, "{}", indent(&dst.to_string(), 4))?;
                // writeln!(f, ")")
            }
        }
    }
}

impl fmt::Display for TackyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TackyValue::Constant(c) => write!(f, "Constant({})", c),
            TackyValue::Var(id) => write!(f, "Var({})", id.value),
        }
    }
}

impl fmt::Display for TackyUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // todo(fede) this might be replaced with derive debug
            TackyUnaryOperator::Complement => write!(f, "Complement"),
            TackyUnaryOperator::Negate => write!(f, "Negate"),
        }
    }
}
