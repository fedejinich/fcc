use std::slice::Iter;

use crate::{parser::parse_next, token::Token};

use super::statement::{parse_statement, Statement};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    statement: Statement,
}

impl FunctionDeclaration {
    pub fn new(name: String, statement: Statement) -> FunctionDeclaration {
        FunctionDeclaration { name, statement }
    }
}

// todo(fedejinich) lacks unit test
pub fn parse_function_declaration(tokens_iter: Iter<Token>) -> FunctionDeclaration {
    let mut tokens_iter = parse_next(Token::IntKeyword, tokens_iter);

    let token = tokens_iter.next().unwrap().clone();

    let name = match token {
        Token::Identifier(name) => name,
        _ => panic!("expected Token::Identifier"),
    };

    tokens_iter = parse_next(Token::OpenParenthesis, tokens_iter);

    tokens_iter = parse_next(Token::CloseParenthesis, tokens_iter);

    tokens_iter = parse_next(Token::OpenBrace, tokens_iter);

    let (statement, tokens_iter) = parse_statement(tokens_iter);

    parse_next(Token::CloseBrace, tokens_iter);

    FunctionDeclaration { name, statement }
}
