use std::slice::Iter;

use super::{super::lexer::Token, statement::{Statement, parse_statement}};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    statement: Statement
}

impl FunctionDeclaration {
    pub fn new(name: String, statement: Statement) -> FunctionDeclaration {
        FunctionDeclaration {
            name: name,
            statement: statement
        }
    }
}

pub fn parse_function_declaration(mut tokens_iter: Iter<Token>) -> FunctionDeclaration {
    let token = tokens_iter.next()
        .unwrap()
        .clone();
    
    if token != Token::IntKeyword {
        panic!("expected 'int'");
    }

    let token = tokens_iter.next()
        .unwrap()
        .clone();

    let function_name = match token {
        Token::Identifier(name) => name,
        _ => panic!("expected Token::Identifier")
    };

    let token = tokens_iter.next()
        .unwrap()
        .clone();

    if token != Token::OpenParenthesis {
        panic!("expected '('");
    }

    let token = tokens_iter.next()
        .unwrap()
        .clone();

    if token != Token::CloseParenthesis {
        panic!("expected ')'");
    }

    let token = tokens_iter.next()
        .unwrap()
        .clone();

    if token != Token::OpenBrace {
        panic!("expected 'OpenBrace'"); // todo change 'OpenBrace' to '{'
    }
    
    let (statement, mut tokens_iter) = parse_statement(tokens_iter);

    let token = tokens_iter.next()
        .unwrap()
        .clone();

    if token != Token::CloseBrace {
        println!("{:?}",token);
        panic!("expected 'CloseBrace'"); // todo change 'CloseBrace' to '}'
    }
    
    FunctionDeclaration {
        name: function_name, 
        statement: statement
    }
}


