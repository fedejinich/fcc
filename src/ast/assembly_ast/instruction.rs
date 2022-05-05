use super::{assembly_ast::AssemblyAST, operand::Operand};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

impl AssemblyAST for Instruction {
    fn assembly_str(&self) -> String {
        match self {
            Instruction::Mov { src, dst } => {
                format!("movl {}, {}", src.assembly_str(), dst.assembly_str())
            }
            Instruction::Ret => format!("ret"),
        }
    }
}
