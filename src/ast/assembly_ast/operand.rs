use super::assembly_ast::AssemblyAST;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm { int: u32 },
    Register, // this will change into different registers
}

impl AssemblyAST for Operand {
    fn assembly_str(&self) -> &str {
        match self {
            Operand::Imm { int } => format!("${}", int).as_str(),
            Operand::Register => "%eax",
        }
    }
}
