use std::slice::Iter;

use crate::token::Token;

use super::expression::{parse_expression, Expression};

pub type Statement = ReturnStatement;

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    expression: Expression,
}

impl Statement {
    pub fn new(expression: Expression) -> Statement {
        Statement {
            expression: expression,
        }
    }
}

pub fn parse_statement(mut tokens_iter: Iter<Token>) -> (Statement, Iter<Token>) {
    let token = tokens_iter.next().unwrap().clone();

    if token != Token::ReturnKeyword {
        panic!("expected 'return'");
    }

    let (expression, mut tokens_iter) = parse_expression(tokens_iter);

    let token = tokens_iter.next().unwrap().clone();

    if token != Token::Semicolon {
        panic!("expected ';'");
    }

    (
        ReturnStatement {
            expression: expression,
        },
        tokens_iter,
    )
}
