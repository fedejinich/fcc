//! This module contains  tacky AST which is an intermediate
//! representation of the source code.

use std::sync::atomic::{AtomicUsize, Ordering};

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
    Comment(String),
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
    pub fn new(value: &str) -> TackyIdentifier {
        TackyIdentifier {
            value: format!("{}.{}", value, next_id()),
        }
    }
}

impl TackyProgram {
    pub fn new(function_definition: TackyFunctionDefinition) -> Self {
        TackyProgram {
            function_definition,
        }
    }
}

impl TackyFunctionDefinition {
    pub fn new(name: TackyIdentifier, instructions: Vec<TackyInstruction>) -> Self {
        TackyFunctionDefinition { name, instructions }
    }
}
