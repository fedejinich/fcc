use std::collections::HashMap;

use log::{debug, trace};

use crate::codegen::x64::asm::AsmInstruction::*;
use crate::codegen::x64::asm::{AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram};

/// replaces pseudoregisters with stack slots and returns the last stack memory address
pub fn replace_pseudoregisters_program(program: &AsmProgram) -> (AsmProgram, i32) {
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
        Binary(bin_op, src, dst) => {
            trace!("Replace pseudoregisters for Binary({bin_op:?}, {src:?}, {dst:?})");
            Binary(
                bin_op.clone(),
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

