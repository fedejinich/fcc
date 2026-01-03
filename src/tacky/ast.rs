//! This module contains  tacky AST which is an intermediate
//! representation of the source code.

use std::sync::atomic::AtomicUsize;

use crate::{c_ast::ast::Identifier, common::util::temporary_name};

pub struct TackyProgram {
    pub function_definition: TackyFunctionDefinition,
}

#[derive(Clone, Debug)]
pub struct TackyFunctionDefinition {
    pub name: TackyIdentifier,
    pub instructions: Vec<TackyInstruction>,
}

#[allow(dead_code)]
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

impl TackyIdentifier {
    /// Creates a new identifier with the given value
    /// note: it increments the counter
    pub fn new(value: &str) -> TackyIdentifier {
        TackyIdentifier {
            value: temporary_name(value, &COUNTER),
        }
    }

    /// Creates a new identifier with the given prefix and the label's value
    /// note: it does not increment the counter
    pub fn with_prefix(prefix: &str, label: Identifier) -> TackyIdentifier {
        let id = prefix.to_string() + &label.value();
        TackyIdentifier { value: id }
        // Self::new(&id)
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
