#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm { int: u32 },
    Register, // this will change into different registers
}
