use crate::{codegen::x64::ast::{AsmBinaryOperator, AsmFunctionDefinition, AsmInstruction, AsmOperand, Reg}, common::folder::FolderAsm};

/// This pass fixes some instructions that are not supported by the x64 architecture.
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
    fn create() -> Self {
        Self::default()
    }

    fn fold_function_definition(
        &mut self,
        function_definition: AsmFunctionDefinition,
    ) -> Result<AsmFunctionDefinition, String> {
        let Some(last_offset) = self.last_offset else {
            return Err("last_offset should be set".to_string());
        };

        let mut instructions = vec![AsmInstruction::AllocateStack(last_offset)];

        let fixed_instructions: Result<Vec<_>, String> = function_definition
            .instructions
            .into_iter()
            .map(|i| self.fold_instruction(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());

        let mut fixed_instructions = fixed_instructions?;
        instructions.append(&mut fixed_instructions);

        Ok(AsmFunctionDefinition::new(function_definition.name, instructions))
    }

    fn fold_instruction(&mut self, instruction: AsmInstruction) -> Result<Vec<AsmInstruction>, String> {
        use AsmBinaryOperator::*;
        use AsmInstruction::*;
        use AsmOperand::*;

        let result = match instruction {
            Mov(Stack(src), Stack(dst)) => {
                vec![
                    Comment("splited mov into two mov instructions".to_string()),
                    Mov(Stack(src), Register(Reg::R10)),
                    Mov(Register(Reg::R10), Stack(dst)),
                ]
            }
            Idiv(Imm(num)) => vec![
                Comment("splited idiv into mov idiv".to_string()),
                Mov(Imm(num), Register(Reg::R10)),
                Idiv(Register(Reg::R10)),
            ],
            Binary(bin_op @ (Add | Sub), Stack(src), Stack(dst)) => vec![
                Comment("splitted add/sub into mov add/sub instructions".to_string()),
                Mov(Stack(src), Register(Reg::R10)),
                Binary(bin_op, Register(Reg::R10), Stack(dst)),
            ],
            Binary(Mult, src, Stack(dst)) => {
                vec![
                    Comment("splitted mul into mov mul mov instructions".to_string()),
                    Mov(Stack(dst), Register(Reg::R11)),
                    Binary(Mult, src, Register(Reg::R11)),
                    Mov(Register(Reg::R11), Stack(dst)),
                ]
            }
            Binary(bin_op @ (BitwiseAnd | BitwiseOr | BitwiseXor), Stack(src), Stack(dst)) => {
                vec![
                    Comment(
                        "splitted bitwise and/or/xor into mov and/or/xor instructions".to_string(),
                    ),
                    Mov(Stack(src), Register(Reg::R10)),
                    Binary(bin_op, Register(Reg::R10), Stack(dst)),
                ]
            }
            Binary(bin_op @ (LeftShift | RightShift), Register(Reg::R10), Stack(dst)) => vec![
                Comment("splitted shl/shr into mov and instructions".to_string()),
                Mov(Register(Reg::R10), Register(Reg::CX)),
                Binary(bin_op, Register(Reg::CL), Stack(dst)),
            ],
            Binary(bin_op @ (LeftShift | RightShift), Stack(src), Stack(dst)) => {
                vec![
                    Comment("splitted shl/shr into mov and instructions".to_string()),
                    Mov(Stack(src), Register(Reg::CX)),
                    Binary(bin_op, Register(Reg::CL), Stack(dst)),
                ]
            }
            Cmp(Stack(op_1), Stack(op_2)) => vec![
                Comment("splitted cmp into mov and cmpl instructions".to_string()),
                Mov(Stack(op_1), Register(Reg::R10)),
                Cmp(Register(Reg::R10), Stack(op_2)),
            ],
            Cmp(op_1, Imm(constant)) => {
                vec![
                    Comment("splitted cmp into mov and cmpl instructions".to_string()),
                    Mov(Imm(constant), Register(Reg::R11)),
                    Cmp(op_1, Register(Reg::R11)),
                ]
            }
            other => vec![other],
        };
        Ok(result)
    }
}
