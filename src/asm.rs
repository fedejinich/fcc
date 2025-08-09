use crate::ast::{CFunctionDefinition, CProgram};

#[allow(dead_code)]
pub struct AsmProgram {
    function_definition: AsmFunctionDefinition,
}

#[allow(dead_code)]
pub struct AsmFunctionDefinition {
    name: String,
    instructions: AsmInstruction,
}

#[allow(dead_code)]
pub enum AsmInstruction {
    Mov(AsmOperand, AsmOperand),
    Ret,
}

#[allow(dead_code)]
pub enum AsmOperand {
    Imm(i32),
    Register,
}

impl From<CProgram> for AsmProgram {
    fn from(c_program: CProgram) -> Self {
        AsmProgram {
            function_definition: AsmFunctionDefinition::from(c_program.function_definition),
        }
    }
}

impl From<CFunctionDefinition> for AsmFunctionDefinition {
    fn from(_c_function_definition: CFunctionDefinition) -> Self {
        todo!()
        // AsmFunctionDefinition {
        //     name: c_function_definition.name,
        //     instructions: c_function_definition.body.map(AsmInstruction::from),
        // }
    }
}
