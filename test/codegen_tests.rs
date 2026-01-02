/*!
This file covers: Lowering Tacky IR to x64 ASM (codegen/x64/from.rs).
Tests instruction conversion, operator mapping, condition codes.
Does NOT cover: assembly string emission (emit.rs), fixers (already in folder_tests).
Suggestions: add tests for edge cases in register allocation.
*/

use fcc::codegen::x64::ast::{
    AsmBinaryOperator, AsmCondCode, AsmInstruction, AsmOperand, AsmProgram,
};
use fcc::tacky::ast::{
    TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram,
    TackyUnaryOperator, TackyValue,
};

// Helper: convert a Tacky program to ASM
fn lower_to_asm(tacky: TackyProgram) -> AsmProgram {
    AsmProgram::from(tacky)
}

// Helper: create a minimal Tacky program with given instructions
fn make_tacky_program(instructions: Vec<TackyInstruction>) -> TackyProgram {
    TackyProgram::new(TackyFunctionDefinition::new(
        TackyIdentifier {
            value: "main".to_string(),
        },
        instructions,
    ))
}

// Helper: check if ASM instructions contain Ret
fn has_ret(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Ret))
}

// Helper: check if ASM instructions contain Mov
fn has_mov(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Mov(_, _)))
}

// Helper: check if ASM instructions contain Cdq
fn has_cdq(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Cdq))
}

// Helper: check if ASM instructions contain Idiv
fn has_idiv(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Idiv(_)))
}

// Helper: check if ASM instructions contain Cmp
fn has_cmp(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Cmp(_, _)))
}

// Helper: check if ASM instructions contain SetCC with specific code
fn has_setcc(instructions: &[AsmInstruction], code: AsmCondCode) -> bool {
    instructions.iter().any(|i| {
        matches!(i, AsmInstruction::SetCC(c, _) if std::mem::discriminant(c) == std::mem::discriminant(&code))
    })
}

// Helper: check if ASM instructions contain JmpCC
fn has_jmpcc(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::JmpCC(_, _)))
}

// Helper: check if ASM instructions contain Jmp
fn has_jmp(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Jmp(_)))
}

// Helper: check if ASM instructions contain Label
fn has_label(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Label(_)))
}

// Helper: check if ASM instructions contain Binary with specific operator
fn has_binary_op(instructions: &[AsmInstruction], op: AsmBinaryOperator) -> bool {
    instructions.iter().any(|i| {
        matches!(i, AsmInstruction::Binary(o, _, _) if std::mem::discriminant(o) == std::mem::discriminant(&op))
    })
}

// Helper: check if ASM instructions contain Unary
fn has_unary(instructions: &[AsmInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Unary(_, _)))
}

// =============================================================================
// RETURN INSTRUCTION
// =============================================================================

#[test]
fn test_codegen_return_constant() {
    let tacky = make_tacky_program(vec![TackyInstruction::Return(TackyValue::Constant(0))]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Return should generate Mov to AX");
    assert!(has_ret(instructions), "Return should generate Ret");
}

#[test]
fn test_codegen_return_variable() {
    let tacky = make_tacky_program(vec![TackyInstruction::Return(TackyValue::Var(
        TackyIdentifier {
            value: "x".to_string(),
        },
    ))]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Return var should generate Mov");
    assert!(has_ret(instructions), "Return should generate Ret");
}

// =============================================================================
// UNARY INSTRUCTIONS
// =============================================================================

#[test]
fn test_codegen_unary_negate() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Unary(
            TackyUnaryOperator::Negate,
            TackyValue::Constant(5),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Unary should generate Mov");
    assert!(
        has_unary(instructions),
        "Negate should generate Unary instruction"
    );
}

#[test]
fn test_codegen_unary_complement() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Unary(
            TackyUnaryOperator::Complement,
            TackyValue::Constant(5),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(
        has_unary(instructions),
        "Complement should generate Unary instruction"
    );
}

#[test]
fn test_codegen_unary_not_special_case() {
    // Not has special handling: Cmp + Mov(0) + SetCC
    let tacky = make_tacky_program(vec![
        TackyInstruction::Unary(
            TackyUnaryOperator::Not,
            TackyValue::Constant(5),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    // Not should use Cmp + SetCC pattern
    assert!(has_cmp(instructions), "Not should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::E),
        "Not should use SetCC with E (equal to zero)"
    );
}

// =============================================================================
// BINARY INSTRUCTIONS (ADD/SUB/MUL)
// =============================================================================

#[test]
fn test_codegen_binary_add() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Add,
            TackyValue::Constant(1),
            TackyValue::Constant(2),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Binary should generate Mov");
    assert!(
        has_binary_op(instructions, AsmBinaryOperator::Add),
        "Add should generate Add"
    );
}

#[test]
fn test_codegen_binary_subtract() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Subtract,
            TackyValue::Constant(5),
            TackyValue::Constant(3),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(
        has_binary_op(instructions, AsmBinaryOperator::Sub),
        "Subtract should generate Sub"
    );
}

#[test]
fn test_codegen_binary_multiply() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Multiply,
            TackyValue::Constant(2),
            TackyValue::Constant(3),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(
        has_binary_op(instructions, AsmBinaryOperator::Mult),
        "Multiply should generate Mult"
    );
}

// =============================================================================
// BINARY INSTRUCTIONS (DIV/REM)
// =============================================================================

#[test]
fn test_codegen_binary_divide() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Divide,
            TackyValue::Constant(6),
            TackyValue::Constant(2),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    // Division uses Cdq + Idiv pattern
    assert!(has_cdq(instructions), "Division should use Cdq");
    assert!(has_idiv(instructions), "Division should use Idiv");
}

#[test]
fn test_codegen_binary_remainder() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Remainder,
            TackyValue::Constant(7),
            TackyValue::Constant(3),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    // Remainder uses same pattern as division
    assert!(has_cdq(instructions), "Remainder should use Cdq");
    assert!(has_idiv(instructions), "Remainder should use Idiv");
}

// =============================================================================
// BITWISE OPERATORS
// =============================================================================

#[test]
fn test_codegen_bitwise_operators() {
    let cases = [
        (
            TackyBinaryOperator::BitwiseAnd,
            AsmBinaryOperator::BitwiseAnd,
        ),
        (TackyBinaryOperator::BitwiseOr, AsmBinaryOperator::BitwiseOr),
        (
            TackyBinaryOperator::BitwiseXor,
            AsmBinaryOperator::BitwiseXor,
        ),
        (TackyBinaryOperator::LeftShift, AsmBinaryOperator::LeftShift),
        (
            TackyBinaryOperator::RightShift,
            AsmBinaryOperator::RightShift,
        ),
    ];

    for (tacky_op, expected_asm_op) in cases {
        let tacky = make_tacky_program(vec![
            TackyInstruction::Binary(
                tacky_op.clone(),
                TackyValue::Constant(1),
                TackyValue::Constant(2),
                TackyValue::Var(TackyIdentifier {
                    value: "dst".to_string(),
                }),
            ),
            TackyInstruction::Return(TackyValue::Constant(0)),
        ]);

        let asm = lower_to_asm(tacky);
        let instructions = &asm.function_definition.instructions;

        assert!(
            has_binary_op(instructions, expected_asm_op.clone()),
            "Tacky {:?} should generate Asm {:?}",
            tacky_op,
            expected_asm_op
        );
    }
}

// =============================================================================
// RELATIONAL OPERATORS -> CONDITION CODES
// =============================================================================

#[test]
fn test_codegen_relational_equal() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::Equal,
            TackyValue::Constant(1),
            TackyValue::Constant(1),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "Equal should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::E),
        "Equal should use SetCC E"
    );
}

#[test]
fn test_codegen_relational_not_equal() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::NotEqual,
            TackyValue::Constant(1),
            TackyValue::Constant(2),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "NotEqual should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::NE),
        "NotEqual should use SetCC NE"
    );
}

#[test]
fn test_codegen_relational_less_than() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::LessThan,
            TackyValue::Constant(1),
            TackyValue::Constant(2),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "LessThan should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::L),
        "LessThan should use SetCC L"
    );
}

#[test]
fn test_codegen_relational_less_than_or_equal() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::LessThanOrEqual,
            TackyValue::Constant(1),
            TackyValue::Constant(2),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "LessThanOrEqual should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::LE),
        "LessThanOrEqual should use SetCC LE"
    );
}

#[test]
fn test_codegen_relational_greater_than() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::GreaterThan,
            TackyValue::Constant(2),
            TackyValue::Constant(1),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "GreaterThan should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::G),
        "GreaterThan should use SetCC G"
    );
}

#[test]
fn test_codegen_relational_greater_than_or_equal() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Binary(
            TackyBinaryOperator::GreaterThanOrEqual,
            TackyValue::Constant(2),
            TackyValue::Constant(1),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "GreaterThanOrEqual should use Cmp");
    assert!(
        has_setcc(instructions, AsmCondCode::GE),
        "GreaterThanOrEqual should use SetCC GE"
    );
}

// =============================================================================
// JUMP AND LABEL INSTRUCTIONS
// =============================================================================

#[test]
fn test_codegen_jump() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Jump(TackyIdentifier {
            value: "label1".to_string(),
        }),
        TackyInstruction::Label(TackyIdentifier {
            value: "label1".to_string(),
        }),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_jmp(instructions), "Jump should generate Jmp");
    assert!(has_label(instructions), "Label should generate Label");
}

#[test]
fn test_codegen_jump_if_zero() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::JumpIfZero(
            TackyValue::Constant(0),
            TackyIdentifier {
                value: "zero_label".to_string(),
            },
        ),
        TackyInstruction::Label(TackyIdentifier {
            value: "zero_label".to_string(),
        }),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "JumpIfZero should use Cmp");
    assert!(has_jmpcc(instructions), "JumpIfZero should generate JmpCC");
}

#[test]
fn test_codegen_jump_if_not_zero() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::JumpIfNotZero(
            TackyValue::Constant(1),
            TackyIdentifier {
                value: "nonzero_label".to_string(),
            },
        ),
        TackyInstruction::Label(TackyIdentifier {
            value: "nonzero_label".to_string(),
        }),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_cmp(instructions), "JumpIfNotZero should use Cmp");
    assert!(
        has_jmpcc(instructions),
        "JumpIfNotZero should generate JmpCC"
    );
}

// =============================================================================
// COPY INSTRUCTION
// =============================================================================

#[test]
fn test_codegen_copy() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Copy(
            TackyValue::Constant(42),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Copy should generate Mov");
}

#[test]
fn test_codegen_copy_var_to_var() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Copy(
            TackyValue::Var(TackyIdentifier {
                value: "src".to_string(),
            }),
            TackyValue::Var(TackyIdentifier {
                value: "dst".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    assert!(has_mov(instructions), "Copy var to var should generate Mov");
}

// =============================================================================
// OPERAND CONVERSION
// =============================================================================

#[test]
fn test_codegen_operand_immediate() {
    let tacky = make_tacky_program(vec![TackyInstruction::Return(TackyValue::Constant(42))]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    // Check that immediate operand is generated
    let has_imm = instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Mov(AsmOperand::Imm(42), _)));
    assert!(has_imm, "Constant should become Imm operand");
}

#[test]
fn test_codegen_operand_pseudo_register() {
    let tacky = make_tacky_program(vec![
        TackyInstruction::Copy(
            TackyValue::Constant(1),
            TackyValue::Var(TackyIdentifier {
                value: "my_var".to_string(),
            }),
        ),
        TackyInstruction::Return(TackyValue::Constant(0)),
    ]);

    let asm = lower_to_asm(tacky);
    let instructions = &asm.function_definition.instructions;

    // Check that pseudo register is generated
    let has_pseudo = instructions
        .iter()
        .any(|i| matches!(i, AsmInstruction::Mov(_, AsmOperand::Pseudo(_))));
    assert!(has_pseudo, "Var should become Pseudo operand");
}
