//! This module defines the structure of the x64 assembly code as an AST

#[derive(Clone)]
pub struct AsmProgram {
    pub function_definition: AsmFunctionDefinition,
}

#[derive(Clone)]
pub struct AsmFunctionDefinition {
    pub name: AsmIdentifier,
    pub instructions: Vec<AsmInstruction>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AsmIdentifier {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmInstruction {
    Comment(String),
    Mov(AsmOperand, AsmOperand),
    Unary(AsmUnaryOperator, AsmOperand),
    Binary(AsmBinaryOperator, AsmOperand, AsmOperand),
    Cmp(AsmOperand, AsmOperand),
    Idiv(AsmOperand),
    Cdq,
    Jmp(AsmIdentifier),
    JmpCC(AsmCondCode, AsmIdentifier),
    SetCC(AsmCondCode, AsmOperand),
    Label(AsmIdentifier),
    AllocateStack(i32),
    Ret,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[derive(Clone, Debug, PartialEq)]
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
    Pseudo(AsmIdentifier),
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
}

impl AsmFunctionDefinition {
    pub fn new(name: AsmIdentifier, instructions: Vec<AsmInstruction>) -> Self {
        AsmFunctionDefinition { name, instructions }
    }
}
