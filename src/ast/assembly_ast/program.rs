use crate::ast::print::Printable;

use super::{assembly_ast::AssemblyAST, function_definition::FunctionDefinition};

#[derive(Debug, PartialEq)]
pub struct Program {
    function_definition: FunctionDefinition,
}

impl Program {
    pub fn new(function_definition: FunctionDefinition) -> Program {
        Program {
            function_definition,
        }
    }
}

impl Printable for Program {
    fn print(&self) -> String {
        todo!()
    }
}

impl AssemblyAST for Program {
    fn assembly_str(&self) -> String {
        self.function_definition.assembly_str()
    }
}
