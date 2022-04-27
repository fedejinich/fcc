use super::operand::Operand;

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}
