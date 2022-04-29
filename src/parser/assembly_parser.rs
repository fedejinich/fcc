use crate::ast::{
    assembly_ast::{
        function_definition::FunctionDefinition as AssemblyFunctionDefinition,
        instruction::Instruction, operand::Operand, program::AssemblyProgram,
    },
    c_ast::{
        expression::Expression, function_definition::FunctionDefinition as CFunctionDefinition,
        program::Program as CProgram, statement::Statement,
    },
};

// todo(fedejinich) rename Program to CProgram to distinguish between CAST & AseemblyAST
pub fn parse_program(program: CProgram) -> AssemblyProgram {
    let function_definition: AssemblyFunctionDefinition =
        parse_function_definition(program.function_declaration);
    AssemblyProgram::new(function_definition)
}

fn parse_function_definition(
    function_definition: CFunctionDefinition,
) -> AssemblyFunctionDefinition {
    let name: String = function_definition.name;
    let instructions: Vec<Instruction> = function_definition
        .body
        .iter()
        .map(|statement| parse_instructions(statement))
        .flatten()
        .collect::<Vec<Instruction>>();

    AssemblyFunctionDefinition::new(name, instructions)
}

fn parse_instructions(statement: &Statement) -> Vec<Instruction> {
    match statement {
        Statement::ReturnStatement { expression } => parse_expression(expression),
    }
}

fn parse_expression(expression: &Expression) -> Vec<Instruction> {
    match expression {
        Expression::Constant(num) => parse_expression_constant(num.to_owned()),
    }
}

fn parse_expression_constant(num: u32) -> Vec<Instruction> {
    vec![
        Instruction::Mov {
            src: Operand::Imm { int: num },
            dst: Operand::Register,
        },
        Instruction::Ret,
    ]
}
