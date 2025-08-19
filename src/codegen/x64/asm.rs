use crate::util::indent;

use crate::tacky::{
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
        self.function_definition.code_emit()
    }
}

impl AsmFunctionDefinition {
    pub fn new(name: AsmIdetifier, instructions: Vec<AsmInstruction>) -> Self {
        AsmFunctionDefinition { name, instructions }
    }

    pub fn code_emit(&self) -> String {
        let instructions = self
            .instructions
            .iter()
            .map(|i| i.code_emit())
            .collect::<Vec<String>>()
            .join("\n");

        [
            indent(format!(".globl _{}", self.name.value).as_str(), 4),
            format!("\n_{}:", self.name.value),
            indent("pushq %rbp", 4),
            indent("movq %rsp, %rbp", 4),
            indent(instructions.as_str(), 4),
        ]
        .join("\n")
    }
}

impl AsmInstruction {
    pub fn code_emit(&self) -> String {
        match self {
            AsmInstruction::Comment(s) => format!("# {s}\n"),
            AsmInstruction::Mov(src, dst) => {
                format!("movl {}, {}\n", src.code_emit(), dst.code_emit())
            }
            AsmInstruction::Ret => ["movq %rbp, %rsp", "popq %rbp", "ret"].join("\n"),
            AsmInstruction::Unary(unary_op, op) => {
                format!("{} {}", unary_op.code_emit(), op.code_emit())
            }
            AsmInstruction::AllocateStack(val) => format!("subq ${val}, %rsp"),
        }
    }
}

impl AsmUnaryOperator {
    fn code_emit(&self) -> String {
        match self {
            AsmUnaryOperator::Neg => "negl".to_string(),
            AsmUnaryOperator::Not => "notl".to_string(),
        }
    }
}

impl AsmOperand {
    fn code_emit(&self) -> String {
        match self {
            AsmOperand::Register(Reg::AX) => "%eax".to_string(),
            AsmOperand::Register(Reg::R10) => "%r10d".to_string(),
            AsmOperand::Stack(val) => format!("{val}(%rbp)"),
            AsmOperand::Imm(num) => format!("${num}"),
            _ => panic!("invalid operand"),
        }
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
