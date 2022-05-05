#[cfg(test)]
mod test {
    use fcc::ast::assembly_ast::{
        function_definition::FunctionDefinition as AssemblyFunctionDefinition,
        instruction::Instruction, operand::Operand, program::Program as AssemblyProgram,
    };
    use fcc::ast::c_ast::{
        expression::Expression, function_definition::FunctionDefinition as CFunctionDefinition,
        program::Program as CProgram, statement::Statement,
    };
    use fcc::parser::assembly_parser::AssemblyParser;

    #[test]
    fn valid_parse() {
        let expression = Expression::Constant(2);
        let statement = Statement::ReturnStatement { expression };
        let function_declaration = CFunctionDefinition::new(String::from("main"), vec![statement]);
        let c_program = CProgram::new(function_declaration);

        let instructions = vec![
            Instruction::Mov {
                src: Operand::Imm { int: 2 },
                dst: Operand::Register,
            },
            Instruction::Ret,
        ];
        let function_definition =
            AssemblyFunctionDefinition::new(String::from("main"), instructions);
        let expected_assembly_program = AssemblyProgram::new(function_definition);

        assert_eq!(
            expected_assembly_program,
            AssemblyParser::new().parse_program(c_program)
        )
    }
}
