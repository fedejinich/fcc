use super::statement::{Statement, StatementNew};
use crate::ast::ast_item::ASTItem;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Statement>,
    pub body_new: Vec<StatementNew>,
}

pub type CFunctionDefinition = FunctionDefinition;

impl FunctionDefinition {
    pub fn new(name: String, body: Vec<Statement>) -> FunctionDefinition {
        FunctionDefinition {
            name,
            body,
            body_new: vec![],
        }
    }
}

impl ASTItem for FunctionDefinition {
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
