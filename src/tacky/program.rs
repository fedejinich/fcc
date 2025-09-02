//! This module contains  tacky AST which is an intermediate
//! representation of the source code.

use std::{
    fmt::format,
    sync::atomic::{AtomicUsize, Ordering},
};

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
    Copy(TackyValue, TackyValue),
    Jump(TackyIdentifier),
    JumpIfZero(TackyValue, TackyIdentifier),
    JumpIfNotZero(TackyValue, TackyIdentifier),
    Label(TackyIdentifier),
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
    // logical unary operators
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TackyBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    // bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    // relational operators
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
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
            TackyInstruction::Unary(op, src, dst) => format!(
                "Unary({}, {}, {})",
                op.pretty_print(),
                src.pretty_print(),
                dst.pretty_print()
            ),
            TackyInstruction::Binary(op, src_1, src_2, dst) => format!(
                "Binary({}, {}, {}, {})",
                op.pretty_print(),
                src_1.pretty_print(),
                src_2.pretty_print(),
                dst.pretty_print()
            ),
            TackyInstruction::Copy(src, dst) => {
                format!("Copy({}, {})", src.pretty_print(), dst.pretty_print())
            }
            TackyInstruction::Jump(id) => format!("Jump({})", id.value),
            TackyInstruction::JumpIfZero(val, id) => {
                format!("JumpIfZero({}, {})", val.pretty_print(), id.value)
            }
            TackyInstruction::JumpIfNotZero(val, id) => {
                format!("JumpIfNotZero({}, {})", val.pretty_print(), id.value)
            }
            TackyInstruction::Label(id) => format!("Label({})", id.value),
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
            TackyUnaryOperator::Complement => "Complement",
            TackyUnaryOperator::Negate => "Negate",
            // logical unary operators
            TackyUnaryOperator::Not => "Not",
        }
        .to_string()
    }
}

impl TackyBinaryOperator {
    pub fn pretty_print(&self) -> String {
        match self {
            TackyBinaryOperator::Add => "Add",
            TackyBinaryOperator::Subtract => "Subtract",
            TackyBinaryOperator::Multiply => "Multiply",
            TackyBinaryOperator::Divide => "Divide",
            TackyBinaryOperator::Remainder => "Remainder",
            TackyBinaryOperator::BitwiseAnd => "BitwiseAnd",
            TackyBinaryOperator::BitwiseOr => "BitwiseOr",
            TackyBinaryOperator::BitwiseXor => "BitwiseXor",
            TackyBinaryOperator::LeftShift => "LeftShift",
            TackyBinaryOperator::RightShift => "RightShift",
            TackyBinaryOperator::Equal => "Equal",
            TackyBinaryOperator::NotEqual => "NotEqual",
            TackyBinaryOperator::GreaterThan => "GreaterThan",
            TackyBinaryOperator::LessThan => "LessThan",
            TackyBinaryOperator::GreaterThanOrEqual => "GreaterThanOrEqual",
            TackyBinaryOperator::LessThanOrEqual => "LessThanOrEqual",
        }
        .to_string()
    }
}
