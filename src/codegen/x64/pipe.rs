//! This module contains the logic to apply actions on the tacky AST and pipe them into a new AST

use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    codegen::x64::asm::{
        AsmBinaryOperator, AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram, Reg,
    },
    tacky::program::TackyProgram,
};

pub struct AsmPipe {
    program: AsmProgram,
    last_offset: Option<i32>,
}

impl From<TackyProgram> for AsmPipe {
    fn from(tp: TackyProgram) -> Self {
        Self {
            program: AsmProgram::from(tp),
            last_offset: None,
        }
    }
}

impl AsmPipe {
    /// fixes Mov instructions
    pub fn fix_instructions(mut self) -> Self {
        let last_offset = self
            .last_offset
            .expect("should call replace_pseudoregisters first");
        self.program = AsmProgram::new(fix_function_definition(
            &self.program.function_definition,
            last_offset,
        ));

        self
    }

    /// replaces pseudoregisters with stack slots and returns the last stack memory address
    pub fn replace_pseudoregisters(mut self) -> Self {
        let (program, last_offset) = replace_pseudoregisters_program(&self.program);
        self.program = program;
        self.last_offset = Some(last_offset);

        self
    }

    pub fn out(self) -> AsmProgram {
        self.program
    }
}

// TODO: this function has too many code
fn fix_function_definition(
    function_definition: &AsmFunctionDefinition,
    last_offset: i32,
) -> AsmFunctionDefinition {
    let mut instructions = vec![AsmInstruction::AllocateStack(last_offset)];
    let mut fixed_instructions = function_definition
        .instructions
        .iter()
        .flat_map(|i| {
            trace!("spliting mov instructions");
            match i {
                AsmInstruction::Mov(AsmOperand::Stack(src), AsmOperand::Stack(dst)) => {
                    vec![
                        AsmInstruction::Comment(
                            "splited mov into two mov instructions".to_string(),
                        ),
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
                AsmInstruction::Idiv(AsmOperand::Imm(num)) => vec![
                    AsmInstruction::Comment("splited idiv into mov idiv".to_string()),
                    AsmInstruction::Mov(AsmOperand::Imm(*num), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Idiv(AsmOperand::Register(Reg::R10)),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::Add,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted add into mov add instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::Add,
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::Sub,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted sub into mov sub instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::Sub,
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(AsmBinaryOperator::Mult, src, AsmOperand::Stack(dst)) => {
                    vec![
                        AsmInstruction::Comment(
                            "splitted mul into mov mul mov instructions".to_string(),
                        ),
                        AsmInstruction::Mov(
                            AsmOperand::Stack(*dst),
                            AsmOperand::Register(Reg::R11),
                        ),
                        AsmInstruction::Binary(
                            AsmBinaryOperator::Mult,
                            src.clone(),
                            AsmOperand::Register(Reg::R11),
                        ),
                        AsmInstruction::Mov(
                            AsmOperand::Register(Reg::R11),
                            AsmOperand::Stack(*dst),
                        ),
                    ]
                }
                AsmInstruction::Binary(
                    AsmBinaryOperator::BitwiseAnd,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted and into mov and instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::BitwiseAnd,
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::BitwiseOr,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted or into mov and instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::BitwiseOr,
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::BitwiseXor,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted xor into mov and instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::R10)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::BitwiseXor,
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::LeftShift,
                    AsmOperand::Register(Reg::R10),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted shl into mov and instructions".to_string()),
                    AsmInstruction::Mov(
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Register(Reg::CX),
                    ),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::LeftShift,
                        AsmOperand::Register(Reg::CL),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::RightShift,
                    AsmOperand::Register(Reg::R10),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted shr into mov and instructions".to_string()),
                    AsmInstruction::Mov(
                        AsmOperand::Register(Reg::R10),
                        AsmOperand::Register(Reg::CX),
                    ),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::RightShift,
                        AsmOperand::Register(Reg::CL),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::LeftShift,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted shl into mov and instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::CX)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::LeftShift,
                        AsmOperand::Register(Reg::CL),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                AsmInstruction::Binary(
                    AsmBinaryOperator::RightShift,
                    AsmOperand::Stack(src),
                    AsmOperand::Stack(dst),
                ) => vec![
                    AsmInstruction::Comment("splitted shr into mov and instructions".to_string()),
                    AsmInstruction::Mov(AsmOperand::Stack(*src), AsmOperand::Register(Reg::CX)),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::RightShift,
                        AsmOperand::Register(Reg::CL),
                        AsmOperand::Stack(*dst),
                    ),
                ],
                _ => vec![i.clone()],
            }
        })
        .collect();
    instructions.append(&mut fixed_instructions);

    AsmFunctionDefinition::new(function_definition.name.clone(), instructions)
}

/// replaces pseudoregisters with stack slots and returns the last stack memory address
fn replace_pseudoregisters_program(program: &AsmProgram) -> (AsmProgram, i32) {
    let (new_fd, last_offset) = replace_pseudoregisters_fd(&program.function_definition);

    (AsmProgram::new(new_fd), last_offset)
}

/// replace each pseudoregister with the same address on the stack every time it appears”
fn replace_pseudoregisters_fd(
    function_definition: &AsmFunctionDefinition,
) -> (AsmFunctionDefinition, i32) {
    let (pseudo_reg_map, last_offset) = ids_offset_map(function_definition);

    debug!("Pseudo registers map: {pseudo_reg_map:?}");

    (
        AsmFunctionDefinition::new(
            function_definition.name.clone(),
            function_definition
                .instructions
                .iter()
                .map(|i| replace_pseudoregisters_i(i, &pseudo_reg_map))
                .collect(),
        ),
        last_offset,
    )
}

fn replace_pseudoregisters_i(
    instruction: &AsmInstruction,
    offset_map: &HashMap<AsmOperand, i32>,
) -> AsmInstruction {
    match instruction {
        AsmInstruction::Mov(src, dst) => {
            debug!("Replace pseudoregisters for Mov({src:?}, {dst:?})");
            AsmInstruction::Mov(
                replace_pseudoregisters_op(src, offset_map),
                replace_pseudoregisters_op(dst, offset_map),
            )
        }
        AsmInstruction::Unary(unary_op, op) => {
            debug!("Replace pseudoregisters for Unary({unary_op:?}, {op:?})");
            AsmInstruction::Unary(unary_op.clone(), replace_pseudoregisters_op(op, offset_map))
        }
        AsmInstruction::Binary(op, src, dst) => {
            debug!("Replace pseudoregisters for Binary({op:?}, {src:?}, {dst:?})");
            AsmInstruction::Binary(
                op.clone(),
                replace_pseudoregisters_op(src, offset_map),
                replace_pseudoregisters_op(dst, offset_map),
            )
        }
        AsmInstruction::Idiv(op) => {
            AsmInstruction::Idiv(replace_pseudoregisters_op(op, offset_map))
        }
        // TODO: same problem here '_' makes errors (maybe this is solved with unit testing)
        _ => {
            debug!("Not replacing registers");
            instruction.clone()
        }
    }
}

fn replace_pseudoregisters_op(
    operand: &AsmOperand,
    offset_map: &HashMap<AsmOperand, i32>,
) -> AsmOperand {
    offset_map
        .get(operand)
        .map_or(operand.clone(), |i| AsmOperand::Stack(*i))
}

fn ids_offset_map(function_definition: &AsmFunctionDefinition) -> (HashMap<AsmOperand, i32>, i32) {
    let (map, last_offset) = function_definition
        .instructions
        .iter()
        .flat_map(operands)
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

// TODO: this should return an empty list instead of None
// TODO: find a way to fail if new instruction is not handled
fn operands(instruction: &AsmInstruction) -> Option<Vec<AsmOperand>> {
    match instruction {
        AsmInstruction::Mov(op_1, op_2) => Some(vec![op_1.clone(), op_2.clone()]),
        AsmInstruction::Unary(_, op) => Some(vec![op.clone()]),
        AsmInstruction::Binary(_, op_1, op_2) => Some(vec![op_1.clone(), op_2.clone()]),
        AsmInstruction::Idiv(op) => Some(vec![op.clone()]),
        AsmInstruction::Cmp(op_1, op_2) => Some(vec![op_1.clone(), op_2.clone()]),
        AsmInstruction::SetCC(_, op) => Some(vec![op.clone()]),
        _ => None,
    }
}
