use crate::ast::print::Printable;

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

impl Printable for Program {
    fn print(&self) -> String {
        format!("Program(\n  {}\n)", self.function_declaration.print())
    }
}
