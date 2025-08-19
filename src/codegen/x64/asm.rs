//! This module defines the structure of the x64 assembly code as an AST

use crate::tacky::program::{
    TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram, TackyUnaryOperator,
    TackyValue,
};

#[derive(Clone)]
pub struct AsmProgram {
    pub function_definition: AsmFunctionDefinition,
}

#[derive(Clone)]
pub struct AsmFunctionDefinition {
    pub name: AsmIdetifier,
    pub instructions: Vec<AsmInstruction>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AsmIdetifier {
    pub value: String,
}

#[derive(Clone)]
pub enum AsmInstruction {
    Comment(String),
    Mov(AsmOperand, AsmOperand),
    Unary(AsmUnaryOperator, AsmOperand),
    AllocateStack(i32),
    Ret,
}

#[derive(Clone, Debug)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AsmOperand {
    Imm(i32),
    Register(Reg),
    Pseudo(AsmIdetifier),
    Stack(i32),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Reg {
    AX,
    R10,
}

impl AsmProgram {
    pub fn new(function_definition: AsmFunctionDefinition) -> Self {
        AsmProgram {
            function_definition,
        }
    }

    pub fn code_emit(&self) -> String {
        self.to_string_asm()
    }
}

impl AsmFunctionDefinition {
    pub fn new(name: AsmIdetifier, instructions: Vec<AsmInstruction>) -> Self {
        AsmFunctionDefinition { name, instructions }
    }
}

impl From<TackyProgram> for AsmProgram {
    fn from(tacky_program: TackyProgram) -> Self {
        AsmProgram {
            function_definition: AsmFunctionDefinition::from(tacky_program.function_definition),
        }
    }
}

impl From<TackyFunctionDefinition> for AsmFunctionDefinition {
    fn from(tacky_function_definition: TackyFunctionDefinition) -> Self {
        AsmFunctionDefinition {
            name: AsmIdetifier::from(tacky_function_definition.name),
            instructions: tacky_function_definition
                .instructions
                .iter()
                // todo(fede) remove clone
                .flat_map(|i| AsmInstruction::from(i.clone()))
                .collect::<Vec<AsmInstruction>>(),
        }
    }
}

impl AsmInstruction {
    fn from(tacky_instruction: TackyInstruction) -> Vec<AsmInstruction> {
        match tacky_instruction {
            TackyInstruction::Return(val) => vec![
                AsmInstruction::Mov(AsmOperand::from(val), AsmOperand::Register(Reg::AX)),
                AsmInstruction::Ret,
            ],
            TackyInstruction::Unary(unary_op, src, dst) => vec![
                AsmInstruction::Mov(AsmOperand::from(src), AsmOperand::from(dst.clone())),
                AsmInstruction::Unary(AsmUnaryOperator::from(unary_op), AsmOperand::from(dst)),
            ],
        }
    }
}

impl From<TackyValue> for AsmOperand {
    fn from(tacky_value: TackyValue) -> Self {
        match tacky_value {
            TackyValue::Constant(c) => AsmOperand::Imm(c),
            TackyValue::Var(id) => AsmOperand::Pseudo(AsmIdetifier::from(id)),
        }
    }
}

impl From<TackyIdentifier> for AsmIdetifier {
    fn from(tacky_identifier: TackyIdentifier) -> Self {
        AsmIdetifier {
            value: tacky_identifier.value,
        }
    }
}

impl From<TackyUnaryOperator> for AsmUnaryOperator {
    fn from(tacky_unary_operator: TackyUnaryOperator) -> Self {
        match tacky_unary_operator {
            TackyUnaryOperator::Negate => AsmUnaryOperator::Neg,
            TackyUnaryOperator::Complement => AsmUnaryOperator::Not,
        }
    }
}
