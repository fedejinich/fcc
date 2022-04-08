use super::{ast_item::ASTItem, statement::Statement};

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
}
