pub mod program;
pub mod function_declaration;
pub mod statement;
pub mod expression;

// then this part will be extracte into files

use super::lexer::Token;
trait AST {
    fn put(&self, token: Token) -> Self;
}