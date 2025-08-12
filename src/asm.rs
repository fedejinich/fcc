use crate::ast::{CExpression, CFunctionDefinition, CProgram, CStatement};

#[allow(dead_code)]
pub struct AsmProgram {
    function_definition: AsmFunctionDefinition,
}

#[allow(dead_code)]
pub struct AsmFunctionDefinition {
    name: String,
    instructions: Vec<AsmInstruction>,
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

impl AsmProgram {
    pub fn code_emit(&self) -> String {
        self.function_definition.code_emit()
    }
}

impl AsmFunctionDefinition {
    pub fn code_emit(&self) -> String {
        let instructions = self
            .instructions
            .iter()
            .map(|i| i.code_emit())
            .collect::<String>();

        format!(".globl _{}\n_{}:\n{}", self.name, self.name, instructions)
    }
}

impl AsmInstruction {
    pub fn code_emit(&self) -> String {
        match self {
            AsmInstruction::Mov(src, dst) => {
                format!("mov {}, {}\n", src.code_emit(), dst.code_emit())
            }
            AsmInstruction::Ret => "ret\n".to_string(),
        }
    }
}

impl AsmOperand {
    fn code_emit(&self) -> String {
        match self {
            AsmOperand::Register => "%eax".to_string(),
            AsmOperand::Imm(num) => {
                format!("${num}")
            }
        }
    }
}

impl AsmInstruction {
    fn from(c_statement: CStatement) -> Vec<AsmInstruction> {
        match c_statement {
            CStatement::Return(exp) => vec![
                AsmInstruction::Mov(AsmOperand::from(exp), AsmOperand::Register),
                AsmInstruction::Ret,
            ],
        }
    }
}

impl From<CProgram> for AsmProgram {
    fn from(c_program: CProgram) -> Self {
        AsmProgram {
            function_definition: AsmFunctionDefinition::from(c_program.function_definition),
        }
    }
}

impl From<CFunctionDefinition> for AsmFunctionDefinition {
    fn from(c_function_definition: CFunctionDefinition) -> Self {
        AsmFunctionDefinition {
            name: c_function_definition.name.value,
            instructions: c_function_definition
                .body
                .iter()
                // todo(fede) remove clone
                .flat_map(|e| AsmInstruction::from(e.clone()))
                .collect::<Vec<AsmInstruction>>(),
        }
    }
}

impl From<CExpression> for AsmOperand {
    fn from(c_expression: CExpression) -> Self {
        match c_expression {
            CExpression::Constant(c) => AsmOperand::Imm(c),
        }
    }
}
