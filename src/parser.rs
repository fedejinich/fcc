use std::slice::Iter;

use crate::{
    items::{expression::*, function_declaration::*, program::*, statement::*},
    token::Token,
};

pub fn parse(token_vec: Vec<Token>) -> Program {
    let t = token_vec.iter();
    parse_program(t)
}

pub fn parse_next(expected_token: Token, mut tokens_iter: Iter<Token>) -> Iter<Token> {
    let token_opt = tokens_iter.next();

    if token_opt.is_none() || token_opt.unwrap().clone() != expected_token {
        let found = if token_opt.is_some() {
            token_opt.unwrap().to_string()
        } else {
            String::from("END")
        };

        panic!(
            "expected token: '{}', found: '{}'",
            expected_token.to_string(),
            found
        );
    }

    tokens_iter
}

// todo(fedejinich) lacks unit test
pub fn parse_program(tokens_iter: Iter<Token>) -> Program {
    let function_declaration = parse_function_declaration(tokens_iter);

    Program::new(function_declaration)
}

// todo(fedejinich) lacks unit test
pub fn parse_function_declaration(tokens_iter: Iter<Token>) -> FunctionDeclaration {
    let mut tokens_iter = parse_next(Token::IntKeyword, tokens_iter);

    let token = tokens_iter.next().unwrap().clone();

    let name = match token {
        Token::Identifier(name) => name,
        _ => panic!("expected Token::Identifier"), // todo(fedejinich) error handling
    };

    tokens_iter = parse_next(Token::OpenParenthesis, tokens_iter);

    tokens_iter = parse_next(Token::CloseParenthesis, tokens_iter);

    tokens_iter = parse_next(Token::OpenBrace, tokens_iter);

    let (statement, tokens_iter) = parse_statement(tokens_iter);

    parse_next(Token::CloseBrace, tokens_iter);

    FunctionDeclaration::new(name, statement)
}

// todo(fedejinich) lacks unit test
pub fn parse_statement(tokens_iter: Iter<Token>) -> (Statement, Iter<Token>) {
    let tokens_iter = parse_next(Token::ReturnKeyword, tokens_iter);

    let (expression, mut tokens_iter) = parse_expression(tokens_iter);

    tokens_iter = parse_next(Token::Semicolon, tokens_iter);

    (ReturnStatement::new(expression), tokens_iter)
}

// todo(fedejinich) lacks unit test
pub fn parse_expression(mut tokens_iter: Iter<Token>) -> (Expression, Iter<Token>) {
    let token = tokens_iter.next().unwrap();
    let constant = match token {
        Token::IntegerLiteral(num) => num,
        _ => panic!(
            // todo(fedejinich) error handling
            "expected token: 'IntegerLiteral', found: '{}'",
            token.to_string()
        ),
    }
    .clone();

    (ConstantExpression::new(constant), tokens_iter)
}
