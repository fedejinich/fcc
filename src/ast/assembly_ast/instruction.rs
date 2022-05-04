use super::{assembly_ast::AssemblyAST, operand::Operand};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

impl AssemblyAST for Instruction {
    fn assembly_str(&self) -> &str {
        match self {
            Instruction::Mov { src, dst } => {
                format!("mov {} {}", src.assembly_str(), dst.assembly_str()).as_str()
            }
            Ret => "ret",
        }
    }
}
