//! This module contains the logic to apply actions on the tacky AST and pipe them into a new AST

use std::collections::HashMap;

use log::{debug, trace};

use AsmBinaryOperator::*;
use AsmInstruction::*;
use AsmOperand::*;

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
    /// replaces pseudoregisters with stack slots and returns the last stack memory address
    pub fn replace_pseudoregisters(mut self) -> Self {
        let (program, last_offset) = replace_pseudoregisters_program(&self.program);
        self.program = program;
        self.last_offset = Some(last_offset);

        self
    }

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

    pub fn out(self) -> AsmProgram {
        self.program
    }
}

fn fix_function_definition(
    function_definition: &AsmFunctionDefinition,
    last_offset: i32,
) -> AsmFunctionDefinition {
    let mut instructions = vec![AsmInstruction::AllocateStack(last_offset)];
    let mut fixed_instructions = function_definition
        .instructions
        .iter()
        .flat_map(fix_instruction)
        .collect();
    instructions.append(&mut fixed_instructions);

    AsmFunctionDefinition::new(function_definition.name.clone(), instructions)
}

fn fix_instruction(instruction: &AsmInstruction) -> Vec<AsmInstruction> {
    match instruction {
        Mov(Stack(src), Stack(dst)) => {
            vec![
                Comment("splited mov into two mov instructions".to_string()),
                Mov(Stack(*src), Register(Reg::R10)),
                Mov(Register(Reg::R10), Stack(*dst)),
            ]
        }
        Idiv(Imm(num)) => vec![
            Comment("splited idiv into mov idiv".to_string()),
            Mov(Imm(*num), Register(Reg::R10)),
            Idiv(Register(Reg::R10)),
        ],
        Binary(Add, Stack(src), Stack(dst)) => vec![
            Comment("splitted add into mov add instructions".to_string()),
            Mov(Stack(*src), Register(Reg::R10)),
            Binary(Add, Register(Reg::R10), Stack(*dst)),
        ],
        Binary(Sub, Stack(src), Stack(dst)) => vec![
            Comment("splitted sub into mov sub instructions".to_string()),
            Mov(Stack(*src), Register(Reg::R10)),
            Binary(Sub, Register(Reg::R10), Stack(*dst)),
        ],
        Binary(Mult, src, Stack(dst)) => {
            vec![
                Comment("splitted mul into mov mul mov instructions".to_string()),
                Mov(Stack(*dst), Register(Reg::R11)),
                Binary(Mult, src.clone(), Register(Reg::R11)),
                Mov(Register(Reg::R11), Stack(*dst)),
            ]
        }
        Binary(BitwiseAnd, Stack(src), Stack(dst)) => {
            vec![
                Comment("splitted and into mov and instructions".to_string()),
                Mov(Stack(*src), Register(Reg::R10)),
                Binary(BitwiseAnd, Register(Reg::R10), Stack(*dst)),
            ]
        }
        Binary(BitwiseOr, Stack(src), Stack(dst)) => {
            vec![
                Comment("splitted or into mov and instructions".to_string()),
                Mov(Stack(*src), Register(Reg::R10)),
                Binary(BitwiseOr, Register(Reg::R10), Stack(*dst)),
            ]
        }
        Binary(BitwiseXor, Stack(src), Stack(dst)) => {
            vec![
                Comment("splitted xor into mov and instructions".to_string()),
                Mov(Stack(*src), Register(Reg::R10)),
                Binary(BitwiseXor, Register(Reg::R10), Stack(*dst)),
            ]
        }
        Binary(LeftShift, Register(Reg::R10), Stack(dst)) => vec![
            Comment("splitted shl into mov and instructions".to_string()),
            Mov(Register(Reg::R10), Register(Reg::CX)),
            Binary(LeftShift, Register(Reg::CL), Stack(*dst)),
        ],
        Binary(RightShift, Register(Reg::R10), Stack(dst)) => vec![
            Comment("splitted shr into mov and instructions".to_string()),
            Mov(Register(Reg::R10), Register(Reg::CX)),
            Binary(RightShift, Register(Reg::CL), Stack(*dst)),
        ],
        Binary(LeftShift, Stack(src), Stack(dst)) => {
            vec![
                Comment("splitted shl into mov and instructions".to_string()),
                Mov(Stack(*src), Register(Reg::CX)),
                Binary(LeftShift, Register(Reg::CL), Stack(*dst)),
            ]
        }
        Binary(RightShift, Stack(src), Stack(dst)) => {
            vec![
                Comment("splitted shr into mov and instructions".to_string()),
                Mov(Stack(*src), Register(Reg::CX)),
                Binary(RightShift, Register(Reg::CL), Stack(*dst)),
            ]
        }
        Cmp(Stack(op_1), Stack(op_2)) => vec![
            Comment("splitted cmp into mov and cmpl instructions".to_string()),
            Mov(Stack(*op_1), Register(Reg::R10)),
            Cmp(Register(Reg::R10), Stack(*op_2)),
        ],
        Cmp(op_1, Imm(constant)) => {
            vec![
                Comment("splitted cmp into mov and cmpl instructions".to_string()),
                Mov(Imm(*constant), Register(Reg::R11)),
                Cmp(op_1.clone(), Register(Reg::R11)),
            ]
        }
        _ => vec![instruction.clone()], // TODO: this clone is weird
    }
}

/// replaces pseudoregisters with stack slots and returns the last stack memory address
fn replace_pseudoregisters_program(program: &AsmProgram) -> (AsmProgram, i32) {
    let (new_fd, last_offset) = replace_pseudoregisters_fd(&program.function_definition);

    (AsmProgram::new(new_fd), last_offset)
}

/// "replace each pseudoregister with the same address on the stack every time it appears"
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
        Mov(src, dst) => {
            trace!("Replace pseudoregisters for Mov({src:?}, {dst:?})");
            Mov(
                replace_pseudoregisters_op(src, offset_map),
                replace_pseudoregisters_op(dst, offset_map),
            )
        }
        Unary(unary_op, op) => {
            trace!("Replace pseudoregisters for Unary({unary_op:?}, {op:?})");
            Unary(unary_op.clone(), replace_pseudoregisters_op(op, offset_map))
        }
        Binary(op, src, dst) => {
            trace!("Replace pseudoregisters for Binary({op:?}, {src:?}, {dst:?})");
            Binary(
                op.clone(),
                replace_pseudoregisters_op(src, offset_map),
                replace_pseudoregisters_op(dst, offset_map),
            )
        }
        Idiv(op) => {
            trace!("Replace pseudoregisters for Idiv({op:?})");
            Idiv(replace_pseudoregisters_op(op, offset_map))
        }
        SetCC(cond_code, op) => {
            trace!("Replace pseudoregisters for SetCC({cond_code:?}, {op:?})");
            SetCC(
                cond_code.clone(),
                replace_pseudoregisters_op(op, offset_map),
            )
        }
        Cmp(op_1, op_2) => {
            trace!("Replace pseudoregisters for Cmp({op_1:?}, {op_2:?})");
            Cmp(
                replace_pseudoregisters_op(op_1, offset_map),
                replace_pseudoregisters_op(op_2, offset_map),
            )
        }
        JmpCC(_, _) | Label(_) | AllocateStack(_) | Comment(_) | Jmp(_) | Ret | Cdq => {
            debug!("Not replacing registers {:?}", &instruction);
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
            // tag pseudo register
            if let AsmOperand::Pseudo(_) = op {
                if !acc.contains_key(&op) {
                    acc.insert(op.clone(), next);
                    next -= 4;
                }
            }
            (acc, next)
        });

    (map, last_offset)
}

fn operands(instruction: &AsmInstruction) -> Option<Vec<AsmOperand>> {
    let ops = match instruction {
        Mov(op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        Unary(_, op) => vec![op.clone()],
        Binary(_, op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        Idiv(op) => vec![op.clone()],
        Cmp(op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        SetCC(_, op) => vec![op.clone()],
        _ => return None,
    };
    Some(ops)
}
