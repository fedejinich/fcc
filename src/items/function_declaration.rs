use super::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    statement: Statement,
}

impl FunctionDeclaration {
    pub fn new(name: String, statement: Statement) -> FunctionDeclaration {
        FunctionDeclaration { name, statement }
    }
}
