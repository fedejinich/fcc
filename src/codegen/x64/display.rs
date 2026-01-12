//! Pretty print implementation for Assembly AST.
//!
//! Provides debug-oriented representation showing the internal structure
//! with pseudo-registers and unexpanded instructions.
//!
//! Uses `pretty_print()` methods to avoid conflicts with Display impls
//! in emit.rs that generate actual assembly syntax.

use crate::common::util::indent;

use super::ast::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram,
    AsmUnaryOperator, Reg,
};

impl AsmProgram {
    pub fn pretty_print(&self) -> String {
        format!(
            "AsmProgram(\n{}\n)",
            indent(&self.function_definition.pretty_print(), 4)
        )
    }
}

impl AsmFunctionDefinition {
    pub fn pretty_print(&self) -> String {
        let instructions_str = self
            .instructions
            .iter()
            .map(|ins| indent(&ins.pretty_print(), 8))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "AsmFunction(\n    name = \"{}\",\n    instructions = [\n{}\n    ]\n)",
            self.name.value, instructions_str
        )
    }
}

impl AsmInstruction {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::Comment(s) => format!("Comment(\"{}\")", s),
            Self::Mov(src, dst) => {
                format!("Mov({}, {})", src.pretty_print(), dst.pretty_print())
            }
            Self::Unary(op, operand) => {
                format!("Unary({}, {})", op.pretty_print(), operand.pretty_print())
            }
            Self::Binary(op, src, dst) => format!(
                "Binary({}, {}, {})",
                op.pretty_print(),
                src.pretty_print(),
                dst.pretty_print()
            ),
            Self::Cmp(op1, op2) => {
                format!("Cmp({}, {})", op1.pretty_print(), op2.pretty_print())
            }
            Self::Idiv(operand) => format!("Idiv({})", operand.pretty_print()),
            Self::Cdq => "Cdq".to_string(),
            Self::Jmp(label) => format!("Jmp({})", label.value),
            Self::JmpCC(cc, label) => format!("JmpCC({}, {})", cc.pretty_print(), label.value),
            Self::SetCC(cc, operand) => {
                format!("SetCC({}, {})", cc.pretty_print(), operand.pretty_print())
            }
            Self::Label(label) => format!("Label({})", label.value),
            Self::AllocateStack(n) => format!("AllocateStack({})", n),
            Self::Ret => "Ret".to_string(),
        }
    }
}

impl AsmOperand {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::Imm(n) => format!("Imm({})", n),
            Self::Register(reg) => format!("Reg({})", reg.pretty_print()),
            Self::Pseudo(name) => format!("Pseudo(\"{}\")", name.value),
            Self::Stack(offset) => format!("Stack({})", offset),
        }
    }
}

impl Reg {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::AX => "AX".to_string(),
            Self::DX => "DX".to_string(),
            Self::CX => "CX".to_string(),
            Self::CL => "CL".to_string(),
            Self::R10 => "R10".to_string(),
            Self::R11 => "R11".to_string(),
        }
    }
}

impl AsmUnaryOperator {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::Neg => "Neg".to_string(),
            Self::Not => "Not".to_string(),
        }
    }
}

impl AsmBinaryOperator {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::Add => "Add".to_string(),
            Self::Sub => "Sub".to_string(),
            Self::Mult => "Mult".to_string(),
            Self::BitwiseAnd => "BitwiseAnd".to_string(),
            Self::BitwiseOr => "BitwiseOr".to_string(),
            Self::BitwiseXor => "BitwiseXor".to_string(),
            Self::LeftShift => "LeftShift".to_string(),
            Self::RightShift => "RightShift".to_string(),
        }
    }
}

impl AsmCondCode {
    pub fn pretty_print(&self) -> String {
        match self {
            Self::E => "E".to_string(),
            Self::NE => "NE".to_string(),
            Self::G => "G".to_string(),
            Self::GE => "GE".to_string(),
            Self::L => "L".to_string(),
            Self::LE => "LE".to_string(),
        }
    }
}
