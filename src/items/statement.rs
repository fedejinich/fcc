use super::expression::Expression;

pub type Statement = ReturnStatement;

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    expression: Expression,
}

impl Statement {
    pub fn new(expression: Expression) -> Statement {
        Statement { expression }
    }
}
