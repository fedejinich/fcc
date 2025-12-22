/*!
This file covers: Lowering C AST to Tacky IR (tacky/from.rs).
Tests control flow generation for if/else, ternary, short-circuit AND/OR.
Does NOT cover: exact instruction sequences (too fragile), optimization passes.
Suggestions: add property tests for instruction count bounds.
*/

use fcc::c_ast::ast::Program;
use fcc::c_ast::semantic::validate::validate_semantics;
use fcc::lexer::lex;
use fcc::tacky::ast::{TackyInstruction, TackyProgram, TackyBinaryOperator, TackyUnaryOperator, TackyValue};

// Helper: full pipeline from source to Tacky IR
fn lower_to_tacky(src: &str) -> Result<TackyProgram, String> {
    let tokens = lex(src)?;
    let program = Program::try_from(tokens)?;
    let validated = validate_semantics(&program)?;
    Ok(TackyProgram::from(validated))
}

// Helper: check if instructions contain at least one Return
fn has_return(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::Return(_)))
}

// Helper: check if instructions contain a Jump
fn has_jump(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::Jump(_)))
}

// Helper: check if instructions contain JumpIfZero
fn has_jump_if_zero(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::JumpIfZero(_, _)))
}

// Helper: check if instructions contain JumpIfNotZero
fn has_jump_if_not_zero(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::JumpIfNotZero(_, _)))
}

// Helper: check if instructions contain a Label
fn has_label(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::Label(_)))
}

// Helper: check if instructions contain a Copy
fn has_copy(instructions: &[TackyInstruction]) -> bool {
    instructions.iter().any(|i| matches!(i, TackyInstruction::Copy(_, _)))
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
    assert!(has_return(instructions), "Should contain Return instruction");
}

#[test]
fn test_tacky_gen_return_constant_nonzero() {
    let src = "int main(void){ return 42; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    
    // Should have a return, and somewhere there should be constant 42
    assert!(has_return(instructions), "Should contain Return instruction");
    
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
    assert!(has_unary_op(instructions, &TackyUnaryOperator::Negate), "Should contain Negate unary");
}

#[test]
fn test_tacky_gen_unary_complement() {
    let src = "int main(void){ int x=1; return ~x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_unary_op(instructions, &TackyUnaryOperator::Complement), "Should contain Complement unary");
}

#[test]
fn test_tacky_gen_unary_not() {
    let src = "int main(void){ int x=1; return !x; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_unary_op(instructions, &TackyUnaryOperator::Not), "Should contain Not unary");
}

// =============================================================================
// BINARY OPERATIONS (NON SHORT-CIRCUIT)
// =============================================================================

#[test]
fn test_tacky_gen_binary_add() {
    let src = "int main(void){ int x=1; int y=2; return x + y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_binary_op(instructions, &TackyBinaryOperator::Add), "Should contain Add binary");
}

#[test]
fn test_tacky_gen_binary_subtract() {
    let src = "int main(void){ int x=5; int y=3; return x - y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_binary_op(instructions, &TackyBinaryOperator::Subtract), "Should contain Subtract binary");
}

#[test]
fn test_tacky_gen_binary_multiply() {
    let src = "int main(void){ int x=2; int y=3; return x * y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_binary_op(instructions, &TackyBinaryOperator::Multiply), "Should contain Multiply binary");
}

#[test]
fn test_tacky_gen_binary_divide() {
    let src = "int main(void){ int x=6; int y=2; return x / y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_binary_op(instructions, &TackyBinaryOperator::Divide), "Should contain Divide binary");
}

#[test]
fn test_tacky_gen_binary_remainder() {
    let src = "int main(void){ int x=7; int y=3; return x % y; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    assert!(has_binary_op(instructions, &TackyBinaryOperator::Remainder), "Should contain Remainder binary");
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
    assert!(has_jump_if_zero(instructions), "AND should use JumpIfZero for short-circuit");
    assert!(has_label(instructions), "AND should generate labels");
}

#[test]
fn test_tacky_gen_or_short_circuit() {
    let src = "int main(void){ int a=0; int b=1; return a || b; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    
    // OR short-circuit should generate JumpIfNotZero and labels
    assert!(has_jump_if_not_zero(instructions), "OR should use JumpIfNotZero for short-circuit");
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
    assert!(has_jump_if_zero(instructions) || has_jump_if_not_zero(instructions), 
            "Ternary should generate conditional jump");
    assert!(has_label(instructions), "Ternary should generate labels");
    assert!(has_jump(instructions), "Ternary should generate unconditional jump");
}

#[test]
fn test_tacky_gen_ternary_with_expressions() {
    let src = "int main(void){ int a=1; int b=2; return a > b ? a : b; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    
    // Should have relational comparison and control flow
    assert!(has_binary_op(instructions, &TackyBinaryOperator::GreaterThan), 
            "Should have GreaterThan comparison");
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
    assert!(has_jump(instructions), "If-else should have unconditional jump");
    assert!(has_label(instructions), "If-else should generate labels");
    
    // Should have multiple labels (at least else_label and end_label)
    let label_count = instructions.iter()
        .filter(|i| matches!(i, TackyInstruction::Label(_)))
        .count();
    assert!(label_count >= 2, "If-else should have at least 2 labels, found {}", label_count);
}

#[test]
fn test_tacky_gen_if_with_complex_condition() {
    let src = "int main(void){ int x=1; int y=2; if (x < y) return 1; return 0; }";
    let tacky = lower_to_tacky(src).expect("should lower");

    let instructions = &tacky.function_definition.instructions;
    
    assert!(has_binary_op(instructions, &TackyBinaryOperator::LessThan), 
            "Should have LessThan comparison");
    assert!(has_jump_if_zero(instructions), "If should use conditional jump");
}

// =============================================================================
// RELATIONAL OPERATORS
// =============================================================================

#[test]
fn test_tacky_gen_relational_operators() {
    let cases = [
        ("int main(void){ return 1 < 2; }", TackyBinaryOperator::LessThan),
        ("int main(void){ return 1 <= 2; }", TackyBinaryOperator::LessThanOrEqual),
        ("int main(void){ return 1 > 2; }", TackyBinaryOperator::GreaterThan),
        ("int main(void){ return 1 >= 2; }", TackyBinaryOperator::GreaterThanOrEqual),
        ("int main(void){ return 1 == 2; }", TackyBinaryOperator::Equal),
        ("int main(void){ return 1 != 2; }", TackyBinaryOperator::NotEqual),
    ];

    for (src, ref expected_op) in cases {
        let tacky = lower_to_tacky(src).expect("should lower");
        let instructions = &tacky.function_definition.instructions;
        assert!(has_binary_op(instructions, expected_op), 
                "Should have {:?} for source: {}", expected_op, src);
    }
}

// =============================================================================
// BITWISE OPERATORS
// =============================================================================

#[test]
fn test_tacky_gen_bitwise_operators() {
    let cases = [
        ("int main(void){ int x=1; int y=2; return x & y; }", TackyBinaryOperator::BitwiseAnd),
        ("int main(void){ int x=1; int y=2; return x | y; }", TackyBinaryOperator::BitwiseOr),
        ("int main(void){ int x=1; int y=2; return x ^ y; }", TackyBinaryOperator::BitwiseXor),
        ("int main(void){ int x=1; return x << 2; }", TackyBinaryOperator::LeftShift),
        ("int main(void){ int x=8; return x >> 2; }", TackyBinaryOperator::RightShift),
    ];

    for (src, ref expected_op) in cases {
        let tacky = lower_to_tacky(src).expect("should lower");
        let instructions = &tacky.function_definition.instructions;
        assert!(has_binary_op(instructions, expected_op), 
                "Should have {:?} for source: {}", expected_op, src);
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
    assert!(has_copy(instructions), "Declaration with init should generate Copy");
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
