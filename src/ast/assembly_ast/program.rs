use super::function_definition::FunctionDefinition;

pub struct Program {
    function_definition: FunctionDefinition,
}

pub type AssemblyProgram = Program;

impl Program {
    pub fn new(function_definition: FunctionDefinition) -> Program {
        Program {
            function_definition,
        }
    }
}
