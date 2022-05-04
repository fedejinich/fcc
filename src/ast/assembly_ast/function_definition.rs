use super::{assembly_ast::AssemblyAST, instruction::Instruction};

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    instructions: Vec<Instruction>,
}

impl FunctionDefinition {
    pub fn new(name: String, instructions: Vec<Instruction>) -> FunctionDefinition {
        FunctionDefinition { name, instructions }
    }
}

impl AssemblyAST for FunctionDefinition {
    fn assembly_str(&self) -> &str {
        let instructions_assembly_str = |instructions: &Vec<Instruction>| {
            instructions
                .iter()
                .map(|instruction| instruction.assembly_str())
                .reduce(|a, b| format!("{} {}", a, b).as_str())
                .unwrap()
        };

        format!(
            "    .globl {}\n{}:\n    {}",
            self.name,
            self.name,
            instructions_assembly_str(&self.instructions)
        )
        .as_str()
    }
}
