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
    program.function_definition().body().iter().collect()
}

// =============================================================================
// HAPPY PATHS
// =============================================================================

#[test]
fn test_parser_simple_return() {
    let src = "int main(void){ return 0; }";
    let program = parse_program(src).expect("should parse");

    assert_eq!(program.function_definition().name().value(), "main");

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
            assert_eq!(decl.name().value(), "x");
            assert!(decl.initializer().is_none());
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
            assert_eq!(decl.name().value(), "x");
            assert!(matches!(decl.initializer(), Some(Expression::Constant(1))));
        }
        _ => panic!("Expected declaration with initializer"),
    }

    match items[1] {
        BlockItem::S(Statement::Return(Expression::Var(id))) => {
            assert_eq!(id.value(), "x");
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

// =============================================================================
// LOOP STATEMENTS
// =============================================================================

#[test]
fn test_parser_while_statement() {
    let src = "int main(void){ int x=5; while (x) x = x - 1; return x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 3);

    match items[1] {
        BlockItem::S(Statement::While(cond, body, _label)) => {
            assert!(matches!(**cond, Expression::Var(_)));
            assert!(matches!(**body, Statement::Expression(_)));
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_parser_while_with_block() {
    let src = "int main(void){ int x=5; while (x) { x = x - 1; } return x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::While(_, body, _)) => {
            assert!(matches!(**body, Statement::Compound(_)));
        }
        _ => panic!("Expected while statement with block body"),
    }
}

#[test]
fn test_parser_do_while_statement() {
    let src = "int main(void){ int x=0; do x = x + 1; while (x < 5); return x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);
    assert_eq!(items.len(), 3);

    match items[1] {
        BlockItem::S(Statement::DoWhile(body, cond, _label)) => {
            assert!(matches!(**body, Statement::Expression(_)));
            assert!(matches!(
                **cond,
                Expression::Binary(BinaryOperator::LessThan, _, _)
            ));
        }
        _ => panic!("Expected do-while statement"),
    }
}

#[test]
fn test_parser_do_while_with_block() {
    let src = "int main(void){ int x=0; do { x = x + 1; } while (x < 5); return x; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::DoWhile(body, _, _)) => {
            assert!(matches!(**body, Statement::Compound(_)));
        }
        _ => panic!("Expected do-while statement with block body"),
    }
}

#[test]
fn test_parser_for_statement_full() {
    let src = "int main(void){ for (int i = 0; i < 10; i = i + 1) return i; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::For(init, cond, post, body, _label)) => {
            // init should be a declaration
            assert!(matches!(**init, fcc::c_ast::ast::ForInit::InitDecl(_)));
            // cond should be present
            assert!(cond.is_some());
            // post should be present
            assert!(post.is_some());
            // body
            assert!(matches!(**body, Statement::Return(_)));
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_parser_for_statement_empty_init() {
    let src = "int main(void){ int i = 0; for (; i < 10; i = i + 1) i = i; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[1] {
        BlockItem::S(Statement::For(init, cond, post, _, _)) => {
            // init should be empty expression
            match init.as_ref() {
                fcc::c_ast::ast::ForInit::InitExp(exp) => {
                    assert!(exp.is_none(), "Init expression should be None");
                }
                _ => panic!("Expected InitExp for empty init"),
            }
            assert!(cond.is_some());
            assert!(post.is_some());
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_parser_for_statement_empty_cond() {
    let src = "int main(void){ for (int i = 0; ; i = i + 1) return i; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::For(_, cond, post, _, _)) => {
            assert!(cond.is_none(), "Condition should be None");
            assert!(post.is_some());
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_parser_for_statement_empty_post() {
    let src = "int main(void){ for (int i = 0; i < 10;) return i; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::For(_, cond, post, _, _)) => {
            assert!(cond.is_some());
            assert!(post.is_none(), "Post expression should be None");
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_parser_for_statement_infinite() {
    let src = "int main(void){ for (;;) return 0; return 1; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::For(init, cond, post, _, _)) => {
            match init.as_ref() {
                fcc::c_ast::ast::ForInit::InitExp(exp) => {
                    assert!(exp.is_none(), "Init should be None");
                }
                _ => panic!("Expected InitExp for empty init"),
            }
            assert!(cond.is_none(), "Condition should be None");
            assert!(post.is_none(), "Post should be None");
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_parser_break_statement() {
    let src = "int main(void){ while (1) break; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => {
            assert!(matches!(**body, Statement::Break(_)));
        }
        _ => panic!("Expected while with break"),
    }
}

#[test]
fn test_parser_continue_statement() {
    let src = "int main(void){ while (1) continue; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => {
            assert!(matches!(**body, Statement::Continue(_)));
        }
        _ => panic!("Expected while with continue"),
    }
}

#[test]
fn test_parser_nested_loops() {
    let src = "int main(void){ while (1) while (0) return 1; return 0; }";
    let program = parse_program(src).expect("should parse");

    let items = get_body_items(&program);

    match items[0] {
        BlockItem::S(Statement::While(_, body, _)) => {
            assert!(matches!(**body, Statement::While(_, _, _)));
        }
        _ => panic!("Expected nested while statements"),
    }
}

// =============================================================================
// LOOP ERROR PATHS
// =============================================================================

#[test]
fn test_parser_error_while_missing_paren() {
    let src = "int main(void){ while x return 0; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail without parentheses in while");
}

#[test]
fn test_parser_error_do_while_missing_while() {
    let src = "int main(void){ do return 0; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail without while in do-while");
}

#[test]
fn test_parser_error_do_while_missing_semicolon() {
    let src = "int main(void){ do return 0; while (1) }";
    let result = parse_program(src);
    assert!(
        result.is_err(),
        "Should fail without semicolon after do-while"
    );
}

#[test]
fn test_parser_error_for_missing_semicolons() {
    let src = "int main(void){ for (int i = 0 i < 10 i = i + 1) return 0; }";
    let result = parse_program(src);
    assert!(result.is_err(), "Should fail without semicolons in for");
}
