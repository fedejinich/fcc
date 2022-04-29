use super::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Statement>,
}

impl FunctionDefinition {
    pub fn new(name: String, body: Vec<Statement>) -> FunctionDefinition {
        FunctionDefinition { name, body }
    }
}
