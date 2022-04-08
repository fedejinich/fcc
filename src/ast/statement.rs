use std::slice::Iter;

use super::{super::lexer::Token, expression::{Expression, parse_expression}};

pub type Statement = ReturnStatement;

pub struct ReturnStatement {
    expression: Expression
}

pub fn parse_statement(mut tokens_iter: Iter<Token>) -> (Statement, Iter<Token>) {
    let token = tokens_iter.next()
        .unwrap()
        .clone();
    
    if token != Token::ReturnKeyword {
        panic!("expected 'return'");
    }

    let (expression, mut tokens_iter) = parse_expression(tokens_iter);

    let token = tokens_iter.next()
        .unwrap()
        .clone();
    
    if token != Token::Semicolon {
        panic!("expected ';'");
    }

    (ReturnStatement {
        expression: expression
    }, tokens_iter)
}
