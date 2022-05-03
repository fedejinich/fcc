use crate::ast::print::Printable;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    ReturnStatement { expression: Expression },
}

impl Printable for Statement {
    fn print(&self) -> String {
        match self {
            Statement::ReturnStatement { expression } => {
                format!("Return(\n      {}\n    )", expression.print())
            }
        }
    }
}
