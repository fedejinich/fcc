use fcc::c_ast::ast::{
    BinaryOperator, BlockItem, Declaration, Expression, FunctionDefinition,
    Identifier, Program, Statement, UnaryOperator
};

#[test]
fn test_identifier_creation() {
    let id = Identifier::new("main".to_string());
    assert_eq!(id.value, "main");

    let id2 = Identifier::new("variable_name".to_string());
    assert_eq!(id2.value, "variable_name");
}

#[test]
fn test_program_creation() {
    let identifier = Identifier::new("main".to_string());
    let function_def = FunctionDefinition::new(identifier, vec![]);
    let program = Program::new(function_def);

    assert_eq!(program.function_definition.name.value, "main");
    assert_eq!(program.function_definition.body.len(), 0);
}

#[test]
fn test_function_definition_creation() {
    let identifier = Identifier::new("test_func".to_string());
    let return_stmt = Statement::Return(Expression::Constant(42));
    let block_item = BlockItem::S(return_stmt);
    let function_def = FunctionDefinition::new(identifier, vec![block_item]);

    assert_eq!(function_def.name.value, "test_func");
    assert_eq!(function_def.body.len(), 1);
}

#[test]
fn test_declaration_without_initializer() {
    let name = Identifier::new("x".to_string());
    let declaration = Declaration::new(name, None);

    assert_eq!(declaration.name.value, "x");
    assert!(declaration.initializer.is_none());
}

#[test]
fn test_declaration_with_initializer() {
    let name = Identifier::new("y".to_string());
    let init_expr = Expression::Constant(10);
    let declaration = Declaration::new(name, Some(init_expr));

    assert_eq!(declaration.name.value, "y");
    assert!(declaration.initializer.is_some());

    if let Some(Expression::Constant(value)) = &declaration.initializer {
        assert_eq!(*value, 10);
    } else {
        panic!("Expected constant expression");
    }
}

#[test]
fn test_expression_constant() {
    let expr = Expression::Constant(123);

    if let Expression::Constant(value) = expr {
        assert_eq!(value, 123);
    } else {
        panic!("Expected constant expression");
    }
}

#[test]
fn test_expression_variable() {
    let id = Identifier::new("var".to_string());
    let expr = Expression::Var(id);

    if let Expression::Var(identifier) = expr {
        assert_eq!(identifier.value, "var");
    } else {
        panic!("Expected variable expression");
    }
}

#[test]
fn test_unary_expression() {
    let inner_expr = Expression::Constant(5);
    let unary_expr = Expression::Unary(UnaryOperator::Negate, Box::new(inner_expr));

    if let Expression::Unary(op, expr) = unary_expr {
        assert!(matches!(op, UnaryOperator::Negate));
        if let Expression::Constant(value) = *expr {
            assert_eq!(value, 5);
        } else {
            panic!("Expected constant expression inside unary");
        }
    } else {
        panic!("Expected unary expression");
    }
}

#[test]
fn test_binary_expression() {
    let left = Expression::Constant(10);
    let right = Expression::Constant(20);
    let binary_expr = Expression::Binary(BinaryOperator::Add, Box::new(left), Box::new(right));

    if let Expression::Binary(op, left_expr, right_expr) = binary_expr {
        assert!(matches!(op, BinaryOperator::Add));

        if let Expression::Constant(left_val) = *left_expr {
            assert_eq!(left_val, 10);
        } else {
            panic!("Expected constant in left operand");
        }

        if let Expression::Constant(right_val) = *right_expr {
            assert_eq!(right_val, 20);
        } else {
            panic!("Expected constant in right operand");
        }
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_assignment_expression() {
    let var_expr = Expression::Var(Identifier::new("x".to_string()));
    let value_expr = Expression::Constant(42);
    let assign_expr = Expression::Assignment(Box::new(var_expr), Box::new(value_expr));

    if let Expression::Assignment(left, right) = assign_expr {
        if let Expression::Var(id) = *left {
            assert_eq!(id.value, "x");
        } else {
            panic!("Expected variable on left side of assignment");
        }

        if let Expression::Constant(value) = *right {
            assert_eq!(value, 42);
        } else {
            panic!("Expected constant on right side of assignment");
        }
    } else {
        panic!("Expected assignment expression");
    }
}

#[test]
fn test_nested_unary_expressions() {
    let inner = Expression::Constant(5);
    let neg = Expression::Unary(UnaryOperator::Negate, Box::new(inner));
    let not = Expression::Unary(UnaryOperator::Not, Box::new(neg));

    if let Expression::Unary(UnaryOperator::Not, inner_expr) = not {
        if let Expression::Unary(UnaryOperator::Negate, innermost) = *inner_expr {
            if let Expression::Constant(value) = *innermost {
                assert_eq!(value, 5);
            } else {
                panic!("Expected constant at innermost level");
            }
        } else {
            panic!("Expected negation in middle");
        }
    } else {
        panic!("Expected NOT at outermost level");
    }
}

#[test]
fn test_complex_binary_expression() {
    // (a + b) * c
    let a = Expression::Var(Identifier::new("a".to_string()));
    let b = Expression::Var(Identifier::new("b".to_string()));
    let c = Expression::Var(Identifier::new("c".to_string()));

    let add_expr = Expression::Binary(BinaryOperator::Add, Box::new(a), Box::new(b));
    let mult_expr = Expression::Binary(BinaryOperator::Multiply, Box::new(add_expr), Box::new(c));

    if let Expression::Binary(BinaryOperator::Multiply, left, right) = mult_expr {
        if let Expression::Binary(BinaryOperator::Add, _, _) = *left {
            // Expected structure
        } else {
            panic!("Expected addition on left side of multiplication");
        }

        if let Expression::Var(id) = *right {
            assert_eq!(id.value, "c");
        } else {
            panic!("Expected variable 'c' on right side");
        }
    } else {
        panic!("Expected multiplication expression");
    }
}

#[test]
fn test_all_unary_operators() {
    let expr = Expression::Constant(1);

    let complement = Expression::Unary(UnaryOperator::Complement, Box::new(expr.clone()));
    let negate = Expression::Unary(UnaryOperator::Negate, Box::new(expr.clone()));
    let not = Expression::Unary(UnaryOperator::Not, Box::new(expr));

    assert!(matches!(complement, Expression::Unary(UnaryOperator::Complement, _)));
    assert!(matches!(negate, Expression::Unary(UnaryOperator::Negate, _)));
    assert!(matches!(not, Expression::Unary(UnaryOperator::Not, _)));
}

#[test]
fn test_all_binary_operators() {
    let left = Expression::Constant(1);
    let right = Expression::Constant(2);

    let operators = vec![
        BinaryOperator::Add,
        BinaryOperator::Subtract,
        BinaryOperator::Multiply,
        BinaryOperator::Divide,
        BinaryOperator::Remainder,
        BinaryOperator::BitwiseAnd,
        BinaryOperator::BitwiseOr,
        BinaryOperator::BitwiseXor,
        BinaryOperator::LeftShift,
        BinaryOperator::RightShift,
        BinaryOperator::And,
        BinaryOperator::Or,
        BinaryOperator::Equal,
        BinaryOperator::NotEqual,
        BinaryOperator::GreaterThan,
        BinaryOperator::LessThan,
        BinaryOperator::GreaterThanOrEqual,
        BinaryOperator::LessThanOrEqual,
    ];

    for op in operators {
        let expr = Expression::Binary(op, Box::new(left.clone()), Box::new(right.clone()));
        assert!(matches!(expr, Expression::Binary(_, _, _)));
    }
}

#[test]
fn test_statement_types() {
    let return_stmt = Statement::Return(Expression::Constant(0));
    let expr_stmt = Statement::Expression(Expression::Constant(42));
    let null_stmt = Statement::Null;

    assert!(matches!(return_stmt, Statement::Return(_)));
    assert!(matches!(expr_stmt, Statement::Expression(_)));
    assert!(matches!(null_stmt, Statement::Null));
}

#[test]
fn test_block_item_types() {
    let stmt = Statement::Return(Expression::Constant(0));
    let decl = Declaration::new(Identifier::new("x".to_string()), None);

    let stmt_block = BlockItem::S(stmt);
    let decl_block = BlockItem::D(decl);

    assert!(matches!(stmt_block, BlockItem::S(_)));
    assert!(matches!(decl_block, BlockItem::D(_)));
}

#[test]
fn test_statement_if_without_else() {
    let condition = Expression::Var(Identifier::new("x".to_string()));
    let then_stmt = Statement::Return(Expression::Constant(1));
    let if_stmt = Statement::If(Box::new(condition), Box::new(then_stmt), None);

    if let Statement::If(cond, then_branch, else_branch) = if_stmt {
        if let Expression::Var(id) = *cond {
            assert_eq!(id.value, "x");
        } else {
            panic!("Expected variable in condition");
        }

        assert!(matches!(*then_branch, Statement::Return(_)));
        assert!(else_branch.is_none());
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_statement_if_with_else() {
    let condition = Expression::Binary(
        BinaryOperator::GreaterThan,
        Box::new(Expression::Var(Identifier::new("a".to_string()))),
        Box::new(Expression::Constant(0)),
    );
    let then_stmt = Statement::Return(Expression::Constant(1));
    let else_stmt = Statement::Return(Expression::Constant(0));
    let if_stmt = Statement::If(
        Box::new(condition),
        Box::new(then_stmt),
        Some(Box::new(else_stmt)),
    );

    if let Statement::If(cond, then_branch, else_branch) = if_stmt {
        assert!(matches!(*cond, Expression::Binary(BinaryOperator::GreaterThan, _, _)));
        assert!(matches!(*then_branch, Statement::Return(_)));
        assert!(else_branch.is_some());

        if let Some(else_stmt) = else_branch {
            assert!(matches!(*else_stmt, Statement::Return(_)));
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_statement_nested_if() {
    // if (x) { if (y) return 1; } else return 0;
    let inner_if = Statement::If(
        Box::new(Expression::Var(Identifier::new("y".to_string()))),
        Box::new(Statement::Return(Expression::Constant(1))),
        None,
    );
    let outer_if = Statement::If(
        Box::new(Expression::Var(Identifier::new("x".to_string()))),
        Box::new(inner_if),
        Some(Box::new(Statement::Return(Expression::Constant(0)))),
    );

    if let Statement::If(_, then_branch, else_branch) = outer_if {
        assert!(matches!(*then_branch, Statement::If(_, _, _)));
        assert!(else_branch.is_some());
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_expression_conditional() {
    // x ? 1 : 0
    let condition = Expression::Var(Identifier::new("x".to_string()));
    let then_expr = Expression::Constant(1);
    let else_expr = Expression::Constant(0);
    let cond_expr = Expression::Conditional(
        Box::new(condition),
        Box::new(then_expr),
        Box::new(else_expr),
    );

    if let Expression::Conditional(cond, then_branch, else_branch) = cond_expr {
        if let Expression::Var(id) = *cond {
            assert_eq!(id.value, "x");
        } else {
            panic!("Expected variable in condition");
        }

        if let Expression::Constant(val) = *then_branch {
            assert_eq!(val, 1);
        } else {
            panic!("Expected constant in then branch");
        }

        if let Expression::Constant(val) = *else_branch {
            assert_eq!(val, 0);
        } else {
            panic!("Expected constant in else branch");
        }
    } else {
        panic!("Expected conditional expression");
    }
}

#[test]
fn test_expression_conditional_with_binary_condition() {
    // (a > b) ? a : b
    let a = Expression::Var(Identifier::new("a".to_string()));
    let b = Expression::Var(Identifier::new("b".to_string()));
    let condition = Expression::Binary(
        BinaryOperator::GreaterThan,
        Box::new(a.clone()),
        Box::new(b.clone()),
    );
    let cond_expr = Expression::Conditional(
        Box::new(condition),
        Box::new(a),
        Box::new(b),
    );

    if let Expression::Conditional(cond, then_branch, else_branch) = cond_expr {
        assert!(matches!(*cond, Expression::Binary(BinaryOperator::GreaterThan, _, _)));

        if let Expression::Var(id) = *then_branch {
            assert_eq!(id.value, "a");
        } else {
            panic!("Expected variable 'a' in then branch");
        }

        if let Expression::Var(id) = *else_branch {
            assert_eq!(id.value, "b");
        } else {
            panic!("Expected variable 'b' in else branch");
        }
    } else {
        panic!("Expected conditional expression");
    }
}

#[test]
fn test_expression_nested_conditional() {
    // x ? (y ? 1 : 2) : 3
    let inner_cond = Expression::Conditional(
        Box::new(Expression::Var(Identifier::new("y".to_string()))),
        Box::new(Expression::Constant(1)),
        Box::new(Expression::Constant(2)),
    );
    let outer_cond = Expression::Conditional(
        Box::new(Expression::Var(Identifier::new("x".to_string()))),
        Box::new(inner_cond),
        Box::new(Expression::Constant(3)),
    );

    if let Expression::Conditional(cond, then_branch, else_branch) = outer_cond {
        if let Expression::Var(id) = *cond {
            assert_eq!(id.value, "x");
        } else {
            panic!("Expected variable 'x' in outer condition");
        }

        assert!(matches!(*then_branch, Expression::Conditional(_, _, _)));

        if let Expression::Constant(val) = *else_branch {
            assert_eq!(val, 3);
        } else {
            panic!("Expected constant 3 in else branch");
        }
    } else {
        panic!("Expected conditional expression");
    }
}
