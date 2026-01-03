use std::collections::HashMap;

use log::{debug, error, info};

use crate::{
    codegen::x64::ast::{AsmFunctionDefinition, AsmInstruction, AsmOperand},
    common::folder::FolderAsm,
};

/// This pass replaces pseudo-registers with stack offsets.
///
/// # Overview
///
/// After lowering TACKY to ASM, the code uses "pseudo-registers" (`AsmOperand::Pseudo`)
/// to represent variables and temporaries. x86_64 has a limited number of registers,
/// so this pass allocates stack slots for each pseudo-register.
///
/// # Stack Layout
///
/// Each pseudo-register gets a unique 4-byte slot on the stack. The stack grows
/// downward (toward lower addresses), so offsets are negative relative to RBP:
///
/// ```text
///     ┌─────────────────┐  Higher addresses
///     │  Return addr    │
///     ├─────────────────┤  ← RBP (frame pointer)
///     │  var_1          │  -4(%rbp)
///     ├─────────────────┤
///     │  var_2          │  -8(%rbp)
///     ├─────────────────┤
///     │  tmp.0          │  -12(%rbp)
///     ├─────────────────┤
///     │  ...            │
///     └─────────────────┘  ← RSP (stack pointer)
/// ```
///
/// # Pipeline Contract
///
/// This pass produces two outputs:
/// 1. **Transformed instructions**: All `Pseudo(id)` operands replaced with `Stack(offset)`
/// 2. **`last_offset`**: The total stack space needed (used by `InstructionFixer` to emit `AllocateStack`)
///
/// The `InstructionFixer` pass **must** run after this pass and use `last_offset`
/// to allocate the correct amount of stack space.
///
/// # Allocation Strategy
///
/// - First pseudo-register seen → offset -4
/// - Second pseudo-register → offset -8
/// - And so on...
/// - `last_offset` is the next available offset (e.g., -16 if 3 variables allocated)
///
/// Note: Currently allocates 4 bytes per variable regardless of type (int = 4 bytes)
#[derive(Default)]
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
    fn name(&self) -> &'static str {
        "reg_replace"
    }

    fn create() -> Self {
        Self {
            offset_map: None,
            last_offset: None,
        }
    }

    fn fold_fun_def(
        &mut self,
        function: AsmFunctionDefinition,
    ) -> Result<AsmFunctionDefinition, String> {
        let (pseudo_reg_map, last_offset) = ids_offset_map(&function);
        self.last_offset = Some(last_offset);
        self.offset_map = Some(pseudo_reg_map.clone());

        info!(
            "[codegen] {} pseudo registers, stack size {}",
            pseudo_reg_map.len(),
            -last_offset
        );

        debug!("[codegen] pseudo register map: {pseudo_reg_map:?}");

        let instructions: Result<Vec<_>, String> = function
            .instructions
            .into_iter()
            .map(|i| self.fold_ins(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());

        Ok(AsmFunctionDefinition::new(function.name, instructions?))
    }

    fn fold_ins(&mut self, instruction: AsmInstruction) -> Result<Vec<AsmInstruction>, String> {
        use AsmInstruction::*;

        let res = match instruction {
            Mov(src, dst) => Mov(self.fold_op(src)?, self.fold_op(dst)?),
            Unary(op, operand) => Unary(op, self.fold_op(operand)?),
            Binary(op, src, dst) => Binary(op, self.fold_op(src)?, self.fold_op(dst)?),
            Idiv(op) => Idiv(self.fold_op(op)?),
            SetCC(cc, op) => SetCC(cc, self.fold_op(op)?),
            Cmp(op1, op2) => Cmp(self.fold_op(op1)?, self.fold_op(op2)?),
            JmpCC(_, _) | Label(_) | AllocateStack(_) | Comment(_) | Jmp(_) | Ret | Cdq => {
                instruction
            }
        };

        Ok(vec![res])
    }

    fn fold_op(&mut self, operand: AsmOperand) -> Result<AsmOperand, String> {
        let Some(offset_map) = &self.offset_map else {
            error!("[codegen] offset_map not initialized");

            return Err("offset_map not initialized".to_string());
        };

        Ok(offset_map
            .get(&operand)
            .map_or(operand, |i| AsmOperand::Stack(*i)))
    }
}

/// Builds a map from pseudo-registers to stack offsets.
///
/// Returns:
/// - `HashMap<AsmOperand, i32>`: Maps each `Pseudo(id)` to its stack offset
/// - `i32`: The next available offset (used to calculate total stack size)
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

/// Extracts all operands from an instruction for pseudo-register discovery.
/// Returns `None` for instructions that don't have operands (jumps, labels, etc.)
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
