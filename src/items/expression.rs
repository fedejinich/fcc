use std::slice::Iter;

use crate::token::Token;

pub type Expression = ConstantExpression;

#[derive(Debug, PartialEq)]
pub struct ConstantExpression {
    constant: u32,
}

impl ConstantExpression {
    pub fn new(constant: u32) -> ConstantExpression {
        ConstantExpression { constant }
    }
}

// todo(fedejinich) lacks unit test
pub fn parse_expression(mut tokens_iter: Iter<Token>) -> (Expression, Iter<Token>) {
    let constant = match tokens_iter.next().unwrap() {
        Token::IntegerLiteral(num) => num,
        _ => panic!("expected IntegerLiteral"),
    }
    .clone();

    (ConstantExpression { constant }, tokens_iter)
}
