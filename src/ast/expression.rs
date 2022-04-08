use std::slice::Iter;

use super::{super::lexer::Token};

pub type Expression = ConstantExpression;

pub struct ConstantExpression {
    constant: u32
}

pub fn parse_expression(mut tokens_iter: Iter<Token>) -> (Expression, Iter<Token>) {
    let constant = match tokens_iter.next().unwrap() {
        Token::IntegerLiteral(num) => num,
        _ => panic!("expected IntegerLiteral")
    };

    (ConstantExpression {
        constant: constant.clone()
    }, tokens_iter)
}
