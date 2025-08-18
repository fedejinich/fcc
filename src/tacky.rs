#![allow(warnings)]

use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::debug;

use crate::{
    ast::{CFunctionDefinition, CIdentifier, CProgram, CStatement},
    util::indent,
};

use super::ast::{CExpression, CUnaryOperator};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug)]
pub struct TackyProgram {
    pub function_definition: TackyFunctionDefinition,
}

#[derive(Clone, Debug)]
pub struct TackyFunctionDefinition {
    pub name: TackyIdentifier,
    pub instructions: Vec<TackyInstruction>,
}

#[derive(Clone, Debug)]
pub enum TackyInstruction {
    Return(TackyValue),
    Unary(TackyUnaryOperator, TackyValue, TackyValue),
}

#[derive(Clone, Debug)]
pub struct TackyIdentifier {
    pub value: String,
}

impl TackyIdentifier {
    pub fn new(desc: &str) -> TackyIdentifier {
        TackyIdentifier {
            value: format!("{}.{}", desc, next_id()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TackyValue {
    Constant(i32),
    Var(TackyIdentifier),
}

#[derive(Clone, Debug)]
pub enum TackyUnaryOperator {
    Complement,
    Negate,
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
