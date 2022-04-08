use std::slice::Iter;

use crate::{parser::parse_next, token::Token};

use super::expression::{parse_expression, Expression};

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

// todo(fedejinich) lacks unit test
pub fn parse_statement(tokens_iter: Iter<Token>) -> (Statement, Iter<Token>) {
    let tokens_iter = parse_next(Token::ReturnKeyword, tokens_iter);

    let (expression, mut tokens_iter) = parse_expression(tokens_iter);

    tokens_iter = parse_next(Token::Semicolon, tokens_iter);

    (ReturnStatement { expression }, tokens_iter)
}
