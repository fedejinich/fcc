use super::statement::Statement;
use crate::ast::ast_item::ASTItem;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    statement: Statement,
}

impl FunctionDeclaration {
    pub fn new(name: String, statement: Statement) -> FunctionDeclaration {
        FunctionDeclaration { name, statement }
    }
}

impl ASTItem for FunctionDeclaration {
    fn generate_assembly(&self) -> String {
        format!(
            " .globl _{}\n_{}:\n{}",
            self.name,
            self.name,
            &self.statement.generate_assembly()
        )
        .to_string()
    }

    fn pretty_print(&self) -> String {
        format!("FUN {}:\n    {}", self.name, self.statement.pretty_print())
    }
}
