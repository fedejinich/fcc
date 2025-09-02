//! This module defines the structure of the x64 assembly code as an AST

use log::debug;

use crate::tacky::program::{
    TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram,
    TackyUnaryOperator, TackyValue,
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
    Binary(AsmBinaryOperator, AsmOperand, AsmOperand),
    Cmp(AsmOperand, AsmOperand),
    Idiv(AsmOperand),
    Cdq,
    Jmp(AsmIdetifier),
    JmpCC(AsmCondCode, AsmIdetifier),
    SetCC(AsmCondCode, AsmOperand),
    Label(AsmIdetifier),
    AllocateStack(i32),
    Ret,
}

#[derive(Clone, Debug)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[derive(Clone, Debug)]
pub enum AsmBinaryOperator {
    Add,
    Sub,
    Mult,

    // bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
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
    DX,
    CX,
    CL,
    R10,
    R11,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AsmCondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
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
                // TODO: remove clone
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
            TackyInstruction::Unary(TackyUnaryOperator::Not, src, dst) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(src)),
                AsmInstruction::Mov(AsmOperand::Imm(0), AsmOperand::from(dst.clone())),
                AsmInstruction::SetCC(AsmCondCode::E, AsmOperand::from(dst)),
            ],
            TackyInstruction::Unary(unary_op, src, dst) => vec![
                AsmInstruction::Mov(AsmOperand::from(src), AsmOperand::from(dst.clone())),
                AsmInstruction::Unary(AsmUnaryOperator::from(unary_op), AsmOperand::from(dst)),
            ],
            TackyInstruction::Binary(op, src_1, src_2, dst) => {
                let is_div = op == TackyBinaryOperator::Divide;
                match &op {
                    // “addition, subtraction, and multiplication,
                    // we convert a single TACKY instruction into two assembly instructions”
                    TackyBinaryOperator::Add
                    | TackyBinaryOperator::Subtract
                    | TackyBinaryOperator::Multiply
                    // bitwise operators
                    | TackyBinaryOperator::BitwiseAnd
                    | TackyBinaryOperator::BitwiseOr
                    | TackyBinaryOperator::BitwiseXor
                    | TackyBinaryOperator::LeftShift
                    | TackyBinaryOperator::RightShift => vec![
                        AsmInstruction::Mov(AsmOperand::from(src_1), AsmOperand::from(dst.clone())),
                        AsmInstruction::Binary(
                            AsmBinaryOperator::from(op),
                            AsmOperand::from(src_2),
                            AsmOperand::from(dst),
                        ),
                    ],
                    // relational operators
                    TackyBinaryOperator::Equal 
                    | TackyBinaryOperator::NotEqual | TackyBinaryOperator::GreaterThan
                    | TackyBinaryOperator::LessThan | TackyBinaryOperator::LessThanOrEqual 
                    | TackyBinaryOperator::GreaterThanOrEqual => vec![
                        AsmInstruction::Cmp(AsmOperand::from(src_2), AsmOperand::from(src_1)),
                        AsmInstruction::Mov(AsmOperand::Imm(0), AsmOperand::from(dst.clone())),
                        AsmInstruction::SetCC(AsmCondCode::from(op), AsmOperand::from(dst)),
                    ],
                    // logical operators
                    TackyBinaryOperator::And | TackyBinaryOperator::Or => todo!(),
                    TackyBinaryOperator::Divide | TackyBinaryOperator::Remainder => {
                        let reg = if is_div {
                            debug!("is div");
                            AsmOperand::Register(Reg::AX)
                        } else {
                            debug!("is rem");
                            AsmOperand::Register(Reg::DX)
                        };

                        vec![
                            AsmInstruction::Mov(
                                AsmOperand::from(src_1),
                                AsmOperand::Register(Reg::AX),
                            ),
                            AsmInstruction::Cdq,
                            AsmInstruction::Idiv(AsmOperand::from(src_2)),
                            AsmInstruction::Mov(reg, AsmOperand::from(dst)),
                        ]
                    }
                }
            }
            TackyInstruction::Jump(id) => vec![AsmInstruction::Jmp(AsmIdetifier::from(id))],
            // TODO: this is almost same as JumpIfNotZero
            TackyInstruction::JumpIfZero(condition, target) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(condition)),
                AsmInstruction::JmpCC(AsmCondCode::E, AsmIdetifier::from(target)),
            ],
            TackyInstruction::JumpIfNotZero(condition, target) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(condition)),
                AsmInstruction::JmpCC(AsmCondCode::NE, AsmIdetifier::from(target)),
            ],
            TackyInstruction::Copy(src, dst) => vec![
                AsmInstruction::Mov(AsmOperand::from(src), AsmOperand::from(dst)),
            ],
            TackyInstruction::Label(id) => vec![AsmInstruction::Label(AsmIdetifier::from(id))],
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
            // logical unary operators
            TackyUnaryOperator::Not => AsmUnaryOperator::Not,
        }
    }
}

impl From<TackyBinaryOperator> for AsmBinaryOperator {
    fn from(tacky_binary_operator: TackyBinaryOperator) -> Self {
        match tacky_binary_operator {
            TackyBinaryOperator::Add => AsmBinaryOperator::Add,
            TackyBinaryOperator::Subtract => AsmBinaryOperator::Sub,
            TackyBinaryOperator::Multiply => AsmBinaryOperator::Mult,
            TackyBinaryOperator::BitwiseAnd => AsmBinaryOperator::BitwiseAnd,
            TackyBinaryOperator::BitwiseOr => AsmBinaryOperator::BitwiseOr,
            TackyBinaryOperator::BitwiseXor => AsmBinaryOperator::BitwiseXor,
            TackyBinaryOperator::LeftShift => AsmBinaryOperator::LeftShift,
            TackyBinaryOperator::RightShift => AsmBinaryOperator::RightShift,
            _ => {
                debug!("{tacky_binary_operator:?}");
                panic!("this should never happen")
            }
        }
    }
}

impl From<TackyBinaryOperator> for AsmCondCode {
    fn from(op: TackyBinaryOperator) -> Self {
        match op {
            TackyBinaryOperator::Equal => AsmCondCode::E,
            TackyBinaryOperator::NotEqual => AsmCondCode::NE,
            TackyBinaryOperator::GreaterThan => AsmCondCode::G,
            TackyBinaryOperator::LessThan => AsmCondCode::L,
            TackyBinaryOperator::GreaterThanOrEqual => AsmCondCode::GE,
            TackyBinaryOperator::LessThanOrEqual => AsmCondCode::LE,
            _ => {
                debug!("{op:?}");
                panic!("this should never happen")
            }
        }
    }
}
