use super::function_definition::FunctionDefinition;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub function_declaration: FunctionDefinition,
}

impl Program {
    pub fn new(function_declaration: FunctionDefinition) -> Program {
        Program {
            function_declaration,
        }
    }
}
