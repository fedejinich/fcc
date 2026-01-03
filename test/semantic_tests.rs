/*!
This file covers: Semantic analysis - VariableResolver and LoopLabeler (happy/error paths).
Tests variable resolution, duplicate detection, undeclared variables, invalid lvalues.
Tests loop labeling for while, do-while, for loops and break/continue statements.
Does NOT cover: exact renamed identifier format.
Suggestions: add tests for future semantic features (type checking, etc).
*/

use fcc::c_ast::ast::{BlockItem, Expression, Identifier, Program, Statement};
use fcc::c_ast::semantic::loop_lab::LoopLabeler;
use fcc::common::folder::FolderC;
use fcc::driver::validate_semantics;
use fcc::lexer::lex;

// Helper: lex, parse, and validate source code
fn validate_program(src: &str) -> Result<Program, String> {
    let tokens = lex(src)?;
    let program = Program::try_from(tokens)?;
    validate_semantics(program)
}

// Helper: lex, parse, and apply loop labeling
fn label_loops(src: &str) -> Result<Program, String> {
    let tokens = lex(src)?;
    let program = Program::try_from(tokens)?;
    let mut labeler = LoopLabeler::new(Identifier::new("dummy".to_string()));
    labeler.fold_prog(program)
}

// Helper: get block items as a vector from a validated program
fn get_body_items(program: &Program) -> Vec<&BlockItem> {
    program.function_definition().body().iter().collect()
}

// =============================================================================
// HAPPY PATHS
// =============================================================================

#[test]
fn test_semantic_variable_resolution_basic() {
    let src = "int main(void){ int x=1; return x; }";
    let validated = validate_program(src).expect("should validate");

    let items = get_body_items(&validated);
    assert_eq!(items.len(), 2);

    // Get the declaration name
    let decl_name = match items[0] {
        BlockItem::D(decl) => decl.name().value().to_string(),
        _ => panic!("Expected declaration"),
    };

    // Get the return variable name
    let return_var_name = match items[1] {
        BlockItem::S(Statement::Return(Expression::Var(id))) => id.value().to_string(),
        _ => panic!("Expected return with variable"),
    };

    // The resolved names should match (both point to same resolved variable)
    assert_eq!(
        decl_name, return_var_name,
        "Declaration and use should have same resolved name"
    );
}

#[test]
fn test_semantic_two_distinct_variables() {
    let src = "int main(void){ int x=1; int y=2; return x + y; }";
    let validated = validate_program(src).expect("should validate");

    let items = get_body_items(&validated);

    // Get both declaration names
    let x_name = match items[0] {
        BlockItem::D(decl) => decl.name().value().to_string(),
        _ => panic!("Expected declaration"),
    };

    let y_name = match items[1] {
        BlockItem::D(decl) => decl.name().value().to_string(),
        _ => panic!("Expected declaration"),
    };

    // The resolved names should be different
    assert_ne!(
        x_name, y_name,
        "Different variables should have different resolved names"
    );
}

#[test]
fn test_semantic_valid_assignment() {
    let src = "int main(void){ int x=1; x = 2; return x; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Valid assignment should pass semantic validation"
    );
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
    assert!(
        result.is_ok(),
        "Declaration without initializer followed by assignment should validate"
    );
}

#[test]
fn test_semantic_nested_expressions() {
    let src = "int main(void){ int a=1; int b=2; int c=3; return (a + b) * c - a; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Nested expressions with variables should validate"
    );
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
    assert!(
        result.is_err(),
        "Duplicate declaration with initializers should fail"
    );
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
    assert!(
        result.is_err(),
        "Undeclared variable in expression should fail"
    );
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
    assert!(
        result.is_err(),
        "Assignment to undeclared variable should fail"
    );
}

#[test]
fn test_semantic_error_undeclared_in_if_condition() {
    let src = "int main(void){ if (x) return 1; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in if condition should fail"
    );
}

#[test]
fn test_semantic_error_undeclared_in_ternary() {
    let src = "int main(void){ int x=1; return x ? y : 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in ternary should fail"
    );
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

// =============================================================================
// LOOP LABELING - HAPPY PATHS
// =============================================================================

#[test]
fn test_loop_labeling_while_gets_label() {
    let src = "int main(void){ while (1) return 0; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    match items[0] {
        BlockItem::S(Statement::While(_, _, label)) => {
            assert!(
                label.value().starts_with("loop_st"),
                "While loop should have a loop label"
            );
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_loop_labeling_do_while_gets_label() {
    let src = "int main(void){ do return 0; while (1); return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    match items[0] {
        BlockItem::S(Statement::DoWhile(_, _, label)) => {
            assert!(
                label.value().starts_with("loop_st"),
                "Do-while loop should have a loop label"
            );
        }
        _ => panic!("Expected do-while statement"),
    }
}

#[test]
fn test_loop_labeling_for_gets_label() {
    let src = "int main(void){ for (;;) return 0; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    match items[0] {
        BlockItem::S(Statement::For(_, _, _, _, label)) => {
            assert!(
                label.value().starts_with("loop_st"),
                "For loop should have a loop label"
            );
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_loop_labeling_break_inherits_label() {
    let src = "int main(void){ while (1) break; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    let loop_label = match items[0] {
        BlockItem::S(Statement::While(_, _, label)) => label.value().to_string(),
        _ => panic!("Expected while statement"),
    };

    let break_label = match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => match body.as_ref() {
            Statement::Break(label) => label.value().to_string(),
            _ => panic!("Expected break statement in body"),
        },
        _ => panic!("Expected while statement"),
    };

    assert_eq!(
        loop_label, break_label,
        "Break should inherit the enclosing loop's label"
    );
}

#[test]
fn test_loop_labeling_continue_inherits_label() {
    let src = "int main(void){ while (1) continue; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    let loop_label = match items[0] {
        BlockItem::S(Statement::While(_, _, label)) => label.value().to_string(),
        _ => panic!("Expected while statement"),
    };

    let continue_label = match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => match body.as_ref() {
            Statement::Continue(label) => label.value().to_string(),
            _ => panic!("Expected continue statement in body"),
        },
        _ => panic!("Expected while statement"),
    };

    assert_eq!(
        loop_label, continue_label,
        "Continue should inherit the enclosing loop's label"
    );
}

#[test]
fn test_loop_labeling_nested_loops_different_labels() {
    let src = "int main(void){ while (1) while (1) return 0; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    let outer_label = match items[0] {
        BlockItem::S(Statement::While(_, _, label)) => label.value().to_string(),
        _ => panic!("Expected outer while statement"),
    };

    let inner_label = match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => match body.as_ref() {
            Statement::While(_, _, label) => label.value().to_string(),
            _ => panic!("Expected inner while statement"),
        },
        _ => panic!("Expected outer while statement"),
    };

    assert_ne!(
        outer_label, inner_label,
        "Nested loops should have different labels"
    );
}

#[test]
fn test_loop_labeling_break_in_nested_loop() {
    let src = "int main(void){ while (1) while (1) break; return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    // Get inner loop label
    let inner_label = match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => match body.as_ref() {
            Statement::While(_, _, label) => label.value().to_string(),
            _ => panic!("Expected inner while"),
        },
        _ => panic!("Expected outer while"),
    };

    // Get break label
    let break_label = match items[0] {
        BlockItem::S(Statement::While(_, outer_body, _)) => match outer_body.as_ref() {
            Statement::While(_, inner_body, _) => match inner_body.as_ref() {
                Statement::Break(label) => label.value().to_string(),
                _ => panic!("Expected break"),
            },
            _ => panic!("Expected inner while"),
        },
        _ => panic!("Expected outer while"),
    };

    assert_eq!(
        inner_label, break_label,
        "Break should have the innermost loop's label"
    );
}

#[test]
fn test_loop_labeling_for_with_break_continue() {
    let src = "int main(void){ for (;;) { break; continue; } return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    let for_label = match items[0] {
        BlockItem::S(Statement::For(_, _, _, _, label)) => label.value().to_string(),
        _ => panic!("Expected for statement"),
    };

    assert!(
        for_label.starts_with("loop_st"),
        "For loop should have a loop label"
    );
}

#[test]
fn test_loop_labeling_do_while_with_break() {
    let src = "int main(void){ do break; while (1); return 0; }";
    let labeled = label_loops(src).expect("should label");
    let items = get_body_items(&labeled);

    let loop_label = match items[0] {
        BlockItem::S(Statement::DoWhile(_, _, label)) => label.value().to_string(),
        _ => panic!("Expected do-while statement"),
    };

    let break_label = match items[0] {
        BlockItem::S(Statement::DoWhile(body, _, _)) => match body.as_ref() {
            Statement::Break(label) => label.value().to_string(),
            _ => panic!("Expected break statement"),
        },
        _ => panic!("Expected do-while statement"),
    };

    assert_eq!(
        loop_label, break_label,
        "Break in do-while should have the loop's label"
    );
}

// =============================================================================
// VARIABLE RESOLUTION WITH LOOPS - HAPPY PATHS
// =============================================================================

#[test]
fn test_semantic_for_loop_variable_in_init() {
    let src = "int main(void){ for (int i = 0; i; i) return i; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Variable declared in for init should be visible in condition, post, and body"
    );
}

#[test]
fn test_semantic_for_loop_init_shadows_outer() {
    let src = "int main(void){ int i = 1; for (int i = 0; i; i) return i; return i; }";
    let validated = validate_program(src).expect("should validate");
    let items = get_body_items(&validated);

    // Get outer i name
    let outer_name = match items[0] {
        BlockItem::D(decl) => decl.name().value().to_string(),
        _ => panic!("Expected outer declaration"),
    };

    // Get the return statement after for loop - should use outer i
    let return_name = match items[2] {
        BlockItem::S(Statement::Return(Expression::Var(id))) => id.value().to_string(),
        _ => panic!("Expected return with variable"),
    };

    assert_eq!(
        outer_name, return_name,
        "Return after for should use outer variable"
    );
}

#[test]
fn test_semantic_for_loop_init_not_visible_outside() {
    // Variable declared in for init should not be visible after the loop
    let src = "int main(void){ for (int i = 0; i; i) return 0; return i; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Variable declared in for init should not be visible outside"
    );
}

#[test]
fn test_semantic_while_loop_with_variable() {
    let src = "int main(void){ int x = 1; while (x) x = 0; return x; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Variable in while condition should validate"
    );
}

#[test]
fn test_semantic_do_while_with_variable() {
    let src = "int main(void){ int x = 1; do x = 0; while (x); return x; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Variable in do-while body and condition should validate"
    );
}

#[test]
fn test_semantic_nested_for_loops_scoping() {
    let src =
        "int main(void){ for (int i = 0; i; i) for (int j = 0; j; j) return i + j; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Nested for loops with different variables should validate"
    );
}

#[test]
fn test_semantic_for_loop_body_new_scope() {
    let src = "int main(void){ int x = 1; for (;;) { int x = 2; return x; } return x; }";
    let result = validate_program(src);
    assert!(
        result.is_ok(),
        "Variable in for body should shadow outer variable"
    );
}

// =============================================================================
// VARIABLE RESOLUTION WITH LOOPS - ERROR PATHS
// =============================================================================

#[test]
fn test_semantic_error_undeclared_in_while_condition() {
    let src = "int main(void){ while (x) return 0; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in while condition should fail"
    );
}

#[test]
fn test_semantic_error_undeclared_in_for_condition() {
    let src = "int main(void){ for (; x; ) return 0; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in for condition should fail"
    );
}

#[test]
fn test_semantic_error_undeclared_in_for_post() {
    let src = "int main(void){ for (;; x) return 0; return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in for post expression should fail"
    );
}

#[test]
fn test_semantic_error_undeclared_in_do_while_condition() {
    let src = "int main(void){ do return 0; while (x); return 0; }";
    let result = validate_program(src);
    assert!(
        result.is_err(),
        "Undeclared variable in do-while condition should fail"
    );
}
