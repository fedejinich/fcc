use super::statement::Statement;
use crate::ast::ast_item::ASTItem;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    body: Statement,
}

impl FunctionDeclaration {
    pub fn new(name: String, body: Statement) -> FunctionDeclaration {
        FunctionDeclaration { name, body }
    }
}

impl ASTItem for FunctionDeclaration {
    fn generate_assembly(&self) -> String {
        format!(
            " .globl _{}\n_{}:\n{}",
            self.name,
            self.name,
            &self.body.generate_assembly()
        )
        .to_string()
    }

    fn pretty_print(&self) -> String {
        format!("FUN {}:\n    {}", self.name, self.body.pretty_print())
    }

    fn pretty_print_2(&self) -> String {
        format!(
            "Function(\n    name=\"{}\",\n    body={}",
            self.name,
            self.body.pretty_print_2()
        )
    }
}
