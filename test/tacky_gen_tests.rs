/*!
This file covers: Lowering C AST to Tacky IR (tacky/from.rs).
Tests control flow generation for if/else, ternary, short-circuit AND/OR,
while, do-while, for loops, break, and continue statements.
Does NOT cover: exact instruction sequences (too fragile), optimization passes.
Suggestions: add property tests for instruction count bounds.
*/

use fcc::c_ast::ast::Program;
use fcc::driver::validate_semantics;
use fcc::lexer::lex;
use fcc::tacky::ast::{
    TackyBinaryOperator, TackyInstruction, TackyProgram, TackyUnaryOperator, TackyValue,
};

// Helper: full pipeline from source to Tacky IR
fn lower_to_tacky(src: &str) -> Result<TackyProgram, String> {
    let tokens = lex(src)?;
    let program = Program::try_from(tokens)?;
    let validated = validate_semantics(program)?;
    Ok(TackyProgram::from(validated))
}

// Helper: check if instructions contain at least one Return
fn has_return(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::Return(_)))
}

// Helper: check if instructions contain a Jump
fn has_jump(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::Jump(_)))
}

// Helper: check if instructions contain JumpIfZero
fn has_jump_if_zero(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::JumpIfZero(_, _)))
}

// Helper: check if instructions contain JumpIfNotZero
fn has_jump_if_not_zero(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::JumpIfNotZero(_, _)))
}

// Helper: check if instructions contain a Label
fn has_label(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::Label(_)))
}

// Helper: check if instructions contain a Copy
fn has_copy(instructions: &[TackyInstruction]) -> bool {
    instructions
        .iter()
        .any(|i| matches!(i, TackyInstruction::Copy(_, _)))
}

// Helper: check if instructions contain a Binary with specific operator
fn has_binary_op(instructions: &[TackyInstruction], op: &TackyBinaryOperator) -> bool {
    instructions.iter().any(|i| {
        matches!(i, TackyInstruction::Binary(o, _, _, _) if std::mem::discriminant(o) == std::mem::discriminant(op))
    })
}

// Helper: check if instructions contain a Unary with specific operator
fn has_unary_op(instructions: &[TackyInstruction], op: &TackyUnaryOperator) -> bool {
    instructions.iter().any(|i| {
        matches!(i, TackyInstruction::Unary(o, _, _) if std::mem::discriminant(o) == std::mem::discriminant(op))
    })
}

// =============================================================================
// BASIC LOWERING
// =============================================================================

#[test]
fn test_tacky_gen_return_constant() {
    let src = "int main(void){ return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_return(instructions),
        "Should contain Return instruction"
    );
}

#[test]
fn test_tacky_gen_return_constant_nonzero() {
    let src = "int main(void){ return 42; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Should have a return, and somewhere there should be constant 42
    assert!(
        has_return(instructions),
        "Should contain Return instruction"
    );

    // Check that 42 appears somewhere in the instructions
    let has_42 = instructions.iter().any(|i| match i {
        TackyInstruction::Return(TackyValue::Constant(42)) => true,
        TackyInstruction::Copy(TackyValue::Constant(42), _) => true,
        _ => false,
    });
    assert!(has_42, "Should reference constant 42");
}

#[test]
fn test_tacky_gen_unary_negate() {
    let src = "int main(void){ int x=1; return -x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_unary_op(instructions, &TackyUnaryOperator::Negate),
        "Should contain Negate unary"
    );
}

#[test]
fn test_tacky_gen_unary_complement() {
    let src = "int main(void){ int x=1; return ~x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_unary_op(instructions, &TackyUnaryOperator::Complement),
        "Should contain Complement unary"
    );
}

#[test]
fn test_tacky_gen_unary_not() {
    let src = "int main(void){ int x=1; return !x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_unary_op(instructions, &TackyUnaryOperator::Not),
        "Should contain Not unary"
    );
}

// =============================================================================
// BINARY OPERATIONS (NON SHORT-CIRCUIT)
// =============================================================================

#[test]
fn test_tacky_gen_binary_add() {
    let src = "int main(void){ int x=1; int y=2; return x + y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::Add),
        "Should contain Add binary"
    );
}

#[test]
fn test_tacky_gen_binary_subtract() {
    let src = "int main(void){ int x=5; int y=3; return x - y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::Subtract),
        "Should contain Subtract binary"
    );
}

#[test]
fn test_tacky_gen_binary_multiply() {
    let src = "int main(void){ int x=2; int y=3; return x * y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::Multiply),
        "Should contain Multiply binary"
    );
}

#[test]
fn test_tacky_gen_binary_divide() {
    let src = "int main(void){ int x=6; int y=2; return x / y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::Divide),
        "Should contain Divide binary"
    );
}

#[test]
fn test_tacky_gen_binary_remainder() {
    let src = "int main(void){ int x=7; int y=3; return x % y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::Remainder),
        "Should contain Remainder binary"
    );
}

// =============================================================================
// SHORT-CIRCUIT OPERATORS
// =============================================================================

#[test]
fn test_tacky_gen_and_short_circuit() {
    let src = "int main(void){ int a=1; int b=0; return a && b; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // AND short-circuit should generate JumpIfZero and labels
    assert!(
        has_jump_if_zero(instructions),
        "AND should use JumpIfZero for short-circuit"
    );
    assert!(has_label(instructions), "AND should generate labels");
}

#[test]
fn test_tacky_gen_or_short_circuit() {
    let src = "int main(void){ int a=0; int b=1; return a || b; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // OR short-circuit should generate JumpIfNotZero and labels
    assert!(
        has_jump_if_not_zero(instructions),
        "OR should use JumpIfNotZero for short-circuit"
    );
    assert!(has_label(instructions), "OR should generate labels");
}

// =============================================================================
// ASSIGNMENT
// =============================================================================

#[test]
fn test_tacky_gen_assignment() {
    let src = "int main(void){ int x=1; x = 2; return x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Assignment should generate Copy instructions
    assert!(has_copy(instructions), "Assignment should generate Copy");
}

// =============================================================================
// CONDITIONAL (TERNARY)
// =============================================================================

#[test]
fn test_tacky_gen_ternary() {
    let src = "int main(void){ int x=0; return x ? 1 : 2; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Ternary should generate control flow
    assert!(
        has_jump_if_zero(instructions) || has_jump_if_not_zero(instructions),
        "Ternary should generate conditional jump"
    );
    assert!(has_label(instructions), "Ternary should generate labels");
    assert!(
        has_jump(instructions),
        "Ternary should generate unconditional jump"
    );
}

#[test]
fn test_tacky_gen_ternary_with_expressions() {
    let src = "int main(void){ int a=1; int b=2; return a > b ? a : b; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Should have relational comparison and control flow
    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::GreaterThan),
        "Should have GreaterThan comparison"
    );
    assert!(has_label(instructions), "Ternary should generate labels");
}

// =============================================================================
// IF STATEMENTS
// =============================================================================

#[test]
fn test_tacky_gen_if_without_else() {
    let src = "int main(void){ int x=0; if (x) return 1; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // If without else should generate conditional jump and labels
    assert!(has_jump_if_zero(instructions), "If should use JumpIfZero");
    assert!(has_label(instructions), "If should generate labels");
}

#[test]
fn test_tacky_gen_if_with_else() {
    let src = "int main(void){ int x=0; if (x) return 1; else return 2; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // If with else should generate conditional jump, unconditional jump, and labels
    assert!(has_jump_if_zero(instructions), "If should use JumpIfZero");
    assert!(
        has_jump(instructions),
        "If-else should have unconditional jump"
    );
    assert!(has_label(instructions), "If-else should generate labels");

    // Should have multiple labels (at least else_label and end_label)
    let label_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(
        label_count >= 2,
        "If-else should have at least 2 labels, found {}",
        label_count
    );
}

#[test]
fn test_tacky_gen_if_with_complex_condition() {
    let src = "int main(void){ int x=1; int y=2; if (x < y) return 1; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    assert!(
        has_binary_op(instructions, &TackyBinaryOperator::LessThan),
        "Should have LessThan comparison"
    );
    assert!(
        has_jump_if_zero(instructions),
        "If should use conditional jump"
    );
}

// =============================================================================
// RELATIONAL OPERATORS
// =============================================================================

#[test]
fn test_tacky_gen_relational_operators() {
    let cases = [
        (
            "int main(void){ return 1 < 2; }",
            TackyBinaryOperator::LessThan,
        ),
        (
            "int main(void){ return 1 <= 2; }",
            TackyBinaryOperator::LessThanOrEqual,
        ),
        (
            "int main(void){ return 1 > 2; }",
            TackyBinaryOperator::GreaterThan,
        ),
        (
            "int main(void){ return 1 >= 2; }",
            TackyBinaryOperator::GreaterThanOrEqual,
        ),
        (
            "int main(void){ return 1 == 2; }",
            TackyBinaryOperator::Equal,
        ),
        (
            "int main(void){ return 1 != 2; }",
            TackyBinaryOperator::NotEqual,
        ),
    ];

    for (src, ref expected_op) in cases {
        let tacky = lower_to_tacky(src).expect("should lower");
        let instructions = &tacky.function_definition.instructions;
        assert!(
            has_binary_op(instructions, expected_op),
            "Should have {:?} for source: {}",
            expected_op,
            src
        );
    }
}

// =============================================================================
// BITWISE OPERATORS
// =============================================================================

#[test]
fn test_tacky_gen_bitwise_operators() {
    let cases = [
        (
            "int main(void){ int x=1; int y=2; return x & y; }",
            TackyBinaryOperator::BitwiseAnd,
        ),
        (
            "int main(void){ int x=1; int y=2; return x | y; }",
            TackyBinaryOperator::BitwiseOr,
        ),
        (
            "int main(void){ int x=1; int y=2; return x ^ y; }",
            TackyBinaryOperator::BitwiseXor,
        ),
        (
            "int main(void){ int x=1; return x << 2; }",
            TackyBinaryOperator::LeftShift,
        ),
        (
            "int main(void){ int x=8; return x >> 2; }",
            TackyBinaryOperator::RightShift,
        ),
    ];

    for (src, ref expected_op) in cases {
        let tacky = lower_to_tacky(src).expect("should lower");
        let instructions = &tacky.function_definition.instructions;
        assert!(
            has_binary_op(instructions, expected_op),
            "Should have {:?} for source: {}",
            expected_op,
            src
        );
    }
}

// =============================================================================
// DECLARATION LOWERING
// =============================================================================

#[test]
fn test_tacky_gen_declaration_with_init() {
    let src = "int main(void){ int x = 42; return x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Declaration with initializer should generate Copy
    assert!(
        has_copy(instructions),
        "Declaration with init should generate Copy"
    );
}

#[test]
fn test_tacky_gen_declaration_without_init() {
    let src = "int main(void){ int x; x = 5; return x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Should still work and have copies for assignment
    assert!(has_copy(instructions), "Assignment should generate Copy");
    assert!(has_return(instructions), "Should have return");
}

// =============================================================================
// WHILE LOOPS
// =============================================================================

#[test]
fn test_tacky_gen_while_loop_basic() {
    let src = "int main(void){ int x = 0; while (x) x = 1; return x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // While should generate: Label (continue), JumpIfZero (to break), body, Jump (to continue), Label (break)
    assert!(has_label(instructions), "While should generate labels");
    assert!(
        has_jump_if_zero(instructions),
        "While should use JumpIfZero for condition"
    );
    assert!(
        has_jump(instructions),
        "While should have unconditional jump back to start"
    );

    // Should have at least 2 labels (continue and break)
    let label_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(
        label_count >= 2,
        "While should have at least 2 labels (continue, break), found {}",
        label_count
    );
}

#[test]
fn test_tacky_gen_while_with_break() {
    let src = "int main(void){ while (1) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Break generates a Jump instruction
    let jump_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Jump(_)))
        .count();
    assert!(
        jump_count >= 2,
        "While with break should have at least 2 jumps (loop back + break)"
    );
}

#[test]
fn test_tacky_gen_while_with_continue() {
    let src = "int main(void){ int x = 0; while (x) { continue; x = 1; } return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Continue generates a Jump instruction to continue label
    let jump_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Jump(_)))
        .count();
    assert!(
        jump_count >= 2,
        "While with continue should have at least 2 jumps"
    );
}

// =============================================================================
// DO-WHILE LOOPS
// =============================================================================

#[test]
fn test_tacky_gen_do_while_basic() {
    let src = "int main(void){ int x = 0; do x = 1; while (x); return x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Do-while should generate: Label (start), body, Label (continue), JumpIfNotZero (to start), Label (break)
    assert!(has_label(instructions), "Do-while should generate labels");
    assert!(
        has_jump_if_not_zero(instructions),
        "Do-while should use JumpIfNotZero for condition"
    );

    // Should have at least 3 labels (start, continue, break)
    let label_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(
        label_count >= 3,
        "Do-while should have at least 3 labels, found {}",
        label_count
    );
}

#[test]
fn test_tacky_gen_do_while_with_break() {
    let src = "int main(void){ do break; while (1); return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    assert!(
        has_jump(instructions),
        "Do-while with break should have jump"
    );
    assert!(has_label(instructions), "Do-while should have labels");
}

#[test]
fn test_tacky_gen_do_while_with_continue() {
    let src = "int main(void){ int x = 1; do { continue; x = 0; } while (x); return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Continue in do-while jumps to the continue label (before condition check)
    assert!(
        has_jump(instructions),
        "Do-while with continue should have jumps"
    );
}

// =============================================================================
// FOR LOOPS
// =============================================================================

#[test]
fn test_tacky_gen_for_loop_full() {
    let src = "int main(void){ for (int i = 0; i; i) return i; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // For should generate: init, Label (start), cond check, JumpIfZero (to break), body, Label (continue), post, Jump (to start), Label (break)
    assert!(has_label(instructions), "For should generate labels");
    assert!(
        has_jump_if_zero(instructions),
        "For should use JumpIfZero for condition"
    );
    assert!(has_jump(instructions), "For should have unconditional jump");
    assert!(has_copy(instructions), "For should have copies for init");
}

#[test]
fn test_tacky_gen_for_loop_empty_init() {
    let src = "int main(void){ int i = 0; for (; i; i) return i; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    assert!(has_label(instructions), "For should generate labels");
    assert!(has_jump_if_zero(instructions), "For should check condition");
}

#[test]
fn test_tacky_gen_for_loop_empty_cond() {
    let src = "int main(void){ for (int i = 0;; i) { break; } return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Without condition, there's no JumpIfZero for the condition itself
    assert!(has_label(instructions), "For should generate labels");
    assert!(has_jump(instructions), "For should have jumps");
}

#[test]
fn test_tacky_gen_for_loop_empty_post() {
    let src = "int main(void){ for (int i = 0; i;) { break; } return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    assert!(has_label(instructions), "For should generate labels");
}

#[test]
fn test_tacky_gen_for_loop_infinite() {
    let src = "int main(void){ for (;;) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Infinite for loop still has labels and jump structure
    assert!(has_label(instructions), "Infinite for should have labels");
    assert!(has_jump(instructions), "Infinite for should have jumps");
}

#[test]
fn test_tacky_gen_for_with_break() {
    let src = "int main(void){ for (int i = 0; i; i) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Break in for jumps to break label
    let jump_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Jump(_)))
        .count();
    assert!(jump_count >= 2, "For with break should have multiple jumps");
}

#[test]
fn test_tacky_gen_for_with_continue() {
    let src = "int main(void){ for (int i = 0; i; i) continue; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Continue in for jumps to continue label (before post expression)
    let jump_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Jump(_)))
        .count();
    assert!(
        jump_count >= 2,
        "For with continue should have multiple jumps"
    );
}

#[test]
fn test_tacky_gen_for_init_with_expression() {
    let src = "int main(void){ int i; for (i = 0; i; i) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // For with expression init should generate copy
    assert!(
        has_copy(instructions),
        "For with expression init should generate copy"
    );
}

// =============================================================================
// NESTED LOOPS
// =============================================================================

#[test]
fn test_tacky_gen_nested_while_loops() {
    let src = "int main(void){ while (1) while (1) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Nested loops should have multiple sets of labels
    let label_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(
        label_count >= 4,
        "Nested while loops should have at least 4 labels, found {}",
        label_count
    );
}

#[test]
fn test_tacky_gen_nested_for_loops() {
    let src = "int main(void){ for (int i = 0; i; i) for (int j = 0; j; j) break; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    let label_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(
        label_count >= 4,
        "Nested for loops should have multiple labels, found {}",
        label_count
    );
}

#[test]
fn test_tacky_gen_break_in_inner_loop() {
    let src = "int main(void){ while (1) { while (1) break; break; } return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;

    // Each break should jump to its own loop's break label
    let jump_count = instructions
        .iter()
        .filter(|i| matches!(i, TackyInstruction::Jump(_)))
        .count();
    assert!(
        jump_count >= 4,
        "Nested loops with breaks should have multiple jumps"
    );
}
