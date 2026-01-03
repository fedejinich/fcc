use crate::{
    codegen::x64::ast::{
        AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmIdentifier, AsmInstruction,
        AsmOperand, AsmProgram, AsmUnaryOperator, Reg,
    },
    tacky::ast::{
        TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction,
        TackyProgram, TackyUnaryOperator, TackyValue,
    },
};

// helpers for common asm emission patterns

fn emit_conditional_jump(
    condition: TackyValue,
    target: TackyIdentifier,
    jump_when_zero: bool,
) -> Vec<AsmInstruction> {
    let cond_code = if jump_when_zero {
        AsmCondCode::E
    } else {
        AsmCondCode::NE
    };
    vec![
        AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(condition)),
        AsmInstruction::JmpCC(cond_code, AsmIdentifier::from(target)),
    ]
}

fn emit_relational(
    op: TackyBinaryOperator,
    src_1: TackyValue,
    src_2: TackyValue,
    dst: TackyValue,
) -> Vec<AsmInstruction> {
    vec![
        AsmInstruction::Cmp(AsmOperand::from(src_2), AsmOperand::from(src_1)),
        AsmInstruction::Mov(AsmOperand::Imm(0), AsmOperand::from(dst.clone())),
        AsmInstruction::SetCC(AsmCondCode::from(op), AsmOperand::from(dst)),
    ]
}

fn emit_div_rem(
    is_div: bool,
    src_1: TackyValue,
    src_2: TackyValue,
    dst: TackyValue,
) -> Vec<AsmInstruction> {
    let result_reg = if is_div { Reg::AX } else { Reg::DX };
    vec![
        AsmInstruction::Mov(AsmOperand::from(src_1), AsmOperand::Register(Reg::AX)),
        AsmInstruction::Cdq,
        AsmInstruction::Idiv(AsmOperand::from(src_2)),
        AsmInstruction::Mov(AsmOperand::Register(result_reg), AsmOperand::from(dst)),
    ]
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
            name: AsmIdentifier::from(tacky_function_definition.name),
            instructions: tacky_function_definition
                .instructions
                .into_iter()
                .flat_map(AsmInstruction::from)
                .collect::<Vec<AsmInstruction>>(),
        }
    }
}

impl AsmInstruction {
    fn from(tacky_instruction: TackyInstruction) -> Vec<AsmInstruction> {
        match tacky_instruction {
            TackyInstruction::Comment(_) => vec![],
            TackyInstruction::Return(val) => vec![
                AsmInstruction::Mov(AsmOperand::from(val), AsmOperand::Register(Reg::AX)),
                AsmInstruction::Ret,
            ],
            TackyInstruction::Unary(TackyUnaryOperator::Not, src, dst) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(src)),
                AsmInstruction::Mov(AsmOperand::Imm(0), AsmOperand::from(dst.clone())),
                AsmInstruction::SetCC(AsmCondCode::E, AsmOperand::from(dst)),
            ],
            TackyInstruction::Unary(unary_op, src, dst) => vec![
                AsmInstruction::Mov(AsmOperand::from(src), AsmOperand::from(dst.clone())),
                AsmInstruction::Unary(AsmUnaryOperator::from(unary_op), AsmOperand::from(dst)),
            ],
            TackyInstruction::Binary(op, src_1, src_2, dst) => match op {
                // arithmetic and bitwise
                TackyBinaryOperator::Add
                | TackyBinaryOperator::Subtract
                | TackyBinaryOperator::Multiply
                | TackyBinaryOperator::BitwiseAnd
                | TackyBinaryOperator::BitwiseOr
                | TackyBinaryOperator::BitwiseXor
                | TackyBinaryOperator::LeftShift
                | TackyBinaryOperator::RightShift => vec![
                    AsmInstruction::Mov(AsmOperand::from(src_1), AsmOperand::from(dst.clone())),
                    AsmInstruction::Binary(
                        AsmBinaryOperator::from(op),
                        AsmOperand::from(src_2),
                        AsmOperand::from(dst),
                    ),
                ],
                // relational
                TackyBinaryOperator::Equal
                | TackyBinaryOperator::NotEqual
                | TackyBinaryOperator::GreaterThan
                | TackyBinaryOperator::LessThan
                | TackyBinaryOperator::LessThanOrEqual
                | TackyBinaryOperator::GreaterThanOrEqual => emit_relational(op, src_1, src_2, dst),
                // division and remainder
                TackyBinaryOperator::Divide => emit_div_rem(true, src_1, src_2, dst),
                TackyBinaryOperator::Remainder => emit_div_rem(false, src_1, src_2, dst),
            },
            TackyInstruction::Jump(id) => vec![AsmInstruction::Jmp(AsmIdentifier::from(id))],
            TackyInstruction::JumpIfZero(cond, target) => emit_conditional_jump(cond, target, true),
            TackyInstruction::JumpIfNotZero(cond, target) => {
                emit_conditional_jump(cond, target, false)
            }
            TackyInstruction::Copy(src, dst) => vec![AsmInstruction::Mov(
                AsmOperand::from(src),
                AsmOperand::from(dst),
            )],
            TackyInstruction::Label(id) => vec![AsmInstruction::Label(AsmIdentifier::from(id))],
        }
    }
}

impl From<TackyValue> for AsmOperand {
    fn from(tacky_value: TackyValue) -> Self {
        match tacky_value {
            TackyValue::Constant(c) => AsmOperand::Imm(c),
            TackyValue::Var(id) => AsmOperand::Pseudo(AsmIdentifier::from(id)),
        }
    }
}

impl From<TackyIdentifier> for AsmIdentifier {
    fn from(tacky_identifier: TackyIdentifier) -> Self {
        AsmIdentifier {
            value: tacky_identifier.value,
        }
    }
}

impl From<TackyUnaryOperator> for AsmUnaryOperator {
    fn from(tacky_unary_operator: TackyUnaryOperator) -> Self {
        match tacky_unary_operator {
            TackyUnaryOperator::Negate => AsmUnaryOperator::Neg,
            TackyUnaryOperator::Complement => AsmUnaryOperator::Not,
            TackyUnaryOperator::Not => AsmUnaryOperator::Not,
        }
    }
}

impl From<TackyBinaryOperator> for AsmBinaryOperator {
    fn from(op: TackyBinaryOperator) -> Self {
        match op {
            TackyBinaryOperator::Add => AsmBinaryOperator::Add,
            TackyBinaryOperator::Subtract => AsmBinaryOperator::Sub,
            TackyBinaryOperator::Multiply => AsmBinaryOperator::Mult,
            TackyBinaryOperator::BitwiseAnd => AsmBinaryOperator::BitwiseAnd,
            TackyBinaryOperator::BitwiseOr => AsmBinaryOperator::BitwiseOr,
            TackyBinaryOperator::BitwiseXor => AsmBinaryOperator::BitwiseXor,
            TackyBinaryOperator::LeftShift => AsmBinaryOperator::LeftShift,
            TackyBinaryOperator::RightShift => AsmBinaryOperator::RightShift,
            _ => panic!("invalid binary operator for asm: {op:?}"),
        }
    }
}

impl From<TackyBinaryOperator> for AsmCondCode {
    fn from(op: TackyBinaryOperator) -> Self {
        match op {
            TackyBinaryOperator::Equal => AsmCondCode::E,
            TackyBinaryOperator::NotEqual => AsmCondCode::NE,
            TackyBinaryOperator::GreaterThan => AsmCondCode::G,
            TackyBinaryOperator::LessThan => AsmCondCode::L,
            TackyBinaryOperator::GreaterThanOrEqual => AsmCondCode::GE,
            TackyBinaryOperator::LessThanOrEqual => AsmCondCode::LE,
            _ => panic!("invalid relational operator for cond code: {op:?}"),
        }
    }
}
