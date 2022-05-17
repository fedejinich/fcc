use crate::ast::print::Printable;

use super::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Statement>,
}

impl FunctionDefinition {
    pub fn new(name: String, body: Vec<Statement>) -> FunctionDefinition {
        FunctionDefinition { name, body }
    }
}

impl Printable for FunctionDefinition {
    fn print(&self) -> String {
        format!(
            "Function(\n    name=\"{}\",\n    body={}",
            self.name,
            self.body
                .iter()
                .map(|s| s.print())
                .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
        )
    }
}
