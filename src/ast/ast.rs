pub struct FunctionDeclaration {
    name: &str,
    statement: Statement
}

pub struct ReturnStatement {
    expression: ConstantExpression
}

pub struct ConstantExpression {
    constant: u32
}

trait AST {
    fn put(&self, token: Token) -> Self;
}