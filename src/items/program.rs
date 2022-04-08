use super::{ast_item::ASTItem, function_declaration::FunctionDeclaration};

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

impl ASTItem for Program {
    fn generate_assembly(&self) -> String {
        self.function_declaration.generate_assembly()
    }
}
