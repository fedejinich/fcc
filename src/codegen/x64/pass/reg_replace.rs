use std::collections::HashMap;

use log::{debug, trace};

use crate::{codegen::x64::asm::{AsmFunctionDefinition, AsmInstruction, AsmOperand}, common::folder::FolderAsm};

/// This pass replaces pseudoregisters with stack offsets
pub struct PseudoRegisterReplacer {
    pub offset_map: Option<HashMap<AsmOperand, i32>>,
    pub last_offset: Option<i32>,
}

impl PseudoRegisterReplacer {
    pub fn last_offset(&self) -> i32 {
        let Some(last_offset) = self.last_offset else {
            panic!("couldn't find last_offset");
        };
        last_offset
    }
}

impl FolderAsm for PseudoRegisterReplacer {
    fn create() -> Self {
        Self {
            offset_map: None,
            last_offset: None,
        }
    }

    fn fold_function_definition(
        &mut self,
        function: &AsmFunctionDefinition,
    ) -> AsmFunctionDefinition {
        let (pseudo_reg_map, last_offset) = ids_offset_map(function);

        self.last_offset = Some(last_offset);
        self.offset_map = Some(pseudo_reg_map.clone());

        debug!("Pseudo registers map: {pseudo_reg_map:?}");

        // TODO: this is duplicated code
        AsmFunctionDefinition::new(
            function.name.clone(),
            function
                .instructions
                .iter()
                .flat_map(|i| self.fold_instruction(i))
                .collect(),
        )
    }

    fn fold_instruction(&mut self, instruction: &AsmInstruction) -> Vec<AsmInstruction> {
        use AsmInstruction::*;
        let res = match instruction {
            Mov(src, dst) => {
                trace!("Replace pseudoregisters for Mov({src:?}, {dst:?})");
                Mov(self.fold_operand(src), self.fold_operand(dst))
            }
            Unary(unary_op, op) => {
                trace!("Replace pseudoregisters for Unary({unary_op:?}, {op:?})");
                Unary(unary_op.clone(), self.fold_operand(op))
            }
            Binary(bin_op, src, dst) => {
                trace!("Replace pseudoregisters for Binary({bin_op:?}, {src:?}, {dst:?})");
                Binary(
                    bin_op.clone(),
                    self.fold_operand(src),
                    self.fold_operand(dst),
                )
            }
            Idiv(op) => {
                trace!("Replace pseudoregisters for Idiv({op:?})");
                Idiv(self.fold_operand(op))
            }
            SetCC(cond_code, op) => {
                trace!("Replace pseudoregisters for SetCC({cond_code:?}, {op:?})");
                SetCC(cond_code.clone(), self.fold_operand(op))
            }
            Cmp(op_1, op_2) => {
                trace!("Replace pseudoregisters for Cmp({op_1:?}, {op_2:?})");
                Cmp(self.fold_operand(op_1), self.fold_operand(op_2))
            }
            JmpCC(_, _) | Label(_) | AllocateStack(_) | Comment(_) | Jmp(_) | Ret | Cdq => {
                debug!("Not replacing registers {:?}", &instruction);
                instruction.clone()
            }
        };
        vec![res]
    }

    fn fold_operand(&mut self, operand: &AsmOperand) -> AsmOperand {
        let Some(offset_map) = &self.offset_map else {
            panic!("this should not happen");
        };

        offset_map
            .get(operand)
            .map_or(operand.clone(), |i| AsmOperand::Stack(*i))
    }
}

// returns a map that maps each pseudoregister to its offset on the stack
// and the last offset on the stack
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

// retrieves all operands of an instruction
fn operands(instruction: &AsmInstruction) -> Option<Vec<AsmOperand>> {
    use AsmInstruction::*;
    let ops = match instruction {
        Mov(op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        Unary(_, op) => vec![op.clone()],
        Binary(_, op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        Idiv(op) => vec![op.clone()],
        Cmp(op_1, op_2) => vec![op_1.clone(), op_2.clone()],
        SetCC(_, op) => vec![op.clone()],
        Comment(_) | Cdq | Jmp(_) | JmpCC(_, _) | Label(_) | AllocateStack(_) | Ret => return None,
    };
    Some(ops)
}
