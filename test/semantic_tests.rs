/*!
This file covers: Semantic analysis - VariableResolver (happy/error paths).
Tests variable resolution, duplicate detection, undeclared variables, invalid lvalues.
Does NOT cover: exact renamed identifier format, other semantic passes.
Suggestions: add tests for future semantic features (type checking, etc).
*/

use fcc::c_ast::ast::{Expression, Program, Statement, BlockItem};
use fcc::c_ast::semantic::validate::validate_semantics;
use fcc::lexer::lex;

// Helper: lex, parse, and validate source code
fn validate_program(src: &str) -> Result<Program, String> {
    let tokens = lex(src)?;
    let program = Program::try_from(tokens)?;
    validate_semantics(&program)
}

// Helper: just parse without validation
#[allow(dead_code)]
fn parse_program(src: &str) -> Result<Program, String> {
    let tokens = lex(src)?;
    Program::try_from(tokens)
}

// =============================================================================
// HAPPY PATHS
// =============================================================================

#[test]
fn test_semantic_variable_resolution_basic() {
    let src = "int main(void){ int x=1; return x; }";
    let validated = validate_program(src).expect("should validate");

    // After validation, the variable in return should reference the resolved identifier
    // We just check that the program is valid and has the expected structure
    assert_eq!(validated.function_definition.body.len(), 2);

    // Get the declaration name
    let decl_name = match &validated.function_definition.body[0] {
        BlockItem::D(decl) => decl.name.value.clone(),
        _ => panic!("Expected declaration"),
    };

    // Get the return variable name
    let return_var_name = match &validated.function_definition.body[1] {
        BlockItem::S(Statement::Return(Expression::Var(id))) => id.value.clone(),
        _ => panic!("Expected return with variable"),
    };

    // The resolved names should match (both point to same resolved variable)
    assert_eq!(decl_name, return_var_name, "Declaration and use should have same resolved name");
}

#[test]
fn test_semantic_two_distinct_variables() {
    let src = "int main(void){ int x=1; int y=2; return x + y; }";
    let validated = validate_program(src).expect("should validate");

    // Get both declaration names
    let x_name = match &validated.function_definition.body[0] {
        BlockItem::D(decl) => decl.name.value.clone(),
        _ => panic!("Expected declaration"),
    };

    let y_name = match &validated.function_definition.body[1] {
        BlockItem::D(decl) => decl.name.value.clone(),
        _ => panic!("Expected declaration"),
    };

    // The resolved names should be different
    assert_ne!(x_name, y_name, "Different variables should have different resolved names");
}

#[test]
fn test_semantic_valid_assignment() {
    let src = "int main(void){ int x=1; x = 2; return x; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Valid assignment should pass semantic validation");
}

#[test]
fn test_semantic_variable_in_expression() {
    let src = "int main(void){ int a=1; int b=2; return a + b * a; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Variables in expressions should validate");
}

#[test]
fn test_semantic_variable_in_conditional() {
    let src = "int main(void){ int x=1; return x ? 1 : 0; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Variable in conditional should validate");
}

#[test]
fn test_semantic_variable_in_if_condition() {
    let src = "int main(void){ int x=1; if (x) return 1; return 0; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Variable in if condition should validate");
}

#[test]
fn test_semantic_decl_without_initializer() {
    let src = "int main(void){ int x; x = 5; return x; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Declaration without initializer followed by assignment should validate");
}

#[test]
fn test_semantic_nested_expressions() {
    let src = "int main(void){ int a=1; int b=2; int c=3; return (a + b) * c - a; }";
    let result = validate_program(src);
    assert!(result.is_ok(), "Nested expressions with variables should validate");
}

// =============================================================================
// ERROR PATHS
// =============================================================================

#[test]
fn test_semantic_error_duplicate_declaration() {
    let src = "int main(void){ int x; int x; return 0; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Duplicate declaration should fail");
}

#[test]
fn test_semantic_error_duplicate_declaration_with_init() {
    let src = "int main(void){ int x=1; int x=2; return 0; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Duplicate declaration with initializers should fail");
}

#[test]
fn test_semantic_error_undeclared_variable() {
    let src = "int main(void){ return x; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Undeclared variable should fail");
}

#[test]
fn test_semantic_error_undeclared_in_expression() {
    let src = "int main(void){ int a=1; return a + b; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Undeclared variable in expression should fail");
}

#[test]
fn test_semantic_error_invalid_lvalue_constant() {
    let src = "int main(void){ 1 = 2; return 0; }";
    // This might fail at parse or semantic - either is acceptable
    let result = validate_program(src);
    assert!(result.is_err(), "Assignment to constant should fail");
}

#[test]
fn test_semantic_error_invalid_lvalue_expression() {
    let src = "int main(void){ int x=1; (x+1) = 2; return x; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Assignment to expression should fail");
}

#[test]
fn test_semantic_error_undeclared_in_assignment() {
    let src = "int main(void){ x = 5; return 0; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Assignment to undeclared variable should fail");
}

#[test]
fn test_semantic_error_undeclared_in_if_condition() {
    let src = "int main(void){ if (x) return 1; return 0; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Undeclared variable in if condition should fail");
}

#[test]
fn test_semantic_error_undeclared_in_ternary() {
    let src = "int main(void){ int x=1; return x ? y : 0; }";
    let result = validate_program(src);
    assert!(result.is_err(), "Undeclared variable in ternary should fail");
}

#[test]
fn test_semantic_error_use_before_declaration() {
    // Note: depending on implementation, this might work if declarations are hoisted
    // If not hoisted, should fail
    let src = "int main(void){ return x; int x=1; }";
    let result = validate_program(src);
    // The current implementation likely fails this
    assert!(result.is_err(), "Use before declaration should fail");
}
