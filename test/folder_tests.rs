use fcc::codegen::x64::asm::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmIdetifier, AsmInstruction,
    AsmOperand, AsmProgram, AsmUnaryOperator, Reg,
};
use fcc::codegen::x64::pass::instruction_fix::InstructionFixer;
use fcc::codegen::x64::pass::reg_replace::PseudoRegisterReplacer;
use fcc::common::folder::FolderAsm;

#[test]
fn test_basic_folder_trait() {
    #[derive(Default)]
    struct BasicFolder;
    impl FolderAsm for BasicFolder {
        fn create() -> Self {
            BasicFolder
        }
    }

    let folder = BasicFolder::create();
    let identifier = AsmIdetifier {
        value: "test".to_string(),
    };
    let instructions = vec![AsmInstruction::Ret];
    let function = AsmFunctionDefinition::new(identifier, instructions);
    let program = AsmProgram::new(function);

    let mut basic_folder = folder;
    let folded_program = basic_folder.fold_program(&program).unwrap();

    assert_eq!(folded_program.function_definition.name.value, "test");
    assert_eq!(folded_program.function_definition.instructions.len(), 1);
    match &folded_program.function_definition.instructions[0] {
        AsmInstruction::Ret => {}
        _ => panic!("Expected Ret instruction"),
    }
}

#[test]
fn test_folder_preserves_all_instruction_types() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();

    let instructions = vec![
        AsmInstruction::Comment("test comment".to_string()),
        AsmInstruction::Mov(AsmOperand::Imm(42), AsmOperand::Register(Reg::AX)),
        AsmInstruction::Unary(AsmUnaryOperator::Neg, AsmOperand::Register(Reg::AX)),
        AsmInstruction::Binary(
            AsmBinaryOperator::Add,
            AsmOperand::Imm(1),
            AsmOperand::Register(Reg::AX),
        ),
        AsmInstruction::Cmp(AsmOperand::Register(Reg::AX), AsmOperand::Imm(0)),
        AsmInstruction::Idiv(AsmOperand::Register(Reg::DX)),
        AsmInstruction::Cdq,
        AsmInstruction::Jmp(AsmIdetifier {
            value: "label1".to_string(),
        }),
        AsmInstruction::JmpCC(
            AsmCondCode::E,
            AsmIdetifier {
                value: "label2".to_string(),
            },
        ),
        AsmInstruction::SetCC(AsmCondCode::NE, AsmOperand::Register(Reg::CL)),
        AsmInstruction::Label(AsmIdetifier {
            value: "label1".to_string(),
        }),
        AsmInstruction::AllocateStack(16),
        AsmInstruction::Ret,
    ];

    let function = AsmFunctionDefinition::new(
        AsmIdetifier {
            value: "main".to_string(),
        },
        instructions.clone(),
    );
    let program = AsmProgram::new(function);

    let folded_program = folder.fold_program(&program).unwrap();

    assert_eq!(
        folded_program.function_definition.instructions.len(),
        instructions.len()
    );

    for (original, folded) in instructions
        .iter()
        .zip(&folded_program.function_definition.instructions)
    {
        match (original, folded) {
            (AsmInstruction::Comment(orig), AsmInstruction::Comment(folded)) => {
                assert_eq!(orig, folded)
            }
            (AsmInstruction::Mov(o1, o2), AsmInstruction::Mov(f1, f2)) => {
                assert_eq!((o1, o2), (f1, f2))
            }
            (AsmInstruction::Unary(op1, o1), AsmInstruction::Unary(op2, o2)) => {
                assert_eq!((op1, o1), (op2, o2))
            }
            (AsmInstruction::Binary(op1, o1, o2), AsmInstruction::Binary(op2, f1, f2)) => {
                assert_eq!((op1, o1, o2), (op2, f1, f2))
            }
            (AsmInstruction::Cmp(o1, o2), AsmInstruction::Cmp(f1, f2)) => {
                assert_eq!((o1, o2), (f1, f2))
            }
            (AsmInstruction::Idiv(o), AsmInstruction::Idiv(f)) => assert_eq!(o, f),
            (AsmInstruction::Cdq, AsmInstruction::Cdq) => {}
            (AsmInstruction::Jmp(o), AsmInstruction::Jmp(f)) => assert_eq!(o.value, f.value),
            (AsmInstruction::JmpCC(oc, ol), AsmInstruction::JmpCC(fc, fl)) => {
                assert_eq!((oc, &ol.value), (fc, &fl.value))
            }
            (AsmInstruction::SetCC(oc, oo), AsmInstruction::SetCC(fc, fo)) => {
                assert_eq!((oc, oo), (fc, fo))
            }
            (AsmInstruction::Label(o), AsmInstruction::Label(f)) => {
                assert_eq!(o.value, f.value)
            }
            (AsmInstruction::AllocateStack(o), AsmInstruction::AllocateStack(f)) => {
                assert_eq!(o, f)
            }
            (AsmInstruction::Ret, AsmInstruction::Ret) => {}
            _ => panic!("Instruction types don't match"),
        }
    }
}

#[test]
fn test_folder_with_all_operand_types() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();

    let operands = vec![
        AsmOperand::Imm(42),
        AsmOperand::Register(Reg::AX),
        AsmOperand::Register(Reg::DX),
        AsmOperand::Register(Reg::CX),
        AsmOperand::Register(Reg::CL),
        AsmOperand::Register(Reg::R10),
        AsmOperand::Register(Reg::R11),
        AsmOperand::Pseudo(AsmIdetifier {
            value: "var1".to_string(),
        }),
        AsmOperand::Stack(-4),
        AsmOperand::Stack(-8),
    ];

    for operand in operands {
        let folded = folder.fold_operand(&operand).unwrap();
        match (&operand, &folded) {
            (AsmOperand::Imm(o), AsmOperand::Imm(f)) => assert_eq!(o, f),
            (AsmOperand::Register(o), AsmOperand::Register(f)) => assert_eq!(o, f),
            (AsmOperand::Pseudo(o), AsmOperand::Pseudo(f)) => assert_eq!(o.value, f.value),
            (AsmOperand::Stack(o), AsmOperand::Stack(f)) => assert_eq!(o, f),
            _ => panic!("Operand types don't match"),
        }
    }
}

#[test]
fn test_folder_with_all_binary_operators() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();

    let operators = vec![
        AsmBinaryOperator::Add,
        AsmBinaryOperator::Sub,
        AsmBinaryOperator::Mult,
        AsmBinaryOperator::BitwiseAnd,
        AsmBinaryOperator::BitwiseOr,
        AsmBinaryOperator::BitwiseXor,
        AsmBinaryOperator::LeftShift,
        AsmBinaryOperator::RightShift,
    ];

    for operator in operators {
        let folded = folder.fold_binary_operator(&operator).unwrap();
        match (&operator, &folded) {
            (AsmBinaryOperator::Add, AsmBinaryOperator::Add) => {}
            (AsmBinaryOperator::Sub, AsmBinaryOperator::Sub) => {}
            (AsmBinaryOperator::Mult, AsmBinaryOperator::Mult) => {}
            (AsmBinaryOperator::BitwiseAnd, AsmBinaryOperator::BitwiseAnd) => {}
            (AsmBinaryOperator::BitwiseOr, AsmBinaryOperator::BitwiseOr) => {}
            (AsmBinaryOperator::BitwiseXor, AsmBinaryOperator::BitwiseXor) => {}
            (AsmBinaryOperator::LeftShift, AsmBinaryOperator::LeftShift) => {}
            (AsmBinaryOperator::RightShift, AsmBinaryOperator::RightShift) => {}
            _ => panic!("Binary operator types don't match"),
        }
    }
}

#[test]
fn test_folder_with_all_unary_operators() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();

    let operators = vec![AsmUnaryOperator::Neg, AsmUnaryOperator::Not];

    for operator in operators {
        let folded = folder.fold_unary_operator(&operator).unwrap();
        match (&operator, &folded) {
            (AsmUnaryOperator::Neg, AsmUnaryOperator::Neg) => {}
            (AsmUnaryOperator::Not, AsmUnaryOperator::Not) => {}
            _ => panic!("Unary operator types don't match"),
        }
    }
}

#[test]
fn test_folder_with_all_condition_codes() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();

    let codes = vec![
        AsmCondCode::E,
        AsmCondCode::NE,
        AsmCondCode::G,
        AsmCondCode::GE,
        AsmCondCode::L,
        AsmCondCode::LE,
    ];

    for code in codes {
        let folded = folder.fold_cond_code(&code).unwrap();
        match (&code, &folded) {
            (AsmCondCode::E, AsmCondCode::E) => {}
            (AsmCondCode::NE, AsmCondCode::NE) => {}
            (AsmCondCode::G, AsmCondCode::G) => {}
            (AsmCondCode::GE, AsmCondCode::GE) => {}
            (AsmCondCode::L, AsmCondCode::L) => {}
            (AsmCondCode::LE, AsmCondCode::LE) => {}
            _ => panic!("Condition code types don't match"),
        }
    }
}

#[test]
fn test_instruction_fixer_basic_functionality() {
    let instructions = vec![
        AsmInstruction::Mov(AsmOperand::Stack(-4), AsmOperand::Stack(-8)),
        AsmInstruction::Ret,
    ];

    let function = AsmFunctionDefinition::new(
        AsmIdetifier {
            value: "main".to_string(),
        },
        instructions,
    );

    let mut fixer = InstructionFixer::create().with(-12);

    let fixed_function = fixer.fold_function_definition(&function).unwrap();

    assert_eq!(fixed_function.instructions.len(), 5);

    match &fixed_function.instructions[0] {
        AsmInstruction::AllocateStack(-12) => {}
        _ => panic!("Expected AllocateStack instruction"),
    }

    match &fixed_function.instructions[1] {
        AsmInstruction::Comment(comment) => {
            assert!(comment.contains("splited mov"));
        }
        _ => panic!("Expected Comment instruction"),
    }
}

#[test]
fn test_instruction_fixer_idiv_immediate() {
    let instructions = vec![AsmInstruction::Idiv(AsmOperand::Imm(42))];

    let function = AsmFunctionDefinition::new(
        AsmIdetifier {
            value: "main".to_string(),
        },
        instructions,
    );

    let mut fixer = InstructionFixer::create().with(-4);

    let fixed_function = fixer.fold_function_definition(&function).unwrap();

    assert!(fixed_function.instructions.len() >= 3);

    match &fixed_function.instructions[1] {
        AsmInstruction::Comment(comment) => {
            assert!(comment.contains("splited idiv"));
        }
        _ => panic!("Expected Comment instruction"),
    }

    match &fixed_function.instructions[2] {
        AsmInstruction::Mov(AsmOperand::Imm(42), AsmOperand::Register(Reg::R10)) => {}
        _ => panic!("Expected Mov instruction with immediate to R10"),
    }

    match &fixed_function.instructions[3] {
        AsmInstruction::Idiv(AsmOperand::Register(Reg::R10)) => {}
        _ => panic!("Expected Idiv instruction with R10"),
    }
}

#[test]
fn test_pseudo_register_replacer_basic_functionality() {
    let instructions = vec![
        AsmInstruction::Mov(
            AsmOperand::Imm(42),
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var1".to_string(),
            }),
        ),
        AsmInstruction::Mov(
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var1".to_string(),
            }),
            AsmOperand::Register(Reg::AX),
        ),
        AsmInstruction::Ret,
    ];

    let function = AsmFunctionDefinition::new(
        AsmIdetifier {
            value: "main".to_string(),
        },
        instructions,
    );

    let mut replacer = PseudoRegisterReplacer::create();
    let replaced_function = replacer.fold_function_definition(&function).unwrap();

    assert_eq!(replaced_function.instructions.len(), 3);

    match &replaced_function.instructions[0] {
        AsmInstruction::Mov(AsmOperand::Imm(42), AsmOperand::Stack(-4)) => {}
        _ => panic!("Expected Mov with stack operand"),
    }

    match &replaced_function.instructions[1] {
        AsmInstruction::Mov(AsmOperand::Stack(-4), AsmOperand::Register(Reg::AX)) => {}
        _ => panic!("Expected Mov from stack to register"),
    }
}

#[test]
fn test_pseudo_register_replacer_multiple_variables() {
    let instructions = vec![
        AsmInstruction::Mov(
            AsmOperand::Imm(1),
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var1".to_string(),
            }),
        ),
        AsmInstruction::Mov(
            AsmOperand::Imm(2),
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var2".to_string(),
            }),
        ),
        AsmInstruction::Binary(
            AsmBinaryOperator::Add,
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var1".to_string(),
            }),
            AsmOperand::Pseudo(AsmIdetifier {
                value: "var2".to_string(),
            }),
        ),
        AsmInstruction::Ret,
    ];

    let function = AsmFunctionDefinition::new(
        AsmIdetifier {
            value: "main".to_string(),
        },
        instructions,
    );

    let mut replacer = PseudoRegisterReplacer::create();
    let replaced_function = replacer.fold_function_definition(&function).unwrap();

    assert_eq!(replaced_function.instructions.len(), 4);

    match &replaced_function.instructions[0] {
        AsmInstruction::Mov(AsmOperand::Imm(1), AsmOperand::Stack(-4)) => {}
        _ => panic!("Expected first variable at stack -4"),
    }

    match &replaced_function.instructions[1] {
        AsmInstruction::Mov(AsmOperand::Imm(2), AsmOperand::Stack(-8)) => {}
        _ => panic!("Expected second variable at stack -8"),
    }

    match &replaced_function.instructions[2] {
        AsmInstruction::Binary(
            AsmBinaryOperator::Add,
            AsmOperand::Stack(-4),
            AsmOperand::Stack(-8),
        ) => {}
        _ => panic!("Expected binary operation with stack operands"),
    }
}

#[test]
fn test_folder_preserves_identifiers() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();
    let original_id = AsmIdetifier {
        value: "test_identifier_123".to_string(),
    };

    let folded_id = folder.fold_identifier(&original_id).unwrap();
    assert_eq!(original_id.value, folded_id.value);
}

#[test]
fn test_folder_preserves_registers() {
    #[derive(Default)]
    struct IdentityFolder;
    impl FolderAsm for IdentityFolder {
        fn create() -> Self {
            IdentityFolder
        }
    }

    let mut folder = IdentityFolder::create();
    let registers = vec![Reg::AX, Reg::DX, Reg::CX, Reg::CL, Reg::R10, Reg::R11];

    for reg in registers {
        let folded_reg = folder.fold_reg(&reg).unwrap();
        match (&reg, &folded_reg) {
            (Reg::AX, Reg::AX) => {}
            (Reg::DX, Reg::DX) => {}
            (Reg::CX, Reg::CX) => {}
            (Reg::CL, Reg::CL) => {}
            (Reg::R10, Reg::R10) => {}
            (Reg::R11, Reg::R11) => {}
            _ => panic!("Register types don't match"),
        }
    }
}

