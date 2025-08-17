use crate::ast::program::{CExpression, CFunctionDefinition, CProgram, CStatement};
use crate::util::indent;

use super::program::CIdentifier;
use super::tacky::{
    TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram, TackyUnaryOperator,
    TackyValue,
};

pub struct AsmProgram {
    function_definition: AsmFunctionDefinition,
}

pub struct AsmFunctionDefinition {
    name: AsmIdetifier,
    instructions: Vec<AsmInstruction>,
}

pub struct AsmIdetifier {
    value: String,
}

pub enum AsmInstruction {
    Comment(String),
    Mov(AsmOperand, AsmOperand),
    Unary(AsmUnaryOperator, AsmOperand),
    AllocateStack(i32),
    Ret,
}

pub enum AsmUnaryOperator {
    Neg,
    Not,
}

pub enum AsmOperand {
    Imm(i32),
    Register(Reg),
    Pseudo(AsmIdetifier),
    Stack(i32),
}

pub enum Reg {
    AX,
    R10,
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

// impl From<TackyInstruction> for AsmInstruction {
//     fn from(tacky_instruction: TackyInstruction) -> Self {
//         match tacky_instruction {
//             TackyInstruction::Unary(op, src, dst) => {
//                 AsmInstruction::Unary(AsmUnaryOperator::from(op), AsmOperand::from(src), AsmOperand::from(dst))
//             }
//         }
//     }
// }
//
// impl From<TackyUnaryOperator> for AsmUnaryOperator {
//     fn from(tacky_unary_operator: TackyUnaryOperator) -> Self {
//         match tacky_unary_operator {
//             TackyUnaryOperator::Negate => AsmUnaryOperator::Neg,
//             TackyUnaryOperator::Complement => AsmUnaryOperator::Not,
//         }
//     }
// }
//
// impl From<TackyValue> for AsmOperand {
//     fn from(tacky_value: TackyValue) -> Self {
//         match tacky_value {
//             TackyValue::Constant(c) => AsmOperand::Imm(c),
//             TackyValue::Var

// impl AsmProgram {
//     pub fn code_emit(&self) -> String {
//         self.function_definition.code_emit()
//     }
// }
//
// impl AsmFunctionDefinition {
//     pub fn code_emit(&self) -> String {
//         let instructions = self
//             .instructions
//             .iter()
//             .map(|i| i.code_emit())
//             .collect::<String>();
//
//         format!(
//             ".globl _{}\n{}\n{}",
//             self.name.value,
//             format!("\n_{}:", self.name.value).as_str(),
//             indent(instructions.as_str(), 4)
//         )
//     }
// }
//
// impl AsmInstruction {
//     pub fn code_emit(&self) -> String {
//         match self {
//             AsmInstruction::Comment(s) => format!("# {s}\n"),
//             AsmInstruction::Mov(src, dst) => {
//                 format!("mov {}, {}\n", src.code_emit(), dst.code_emit())
//             }
//             AsmInstruction::Ret => "ret\n".to_string(),
//         }
//     }
// }
//
// impl AsmOperand {
//     fn code_emit(&self) -> String {
//         match self {
//             AsmOperand::Register => "%eax".to_string(),
//             AsmOperand::Imm(num) => {
//                 format!("${num}")
//             }
//         }
//     }
// }
//
// impl AsmInstruction {
//     fn from(c_statement: CStatement) -> Vec<AsmInstruction> {
//         match c_statement {
//             CStatement::Return(exp) => vec![
//                 AsmInstruction::Comment("return statement".to_string()),
//                 AsmInstruction::Mov(AsmOperand::from(exp), AsmOperand::Register),
//                 AsmInstruction::Ret,
//             ],
//         }
//     }
// }
//
// impl From<CProgram> for AsmProgram {
//     fn from(c_program: CProgram) -> Self {
//         AsmProgram {
//             function_definition: AsmFunctionDefinition::from(c_program.function_definition),
//         }
//     }
// }
//
// impl From<CFunctionDefinition> for AsmFunctionDefinition {
//     fn from(c_function_definition: CFunctionDefinition) -> Self {
//         AsmFunctionDefinition {
//             name: AsmIdetifier::from(c_function_definition.name),
//             instructions: c_function_definition
//                 .body
//                 .iter()
//                 // todo(fede) remove clone
//                 .flat_map(|e| AsmInstruction::from(e.clone()))
//                 .collect::<Vec<AsmInstruction>>(),
//         }
//     }
// }
//
// impl From<CIdentifier> for AsmIdetifier {
//     fn from(c_identifier: CIdentifier) -> Self {
//         AsmIdetifier {
//             value: c_identifier.value,
//         }
//     }
// }
//
// impl From<CExpression> for AsmOperand {
//     fn from(c_expression: CExpression) -> Self {
//         match c_expression {
//             CExpression::Constant(c) => AsmOperand::Imm(c),
//             CExpression::Unary(_u, _e) => todo!(),
//         }
//     }
// }
