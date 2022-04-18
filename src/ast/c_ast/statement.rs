use super::expression::Expression;
use crate::ast::ast_item::ASTItem;

pub type Statement = ReturnStatement;

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    expression: Expression,
}

// todo(fedejinich) should be ReturnStatement
impl Statement {
    pub fn new(expression: Expression) -> Statement {
        Statement { expression }
    }
}

impl ASTItem for ReturnStatement {
    fn generate_assembly(&self) -> String {
        format!(
            "movl    ${}, %eax\nret",
            self.expression.generate_assembly()
        )
        .to_string()
    }

    fn pretty_print(&self) -> String {
        format!("RETURN {}", self.expression.pretty_print())
    }

    fn pretty_print_2(&self) -> String {
        format!("Return(\n      {}\n    )", self.expression.pretty_print_2())
    }
}
