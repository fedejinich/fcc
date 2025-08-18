use std::collections::HashMap;

use log::{debug, trace};

use crate::ast::c::{CExpression, CFunctionDefinition, CProgram, CStatement};
use crate::util::indent;

use super::c::CIdentifier;
use super::tacky::{
    TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram, TackyUnaryOperator,
    TackyValue,
};

#[derive(Clone)]
pub struct AsmProgram {
    function_definition: AsmFunctionDefinition,
}

#[derive(Clone)]
pub struct AsmFunctionDefinition {
    name: AsmIdetifier,
    instructions: Vec<AsmInstruction>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AsmIdetifier {
    value: String,
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

    /// replaces pseudoregisters with stack slots and returns the last stack memory address
    pub fn replace_pseudoregisters(&self) -> (Self, i32) {
        let (new_fd, last_offset) = self.function_definition.with_regs();

        (AsmProgram::new(new_fd), last_offset)
    }

    /// allocates stack and fixes Mov instructions
    pub fn fix_instructions(&self, last_offset: i32) -> Self {
        AsmProgram::new(self.function_definition.fix_instructions(last_offset))
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

    /// replace each pseudoregister with the same address on the stack every time it appears”
    pub fn with_regs(&self) -> (Self, i32) {
        let (pseudo_reg_map, last_offset) = self.ids_offset_map();

        debug!("Pseudo registers map: {pseudo_reg_map:?}");

        (
            AsmFunctionDefinition::new(
                self.name.clone(),
                self.instructions
                    .iter()
                    .map(|i| i.with_reg(&pseudo_reg_map))
                    .collect(),
            ),
            last_offset,
        )
    }

    fn ids_offset_map(&self) -> (HashMap<AsmOperand, i32>, i32) {
        let (map, last_offset) = self
            .instructions
            .iter()
            .flat_map(|i| i.operands())
            .flatten()
            .fold((HashMap::new(), -4i32), |(mut acc, mut next), op| {
                if let AsmOperand::Pseudo(_) = op {
                    if !acc.contains_key(&op) {
                        acc.insert(op.clone(), next);
                        next -= 4; // siguiente slot hacia abajo
                    }
                }
                (acc, next)
            });

        (map, last_offset)
    }

    fn fix_instructions(&self, last_offset: i32) -> AsmFunctionDefinition {
        let mut instructions = vec![AsmInstruction::AllocateStack(last_offset)];
        let mut fixed_instructions = self
            .instructions
            .iter()
            .flat_map(|i| {
                // splits mov instructions that move stack slots to registers
                // into two mov instructions
                trace!("spliting mov instructions");
                match i {
                    AsmInstruction::Mov(AsmOperand::Stack(src), AsmOperand::Stack(dst)) => {
                        vec![
                            AsmInstruction::Mov(
                                AsmOperand::Stack(*src),
                                AsmOperand::Register(Reg::R10),
                            ),
                            AsmInstruction::Mov(
                                AsmOperand::Register(Reg::R10),
                                AsmOperand::Stack(*dst),
                            ),
                        ]
                    }
                    _ => vec![i.clone()],
                }
            })
            .collect();
        instructions.append(&mut fixed_instructions);

        AsmFunctionDefinition::new(self.name.clone(), instructions)
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

    pub fn with_reg(&self, offset_map: &HashMap<AsmOperand, i32>) -> Self {
        match self {
            AsmInstruction::Mov(src, dst) => {
                debug!("Moving {src:?} to {dst:?}");
                AsmInstruction::Mov(src.with_reg(offset_map), dst.with_reg(offset_map))
            }
            AsmInstruction::Unary(unary_op, op) => {
                debug!("Unary {unary_op:?} on {op:?}");
                AsmInstruction::Unary(unary_op.clone(), op.with_reg(offset_map))
            }
            _ => {
                debug!("Not replacing registers");
                self.clone()
            }
        }
    }

    fn operands(&self) -> Option<Vec<AsmOperand>> {
        match self {
            AsmInstruction::Mov(src, dst) => Some(vec![src.clone(), dst.clone()]),
            AsmInstruction::Unary(_, op) => Some(vec![op.clone()]),
            _ => None,
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

    fn with_reg(&self, regs_map: &HashMap<AsmOperand, i32>) -> Self {
        regs_map
            .get(self)
            .map_or(self.clone(), |i| AsmOperand::Stack(*i))
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
