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
            TackyInstruction::Binary(op, src_1, src_2, dst) => {
                let is_div = op == TackyBinaryOperator::Divide;
                match &op {
                    // “addition, subtraction, and multiplication,
                    // we convert a single TACKY instruction into two assembly instructions”
                    TackyBinaryOperator::Add
                    | TackyBinaryOperator::Subtract
                    | TackyBinaryOperator::Multiply
                    // bitwise operators
                    | TackyBinaryOperator::BitwiseAnd
                    | TackyBinaryOperator::BitwiseOr
                    | TackyBinaryOperator::BitwiseXor
                    | TackyBinaryOperator::LeftShift
                    | TackyBinaryOperator::RightShift => vec![
                        AsmInstruction::Comment(format!("add/sub/mul/bitwise operator: {:?}", op.clone())),
                        AsmInstruction::Mov(AsmOperand::from(src_1), AsmOperand::from(dst.clone())),
                        AsmInstruction::Binary(
                            AsmBinaryOperator::from(op),
                            AsmOperand::from(src_2),
                            AsmOperand::from(dst),
                        ),
                    ],
                    // relational operators
                    TackyBinaryOperator::Equal
                    | TackyBinaryOperator::NotEqual
                    | TackyBinaryOperator::GreaterThan
                    | TackyBinaryOperator::LessThan
                    | TackyBinaryOperator::LessThanOrEqual
                    | TackyBinaryOperator::GreaterThanOrEqual => vec![
                        AsmInstruction::Comment(format!("relational operator: {:?}", op.clone())),
                        AsmInstruction::Cmp(AsmOperand::from(src_2), AsmOperand::from(src_1)),
                        AsmInstruction::Mov(AsmOperand::Imm(0), AsmOperand::from(dst.clone())),
                        AsmInstruction::SetCC(AsmCondCode::from(op), AsmOperand::from(dst)),
                    ],
                    TackyBinaryOperator::Divide | TackyBinaryOperator::Remainder => {
                        let reg = if is_div { AsmOperand::Register(Reg::AX) } else { AsmOperand::Register(Reg::DX) };
                        vec![
                            AsmInstruction::Comment(format!("div/rem operator: {:?}", op.clone())),
                            AsmInstruction::Mov(
                                AsmOperand::from(src_1),
                                AsmOperand::Register(Reg::AX),
                            ),
                            AsmInstruction::Cdq,
                            AsmInstruction::Idiv(AsmOperand::from(src_2)),
                            AsmInstruction::Mov(reg, AsmOperand::from(dst)),
                        ]
                    }
                }
            }
            TackyInstruction::Jump(id) => vec![AsmInstruction::Jmp(AsmIdentifier::from(id))],
            // TODO: this is almost same as JumpIfNotZero
            TackyInstruction::JumpIfZero(condition, target) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(condition)),
                AsmInstruction::JmpCC(AsmCondCode::E, AsmIdentifier::from(target)),
            ],
            TackyInstruction::JumpIfNotZero(condition, target) => vec![
                AsmInstruction::Cmp(AsmOperand::Imm(0), AsmOperand::from(condition)),
                AsmInstruction::JmpCC(AsmCondCode::NE, AsmIdentifier::from(target)),
            ],
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
            // logical unary operators
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
