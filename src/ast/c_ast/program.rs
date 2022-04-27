use super::function_definition::FunctionDefinition;
use crate::ast::ast_item::ASTItem;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub function_declaration: FunctionDefinition,
}

pub type CProgram = Program;

impl Program {
    pub fn new(function_declaration: FunctionDefinition) -> Program {
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

    fn pretty_print_2(&self) -> String {
        format!(
            "Program(\n  {}\n)",
            self.function_declaration.pretty_print_2()
        )
    }
}
