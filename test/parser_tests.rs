/*!
This file covers: Parser (happy/error paths), precedence, if/else, ternary, unary ops.
Does NOT cover: pretty print / display, exact error messages.
Suggestions: add property tests if proptest is incorporated.
*/

use fcc::c_ast::ast::{BinaryOperator, BlockItem, Expression, Program, Statement, UnaryOperator};
use fcc::lexer::lex;

// Helper: lex and parse source code into a Program
fn parse_program(src: &str) -> Result<Program, String> {
    let tokens = lex(src)?;
    Program::try_from(tokens)
}

// Helper: get block items as a vector from a parsed program
fn get_body_items(program: &Program) -> Vec<&BlockItem> {
    program.function_definition.body.iter().collect()
}

// =============================================================================
// HAPPY PATHS
// =============================================================================

#[test]
fn test_parser_simple_return() {
    let src = "int main(void){ return 0; }";
    let program = parse_program(src).expect("should parse");

    assert_eq!(program.function_definition.name.value, "main");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 1);

    match items[0] {
        BlockItem::S(Statement::Return(Expression::Constant(0))) => {}
        _ => panic!("Expected return 0 statement"),
    }
}

#[test]
fn test_parser_decl_without_initializer() {
    let src = "int main(void){ int x; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 2);

    match items[0] {
        BlockItem::D(decl) => {
            assert_eq!(decl.name.value, "x");
            assert!(decl.initializer.is_none());
        }
        _ => panic!("Expected declaration"),
    }
}

#[test]
fn test_parser_decl_with_initializer() {
    let src = "int main(void){ int x = 1; return x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 2);

    match items[0] {
        BlockItem::D(decl) => {
            assert_eq!(decl.name.value, "x");
            assert!(matches!(decl.initializer, Some(Expression::Constant(1))));
        }
        _ => panic!("Expected declaration with initializer"),
    }

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Var(id))) => {
            assert_eq!(id.value, "x");
        }
        _ => panic!("Expected return x statement"),
    }
}

#[test]
fn test_parser_if_without_else() {
    let src = "int main(void){ int x=0; if (x) return 1; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 3);

    match items[1] {
        BlockItem::S(Statement::If(cond, then_branch, else_branch)) => {
            assert!(matches!(**cond, Expression::Var(_)));
            assert!(matches!(**then_branch, Statement::Return(_)));
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected if statement without else"),
    }
}

#[test]
fn test_parser_if_with_else() {
    let src = "int main(void){ int x=0; if (x) return 1; else return 2; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 2);

    match items[1] {
        BlockItem::S(Statement::If(_, _, else_branch)) => {
            assert!(else_branch.is_some());
            match else_branch.as_ref().map(|b| b.as_ref()) {
                Some(Statement::Return(Expression::Constant(2))) => {}
                _ => panic!("Expected else branch with return 2"),
            }
        }
        _ => panic!("Expected if statement with else"),
    }
}

#[test]
fn test_parser_dangling_else() {
    // The else should belong to the inner if
    let src = "int main(void){ int x=0; if (x) if (x) return 1; else return 2; return 3; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    // Find the outer if
    match items[1] {
        BlockItem::S(Statement::If(_, then_branch, outer_else)) => {
            // Outer if should NOT have else (dangling else goes to inner)
            assert!(outer_else.is_none(), "Outer if should not have else");

            // Inner if should have else
            match then_branch.as_ref() {
                Statement::If(_, _, inner_else) => {
                    assert!(inner_else.is_some(), "Inner if should have else");
                }
                _ => panic!("Expected inner if statement"),
            }
        }
        _ => panic!("Expected outer if statement"),
    }
}

#[test]
fn test_parser_ternary() {
    let src = "int main(void){ int x=0; return x ? 1 : 2; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Conditional(cond, then_expr, else_expr))) => {
            assert!(matches!(**cond, Expression::Var(_)));
            assert!(matches!(**then_expr, Expression::Constant(1)));
            assert!(matches!(**else_expr, Expression::Constant(2)));
        }
        _ => panic!("Expected return with ternary expression"),
    }
}

#[test]
fn test_parser_precedence_mult_before_add() {
    // 1 + 2 * 3 should be parsed as 1 + (2 * 3)
    let src = "int main(void){ return 1 + 2 * 3; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::Return(Expression::Binary(BinaryOperator::Add, left, right))) => {
            assert!(matches!(**left, Expression::Constant(1)));
            // right should be 2 * 3
            match right.as_ref() {
                Expression::Binary(BinaryOperator::Multiply, l, r) => {
                    assert!(matches!(**l, Expression::Constant(2)));
                    assert!(matches!(**r, Expression::Constant(3)));
                }
                _ => panic!("Expected multiplication on right side"),
            }
        }
        _ => panic!("Expected return with binary expression"),
    }
}

#[test]
fn test_parser_precedence_parens() {
    // (1 + 2) * 3 should respect parentheses
    let src = "int main(void){ return (1 + 2) * 3; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::Return(Expression::Binary(
            BinaryOperator::Multiply,
            left,
            right,
        ))) => {
            // left should be 1 + 2
            match left.as_ref() {
                Expression::Binary(BinaryOperator::Add, l, r) => {
                    assert!(matches!(**l, Expression::Constant(1)));
                    assert!(matches!(**r, Expression::Constant(2)));
                }
                _ => panic!("Expected addition on left side"),
            }
            assert!(matches!(**right, Expression::Constant(3)));
        }
        _ => panic!("Expected return with binary expression"),
    }
}

#[test]
fn test_parser_unary_negate() {
    let src = "int main(void){ int x=1; return -x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Unary(UnaryOperator::Negate, inner))) => {
            assert!(matches!(**inner, Expression::Var(_)));
        }
        _ => panic!("Expected return with unary negate"),
    }
}

#[test]
fn test_parser_unary_complement() {
    let src = "int main(void){ int x=1; return ~x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Unary(UnaryOperator::Complement, _))) => {}
        _ => panic!("Expected return with unary complement"),
    }
}

#[test]
fn test_parser_unary_not() {
    let src = "int main(void){ int x=1; return !x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Unary(UnaryOperator::Not, _))) => {}
        _ => panic!("Expected return with unary not"),
    }
}

#[test]
fn test_parser_unary_chained() {
    let src = "int main(void){ int x=1; return -~x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Unary(UnaryOperator::Negate, inner))) => {
            assert!(matches!(
                **inner,
                Expression::Unary(UnaryOperator::Complement, _)
            ));
        }
        _ => panic!("Expected return with chained unary operators"),
    }
}

#[test]
fn test_parser_logical_and() {
    let src = "int main(void){ int a=1; int b=0; return a && b; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[2] {
        BlockItem::S(Statement::Return(Expression::Binary(BinaryOperator::And, _, _))) => {}
        _ => panic!("Expected return with AND expression"),
    }
}

#[test]
fn test_parser_logical_or() {
    let src = "int main(void){ int a=0; int b=1; return a || b; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[2] {
        BlockItem::S(Statement::Return(Expression::Binary(BinaryOperator::Or, _, _))) => {}
        _ => panic!("Expected return with OR expression"),
    }
}

#[test]
fn test_parser_relational_operators() {
    let cases = [
        ("int main(void){ return 1 < 2; }", BinaryOperator::LessThan),
        (
            "int main(void){ return 1 <= 2; }",
            BinaryOperator::LessThanOrEqual,
        ),
        (
            "int main(void){ return 1 > 2; }",
            BinaryOperator::GreaterThan,
        ),
        (
            "int main(void){ return 1 >= 2; }",
            BinaryOperator::GreaterThanOrEqual,
        ),
        ("int main(void){ return 1 == 2; }", BinaryOperator::Equal),
        ("int main(void){ return 1 != 2; }", BinaryOperator::NotEqual),
    ];

    for (src, expected_op) in cases {
        let program = parse_program(src).expect("should parse");
        let items = get_body_items(&program);

        match items[0] {
            BlockItem::S(Statement::Return(Expression::Binary(op, _, _))) => {
                assert!(
                    std::mem::discriminant(op) == std::mem::discriminant(&expected_op),
                    "Expected {:?}, got {:?}",
                    expected_op,
                    op
                );
            }
            _ => panic!("Expected return with binary expression for {}", src),
        }
    }
}

// =============================================================================
// ERROR PATHS
// =============================================================================

#[test]
fn test_parser_error_missing_semicolon() {
    let src = "int main(void){ return 0 }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail without semicolon");
}

#[test]
fn test_parser_error_missing_right_paren() {
    let src = "int main(void){ if (1 return 0; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail with missing right paren");
}

#[test]
fn test_parser_error_unexpected_token() {
    let src = "int main(void){ int = 3; return 0; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail with unexpected token");
}

#[test]
fn test_parser_error_ternary_malformed() {
    let src = "int main(void){ return 1 ? 2; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail with malformed ternary");
}

#[test]
fn test_parser_error_missing_brace() {
    let src = "int main(void){ return 0;";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail with missing closing brace");
}

#[test]
fn test_parser_error_extra_tokens() {
    let src = "int main(void){ return 0; } extra";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail with extra tokens");
}
