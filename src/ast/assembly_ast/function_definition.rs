use super::instruction::Instruction;

pub struct FunctionDefinition {
    name: String,
    instructions: Vec<Instruction>,
}
