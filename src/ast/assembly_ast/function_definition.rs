use super::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    instructions: Vec<Instruction>,
}

pub type AssemblyFunctionDefinition = FunctionDefinition;

impl FunctionDefinition {
    pub fn new(name: String, instructions: Vec<Instruction>) -> FunctionDefinition {
        FunctionDefinition { name, instructions }
    }
}
