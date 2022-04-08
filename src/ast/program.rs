use std::slice::Iter;

use super::super::lexer::Token;
use super::function_declaration::{FunctionDeclaration, parse_function_declaration};


pub struct Program {
    function_declaration: FunctionDeclaration
}

pub fn parse_program(tokens_iter: Iter<Token>) -> Program {
    let fun = parse_function_declaration(tokens_iter);

    Program {
        function_declaration: fun
    }
}
