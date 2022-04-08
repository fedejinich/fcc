use super::function_declaration::FunctionDeclaration;

#[derive(Debug, PartialEq)]
pub struct Program {
    function_declaration: FunctionDeclaration,
}

impl Program {
    pub fn new(function_declaration: FunctionDeclaration) -> Program {
        Program {
            function_declaration,
        }
    }
}
