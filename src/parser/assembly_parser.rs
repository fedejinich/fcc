use crate::ast::{
    assembly_ast::{
        function_definition::FunctionDefinition as AssemblyFunctionDefinition,
        instruction::Instruction, operand::Operand, program::Program as AssemblyProgram,
    },
    c_ast::{
        expression::Expression, function_definition::FunctionDefinition as CFunctionDefinition,
        program::Program as CProgram, statement::Statement,
    },
};

pub struct AssemblyParser;

impl AssemblyParser {
    pub fn new() -> AssemblyParser {
        AssemblyParser {}
    }

    // todo(fedejinich) rename Program to CProgram to distinguish between CAST & AseemblyAST
    pub fn parse_program(&self, program: CProgram) -> AssemblyProgram {
        let function_definition: AssemblyFunctionDefinition =
            self.parse_function_definition(program.function_declaration);
        AssemblyProgram::new(function_definition)
    }

    fn parse_function_definition(
        &self,
        function_definition: CFunctionDefinition,
    ) -> AssemblyFunctionDefinition {
        let name: String = function_definition.name;
        let instructions: Vec<Instruction> = function_definition
            .body
            .iter()
            .map(|statement| self.parse_instructions(statement))
            .flatten()
            .collect::<Vec<Instruction>>();

        AssemblyFunctionDefinition::new(name, instructions)
    }

    fn parse_instructions(&self, statement: &Statement) -> Vec<Instruction> {
        match statement {
            Statement::ReturnStatement { expression } => self.parse_expression(expression),
        }
    }

    fn parse_expression(&self, expression: &Expression) -> Vec<Instruction> {
        match expression {
            Expression::Constant(num) => self.parse_expression_constant(num.to_owned()),
        }
    }

    fn parse_expression_constant(&self, num: u32) -> Vec<Instruction> {
        vec![
            Instruction::Mov {
                src: Operand::Imm { int: num },
                dst: Operand::Register,
            },
            Instruction::Ret,
        ]
    }
}
