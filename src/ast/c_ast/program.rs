use super::function_declaration::FunctionDeclaration;
use crate::ast::ast_item::ASTItem;

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

    fn pretty_print(&self) -> String {
        format!("PROGRAM:\n  {}", self.function_declaration.pretty_print())
    }
}
