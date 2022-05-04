use super::operand::Operand;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}
