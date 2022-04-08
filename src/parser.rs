use std::slice::Iter;

use crate::{
    items::program::{parse_program, Program},
    token::Token,
};

pub fn parse(token_vec: Vec<Token>) -> Program {
    let t = token_vec.iter();
    parse_program(t)
}

pub fn parse_next(expected_token: Token, mut tokens_iter: Iter<Token>) -> Iter<Token> {
    let token = tokens_iter.next().unwrap().clone();

    if token != expected_token {
        panic!("expected '{}'", token.to_string());
    }

    tokens_iter
}
