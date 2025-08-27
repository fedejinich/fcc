//! This module contains  tacky AST which is an intermediate
//! representation of the source code.

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::util::indent;

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
    Binary(TackyBinaryOperator, TackyValue, TackyValue, TackyValue),
}

#[derive(Clone, Debug)]
pub struct TackyIdentifier {
    pub value: String,
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

#[derive(Clone, Debug, PartialEq)]
pub enum TackyBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

impl TackyIdentifier {
    pub fn new(desc: &str) -> TackyIdentifier {
        TackyIdentifier {
            value: format!("{}.{}", desc, next_id()),
        }
    }
}

impl TackyProgram {
    pub fn pretty_print(&self) -> String {
        format!(
            "TackyProgram(\n{}\n)",
            indent(&self.function_definition.pretty_print(), 4)
        )
    }
}

impl TackyFunctionDefinition {
    pub fn pretty_print(&self) -> String {
        format!(
            "TackyFunction(\n{}\n{}\n)",
            indent(&format!("name=\"{}\",", self.name.value), 4),
            indent(
                &format!(
                    "instructions=[\n{}\n]",
                    self.instructions
                        .iter()
                        .map(|s| indent(&s.pretty_print(), 4))
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                4
            )
        )
    }
}

impl TackyInstruction {
    pub fn pretty_print(&self) -> String {
        match self {
            TackyInstruction::Return(value) => {
                format!("Return(\n{}\n)", indent(&value.pretty_print(), 4))
            }
            TackyInstruction::Unary(op, src, dst) => {
                format!(
                    "Unary({}, {}, {})",
                    op.pretty_print(),
                    src.pretty_print(),
                    dst.pretty_print()
                )
            }
            TackyInstruction::Binary(_, _, _, _) => todo!(),
        }
    }
}

impl TackyValue {
    pub fn pretty_print(&self) -> String {
        match self {
            TackyValue::Constant(c) => format!("Constant({})", c),
            TackyValue::Var(id) => format!("Var(\"{}\")", id.value),
        }
    }
}

impl TackyUnaryOperator {
    pub fn pretty_print(&self) -> String {
        match self {
            // TODO: this might be replaced with derive debug
            TackyUnaryOperator::Complement => "Complement".to_string(),
            TackyUnaryOperator::Negate => "Negate".to_string(),
        }
    }
}
