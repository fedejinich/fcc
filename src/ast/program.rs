use std::slice::Iter;

use crate::token::Token;

use super::function_declaration::{parse_function_declaration, FunctionDeclaration};

#[derive(Debug, PartialEq)]
pub struct Program {
    function_declaration: FunctionDeclaration,
}

impl Program {
    pub fn new(function_declaration: FunctionDeclaration) -> Program {
        Program {
            function_declaration: function_declaration,
        }
    }
}

pub fn parse_program(tokens_iter: Iter<Token>) -> Program {
    let fun = parse_function_declaration(tokens_iter);

    Program {
        function_declaration: fun,
    }
}
