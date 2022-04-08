use super::{ast_item::ASTItem, expression::Expression};

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
}
