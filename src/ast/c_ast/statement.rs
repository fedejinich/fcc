use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    ReturnStatement { expression: Expression },
}
