use crate::token::Token;

pub mod expression;
pub mod function_declaration;
pub mod program;
pub mod statement;

// then this part will be extracte into files
trait AST {
    fn put(&self, token: Token) -> Self;
}
