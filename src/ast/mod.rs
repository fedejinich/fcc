pub mod expression;
pub mod function_declaration;
pub mod program;
pub mod statement;

// then this part will be extracte into files

use super::lexer::Token;
trait AST {
    fn put(&self, token: Token) -> Self;
}
