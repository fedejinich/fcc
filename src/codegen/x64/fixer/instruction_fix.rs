use crate::{
    codegen::x64::ast::{
        AsmBinaryOperator, AsmFunctionDefinition, AsmInstruction, AsmOperand, Reg,
    },
    common::folder::FolderAsm,
};

/// This pass fixes instructions that violate x86_64 encoding constraints.
///
/// # x86_64 Constraints Handled
///
/// The x86_64 architecture has specific encoding rules that our initial code generation
/// may violate. This pass rewrites those patterns into valid instruction sequences.
///
/// ## Memory-to-memory operations
/// Most x86_64 instructions cannot have both operands in memory. We use R10 as a
/// scratch register to split these into two instructions:
/// - `mov mem, mem` → `mov mem, R10` + `mov R10, mem`
/// - `add/sub mem, mem` → `mov mem, R10` + `add/sub R10, mem`
/// - `and/or/xor mem, mem` → `mov mem, R10` + `and/or/xor R10, mem`
/// - `cmp mem, mem` → `mov mem, R10` + `cmp R10, mem`
///
/// ## Division (`idiv`)
/// The `idiv` instruction cannot take an immediate operand:
/// - `idiv imm` → `mov imm, R10` + `idiv R10`
///
/// ## Multiplication (`imul`)
/// The two-operand `imul` instruction requires the destination to be a register.
/// We use R11 as scratch because the source might already be in R10:
/// - `imul src, mem` → `mov mem, R11` + `imul src, R11` + `mov R11, mem`
///
/// ## Shifts (`shl`/`shr`)
/// Shift instructions require the count to be in the CL register (low byte of CX):
/// - `shl R10, mem` → `mov R10, CX` + `shl CL, mem`
/// - `shl mem, mem` → `mov mem, CX` + `shl CL, mem`
///
/// ## Compare with immediate as second operand
/// The `cmp` instruction cannot have an immediate as the second operand:
/// - `cmp op, imm` → `mov imm, R11` + `cmp op, R11`
///
/// # Scratch Register Policy
/// - **R10**: Primary scratch register for most rewrites
/// - **R11**: Used when R10 might conflict (e.g., `imul`, `cmp` with immediate)
/// - **CX/CL**: Used exclusively for shift counts
#[derive(Default)]
pub struct InstructionFixer {
    last_offset: Option<i32>, // space reserved for stack
}

impl InstructionFixer {
    pub fn with(&self, last_offset: i32) -> Self {
        Self {
            last_offset: Some(last_offset),
        }
    }
}

impl FolderAsm for InstructionFixer {
    fn name(&self) -> &'static str {
        "ins_fix"
    }

    fn create() -> Self {
        Self::default()
    }

    fn fold_fun_def(
        &mut self,
        function_definition: AsmFunctionDefinition,
    ) -> Result<AsmFunctionDefinition, String> {
        let Some(last_offset) = self.last_offset else {
            return Err("last_offset should be set".to_string());
        };

        let mut instructions = vec![AsmInstruction::AllocateStack(last_offset.abs())];

        let fixed_instructions: Result<Vec<_>, String> = function_definition
            .instructions
            .into_iter()
            .map(|i| self.fold_ins(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());

        let mut fixed_instructions = fixed_instructions?;
        instructions.append(&mut fixed_instructions);

        Ok(AsmFunctionDefinition::new(
            function_definition.name,
            instructions,
        ))
    }

    fn fold_ins(&mut self, instruction: AsmInstruction) -> Result<Vec<AsmInstruction>, String> {
        use AsmBinaryOperator::*;
        use AsmInstruction::*;
        use AsmOperand::*;

        let result = match instruction {
            // Generic mem-mem patterns (use helpers)
            Mov(Stack(src), Stack(dst)) => fix_mov_mem_mem(src, dst),
            Binary(bin_op @ (Add | Sub), Stack(src), Stack(dst)) => {
                fix_binary_mem_mem(bin_op, src, dst)
            }
            Binary(bin_op @ (BitwiseAnd | BitwiseOr | BitwiseXor), Stack(src), Stack(dst)) => {
                fix_binary_mem_mem(bin_op, src, dst)
            }
            Cmp(Stack(op_1), Stack(op_2)) => fix_cmp_mem_mem(op_1, op_2),

            // Special case: idiv cannot take an immediate operand
            Idiv(Imm(num)) => vec![
                Comment("fix: idiv imm -> mov imm,R10 + idiv R10".to_string()),
                Mov(Imm(num), Register(Reg::R10)),
                Idiv(Register(Reg::R10)),
            ],

            // Special case: imul destination must be a register (uses R11)
            Binary(Mult, src, Stack(dst)) => {
                vec![
                    Comment(
                        "fix: imul src,mem -> mov mem,R11 + imul src,R11 + mov R11,mem".to_string(),
                    ),
                    Mov(Stack(dst), Register(Reg::R11)),
                    Binary(Mult, src, Register(Reg::R11)),
                    Mov(Register(Reg::R11), Stack(dst)),
                ]
            }

            // Special case: shift count must be in CL register
            Binary(bin_op @ (LeftShift | RightShift), Register(Reg::R10), Stack(dst)) => vec![
                Comment("fix: shl/shr R10,mem -> mov R10,CX + op CL,mem".to_string()),
                Mov(Register(Reg::R10), Register(Reg::CX)),
                Binary(bin_op, Register(Reg::CL), Stack(dst)),
            ],
            Binary(bin_op @ (LeftShift | RightShift), Stack(src), Stack(dst)) => {
                vec![
                    Comment("fix: shl/shr mem,mem -> mov mem,CX + op CL,mem".to_string()),
                    Mov(Stack(src), Register(Reg::CX)),
                    Binary(bin_op, Register(Reg::CL), Stack(dst)),
                ]
            }

            // Special case: cmp second operand cannot be an immediate (uses R11)
            Cmp(op_1, Imm(constant)) => {
                vec![
                    Comment("fix: cmp op,imm -> mov imm,R11 + cmp op,R11".to_string()),
                    Mov(Imm(constant), Register(Reg::R11)),
                    Cmp(op_1, Register(Reg::R11)),
                ]
            }

            other => vec![other],
        };

        Ok(result)
    }
}

/// Splits a memory-to-memory mov into two instructions using R10 as scratch.
/// Pattern: `mov mem, mem` → `mov mem, R10` + `mov R10, mem`
fn fix_mov_mem_mem(src: i32, dst: i32) -> Vec<AsmInstruction> {
    vec![
        AsmInstruction::Comment("fix: mov mem,mem -> mov mem,R10 + mov R10,mem".to_string()),
        AsmInstruction::Mov(AsmOperand::Stack(src), AsmOperand::Register(Reg::R10)),
        AsmInstruction::Mov(AsmOperand::Register(Reg::R10), AsmOperand::Stack(dst)),
    ]
}

/// Splits a memory-to-memory binary op into two instructions using R10 as scratch.
/// Pattern: `op mem, mem` → `mov mem, R10` + `op R10, mem`
/// Used for: add, sub, and, or, xor
fn fix_binary_mem_mem(op: AsmBinaryOperator, src: i32, dst: i32) -> Vec<AsmInstruction> {
    vec![
        AsmInstruction::Comment("fix: op mem,mem -> mov mem,R10 + op R10,mem".to_string()),
        AsmInstruction::Mov(AsmOperand::Stack(src), AsmOperand::Register(Reg::R10)),
        AsmInstruction::Binary(op, AsmOperand::Register(Reg::R10), AsmOperand::Stack(dst)),
    ]
}

/// Splits a memory-to-memory cmp into two instructions using R10 as scratch.
/// Pattern: `cmp mem, mem` → `mov mem, R10` + `cmp R10, mem`
fn fix_cmp_mem_mem(op_1: i32, op_2: i32) -> Vec<AsmInstruction> {
    vec![
        AsmInstruction::Comment("fix: cmp mem,mem -> mov mem,R10 + cmp R10,mem".to_string()),
        AsmInstruction::Mov(AsmOperand::Stack(op_1), AsmOperand::Register(Reg::R10)),
        AsmInstruction::Cmp(AsmOperand::Register(Reg::R10), AsmOperand::Stack(op_2)),
    ]
}


