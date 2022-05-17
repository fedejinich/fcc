use super::assembly_ast::AssemblyAST;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm { int: u32 },
    Register, // this will change into different registers
}

impl AssemblyAST for Operand {
    fn assembly_str(&self) -> String {
        match self {
            Operand::Imm { int } => format!("${}", int),
            Operand::Register => format!("%eax"),
        }
    }
}
