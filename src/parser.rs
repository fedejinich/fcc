use crate::{ast::program::{parse_program, Program}, lexer::Token};

pub fn parse(token_vec: Vec<Token>) -> Program {
    let t = token_vec.iter();
    parse_program(t)
}