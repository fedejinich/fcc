use super::statement::Statement;
use crate::ast::ast_item::ASTItem;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    body: Vec<Statement>,
}

impl FunctionDeclaration {
    pub fn new(name: String, body: Vec<Statement>) -> FunctionDeclaration {
        FunctionDeclaration { name, body }
    }
}

impl ASTItem for FunctionDeclaration {
    fn generate_assembly(&self) -> String {
        panic!("should be implemented")
    }

    fn pretty_print(&self) -> String {
        format!(
            "FUN {}:\n    {}",
            self.name,
            self.body
                .iter()
                .map(|s| s.pretty_print())
                .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
        )
    }

    fn pretty_print_2(&self) -> String {
        format!(
            "Function(\n    name=\"{}\",\n    body={}",
            self.name,
            self.body
                .iter()
                .map(|s| s.pretty_print_2())
                .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
        )
    }
}
