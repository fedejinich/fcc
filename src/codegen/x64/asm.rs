//! This module defines the structure of the x64 assembly code as an AST

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

#[derive(Clone, Debug)]
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
