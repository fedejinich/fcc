use fcc::tacky::program::{
    TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier,
    TackyInstruction, TackyProgram, TackyUnaryOperator, TackyValue
};

#[test]
fn test_tacky_identifier_creation() {
    let id1 = TackyIdentifier::new("test");
    let id2 = TackyIdentifier::new("test");

    assert!(id1.value.starts_with("test."));
    assert!(id2.value.starts_with("test."));
    assert_ne!(id1.value, id2.value);

    let id3 = TackyIdentifier::new("main");
    assert!(id3.value.starts_with("main."));
}

#[test]
fn test_tacky_value_constant() {
    let val = TackyValue::Constant(42);

    if let TackyValue::Constant(value) = val {
        assert_eq!(value, 42);
    } else {
        panic!("Expected constant value");
    }
}

#[test]
fn test_tacky_value_variable() {
    let id = TackyIdentifier::new("var");
    let val = TackyValue::Var(id);

    if let TackyValue::Var(identifier) = val {
        assert!(identifier.value.starts_with("var."));
    } else {
        panic!("Expected variable value");
    }
}

#[test]
fn test_tacky_program_creation() {
    let name = TackyIdentifier::new("main");
    let instructions = vec![TackyInstruction::Return(TackyValue::Constant(0))];
    let function_def = TackyFunctionDefinition::new(name, instructions);
    let program = TackyProgram::new(function_def);

    assert!(program.function_definition.name.value.starts_with("main."));
    assert_eq!(program.function_definition.instructions.len(), 1);

    if let TackyInstruction::Return(TackyValue::Constant(val)) = &program.function_definition.instructions[0] {
        assert_eq!(*val, 0);
    } else {
        panic!("Expected return instruction with constant 0");
    }
}

#[test]
fn test_tacky_function_definition_creation() {
    let name = TackyIdentifier::new("func");
    let instructions = vec![
        TackyInstruction::Return(TackyValue::Constant(42))
    ];
    let func_def = TackyFunctionDefinition::new(name, instructions);

    assert!(func_def.name.value.starts_with("func."));
    assert_eq!(func_def.instructions.len(), 1);
}

#[test]
fn test_tacky_instruction_return() {
    let ret_inst = TackyInstruction::Return(TackyValue::Constant(10));

    if let TackyInstruction::Return(TackyValue::Constant(val)) = ret_inst {
        assert_eq!(val, 10);
    } else {
        panic!("Expected return instruction");
    }
}

#[test]
fn test_tacky_instruction_unary() {
    let src = TackyValue::Constant(5);
    let dst = TackyValue::Var(TackyIdentifier::new("temp"));
    let unary_inst = TackyInstruction::Unary(TackyUnaryOperator::Negate, src, dst);

    if let TackyInstruction::Unary(op, src_val, dst_val) = unary_inst {
        assert!(matches!(op, TackyUnaryOperator::Negate));

        if let TackyValue::Constant(val) = src_val {
            assert_eq!(val, 5);
        } else {
            panic!("Expected constant source");
        }

        if let TackyValue::Var(_) = dst_val {
            // Expected
        } else {
            panic!("Expected variable destination");
        }
    } else {
        panic!("Expected unary instruction");
    }
}

#[test]
fn test_tacky_instruction_binary() {
    let src1 = TackyValue::Constant(10);
    let src2 = TackyValue::Constant(20);
    let dst = TackyValue::Var(TackyIdentifier::new("result"));
    let binary_inst = TackyInstruction::Binary(TackyBinaryOperator::Add, src1, src2, dst);

    if let TackyInstruction::Binary(op, s1, s2, d) = binary_inst {
        assert!(matches!(op, TackyBinaryOperator::Add));

        if let TackyValue::Constant(val1) = s1 {
            assert_eq!(val1, 10);
        } else {
            panic!("Expected constant first source");
        }

        if let TackyValue::Constant(val2) = s2 {
            assert_eq!(val2, 20);
        } else {
            panic!("Expected constant second source");
        }

        if let TackyValue::Var(_) = d {
            // Expected
        } else {
            panic!("Expected variable destination");
        }
    } else {
        panic!("Expected binary instruction");
    }
}

#[test]
fn test_tacky_instruction_copy() {
    let src = TackyValue::Constant(42);
    let dst = TackyValue::Var(TackyIdentifier::new("temp"));
    let copy_inst = TackyInstruction::Copy(src, dst);

    if let TackyInstruction::Copy(source, destination) = copy_inst {
        if let TackyValue::Constant(val) = source {
            assert_eq!(val, 42);
        } else {
            panic!("Expected constant source");
        }

        if let TackyValue::Var(_) = destination {
            // Expected
        } else {
            panic!("Expected variable destination");
        }
    } else {
        panic!("Expected copy instruction");
    }
}

#[test]
fn test_tacky_instruction_jump() {
    let label = TackyIdentifier::new("label1");
    let jump_inst = TackyInstruction::Jump(label);

    if let TackyInstruction::Jump(lbl) = jump_inst {
        assert!(lbl.value.starts_with("label1."));
    } else {
        panic!("Expected jump instruction");
    }
}

#[test]
fn test_tacky_instruction_jump_if_zero() {
    let val = TackyValue::Var(TackyIdentifier::new("x"));
    let label = TackyIdentifier::new("zero_label");
    let jump_inst = TackyInstruction::JumpIfZero(val, label);

    if let TackyInstruction::JumpIfZero(v, lbl) = jump_inst {
        if let TackyValue::Var(_) = v {
            // Expected
        } else {
            panic!("Expected variable value");
        }
        assert!(lbl.value.starts_with("zero_label."));
    } else {
        panic!("Expected jump if zero instruction");
    }
}

#[test]
fn test_tacky_instruction_jump_if_not_zero() {
    let val = TackyValue::Constant(1);
    let label = TackyIdentifier::new("nonzero_label");
    let jump_inst = TackyInstruction::JumpIfNotZero(val, label);

    if let TackyInstruction::JumpIfNotZero(v, lbl) = jump_inst {
        if let TackyValue::Constant(value) = v {
            assert_eq!(value, 1);
        } else {
            panic!("Expected constant value");
        }
        assert!(lbl.value.starts_with("nonzero_label."));
    } else {
        panic!("Expected jump if not zero instruction");
    }
}

#[test]
fn test_tacky_instruction_label() {
    let label = TackyIdentifier::new("my_label");
    let label_inst = TackyInstruction::Label(label);

    if let TackyInstruction::Label(lbl) = label_inst {
        assert!(lbl.value.starts_with("my_label."));
    } else {
        panic!("Expected label instruction");
    }
}

#[test]
fn test_all_tacky_unary_operators() {
    let operators = vec![
        TackyUnaryOperator::Complement,
        TackyUnaryOperator::Negate,
        TackyUnaryOperator::Not,
    ];

    let src = TackyValue::Constant(1);
    let dst = TackyValue::Var(TackyIdentifier::new("temp"));

    for op in operators {
        let inst = TackyInstruction::Unary(op, src.clone(), dst.clone());
        assert!(matches!(inst, TackyInstruction::Unary(_, _, _)));
    }
}

#[test]
fn test_all_tacky_binary_operators() {
    let operators = vec![
        TackyBinaryOperator::Add,
        TackyBinaryOperator::Subtract,
        TackyBinaryOperator::Multiply,
        TackyBinaryOperator::Divide,
        TackyBinaryOperator::Remainder,
        TackyBinaryOperator::BitwiseAnd,
        TackyBinaryOperator::BitwiseOr,
        TackyBinaryOperator::BitwiseXor,
        TackyBinaryOperator::LeftShift,
        TackyBinaryOperator::RightShift,
        TackyBinaryOperator::Equal,
        TackyBinaryOperator::NotEqual,
        TackyBinaryOperator::GreaterThan,
        TackyBinaryOperator::LessThan,
        TackyBinaryOperator::GreaterThanOrEqual,
        TackyBinaryOperator::LessThanOrEqual,
    ];

    let src1 = TackyValue::Constant(1);
    let src2 = TackyValue::Constant(2);
    let dst = TackyValue::Var(TackyIdentifier::new("result"));

    for op in operators {
        let inst = TackyInstruction::Binary(op, src1.clone(), src2.clone(), dst.clone());
        assert!(matches!(inst, TackyInstruction::Binary(_, _, _, _)));
    }
}

#[test]
fn test_complex_tacky_function() {
    let name = TackyIdentifier::new("complex");
    let instructions = vec![
        TackyInstruction::Copy(TackyValue::Constant(10), TackyValue::Var(TackyIdentifier::new("a"))),
        TackyInstruction::Copy(TackyValue::Constant(20), TackyValue::Var(TackyIdentifier::new("b"))),
        TackyInstruction::Binary(
            TackyBinaryOperator::Add,
            TackyValue::Var(TackyIdentifier::new("a")),
            TackyValue::Var(TackyIdentifier::new("b")),
            TackyValue::Var(TackyIdentifier::new("result"))
        ),
        TackyInstruction::Return(TackyValue::Var(TackyIdentifier::new("result")))
    ];

    let func_def = TackyFunctionDefinition::new(name, instructions);
    assert_eq!(func_def.instructions.len(), 4);

    assert!(matches!(func_def.instructions[0], TackyInstruction::Copy(_, _)));
    assert!(matches!(func_def.instructions[1], TackyInstruction::Copy(_, _)));
    assert!(matches!(func_def.instructions[2], TackyInstruction::Binary(_, _, _, _)));
    assert!(matches!(func_def.instructions[3], TackyInstruction::Return(_)));
}