#[cfg(test)]
mod test {
    use fcc::ast::assembly_ast::{
        assembly_ast::AssemblyAST,
        function_definition::FunctionDefinition as AssemblyFunctionDefinition,
        instruction::Instruction, operand::Operand, program::Program as AssemblyProgram,
    };

    // todo(fedejinich) the real test should test that a new assembly file is generated
    #[test]
    fn valid_parse() {
        let instructions = vec![
            Instruction::Mov {
                src: Operand::Imm { int: 2 },
                dst: Operand::Register,
            },
            Instruction::Ret,
        ];
        let function_definition =
            AssemblyFunctionDefinition::new(String::from("main"), instructions);
        let assembly_program = AssemblyProgram::new(function_definition);

        let expected_assembly_str = String::from(".globl _main\n_main:\nmovl $2, %eax\nret");

        assert_eq!(expected_assembly_str, assembly_program.assembly_str())
    }
}
