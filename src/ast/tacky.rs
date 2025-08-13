use crate::ast::program::CProgram;

pub struct TackyProgram {
    function_definition: TackyFunctionDefinition,
}

pub struct TackyFunctionDefinition {
    name: TackyIdentifier,
    instructions: Vec<TackyInstruction>,
}

pub enum TackyInstruction {
    Return(TackyValue),
    Unary(TackyUnaryOperator, TackyValue, TackyValue),
}

pub struct TackyIdentifier {
    value: String,
}

pub enum TackyValue {
    Constant(i32),
    Var(TackyIdentifier),
}

pub enum TackyUnaryOperator {
    Complement,
    Negate,
}

impl From<CProgram> for TackyProgram {
    fn from(value: CProgram) -> Self {
        todo!()
    }
}
