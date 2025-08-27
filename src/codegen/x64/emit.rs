use std::fmt::{self, Binary};

use crate::codegen::x64::asm::{
    AsmBinaryOperator, AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram,
    AsmUnaryOperator, Reg,
};

pub struct Emitter<W: fmt::Write> {
    out: W,
    indent: usize,
    width: usize,
}

impl<W: fmt::Write> Emitter<W> {
    pub fn new(out: W) -> Self {
        Self {
            out,
            indent: 0,
            width: 4,
        }
    }

    #[inline]
    fn pad(&mut self) -> fmt::Result {
        for _ in 0..(self.indent * self.width) {
            self.out.write_char(' ')?;
        }
        Ok(())
    }

    pub fn line(&mut self, s: &str) -> fmt::Result {
        self.pad()?;
        self.out.write_str(s)?;
        self.out.write_char('\n')
    }

    pub fn indented<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        self.indent += 1;
        let r = f(self);
        self.indent -= 1;
        r
    }

    pub fn finish(self) -> W {
        self.out
    }
}

impl AsmProgram {
    pub fn emit_to<W: fmt::Write>(&self, em: &mut Emitter<W>) -> fmt::Result {
        self.function_definition.emit_to(em)
    }

    pub fn to_string_asm(&self) -> String {
        let mut em = Emitter::new(String::new());
        self.emit_to(&mut em).unwrap();
        em.finish()
    }
}

impl AsmFunctionDefinition {
    pub fn emit_to<W: fmt::Write>(&self, em: &mut Emitter<W>) -> fmt::Result {
        em.line(&format!(".globl _{}", self.name.value))?;
        em.line(&format!("_{}:", self.name.value))?;
        em.indented(|em| {
            em.line("pushq %rbp")?;
            em.line("movq %rsp, %rbp")?;
            for inst in &self.instructions {
                inst.emit_to(em)?;
            }
            Ok(())
        })
    }
}

impl AsmInstruction {
    pub fn emit_to<W: fmt::Write>(&self, em: &mut Emitter<W>) -> fmt::Result {
        use AsmInstruction::*;
        match self {
            Comment(s) => em.line(&format!("# {s}")),
            Mov(src, dst) => em.line(&format!("movl {}, {}", src.fmt(), dst.fmt())),
            Unary(op, x) => em.line(&format!("{} {}", op.fmt(), x.fmt())),
            AllocateStack(n) => em.line(&format!("subq ${n}, %rsp")),
            Ret => {
                em.line("movq %rbp, %rsp")?;
                em.line("popq %rbp")?;
                em.line("ret")
            }
            Binary(op, src, dst) => em.line(&format!("{} {}, {}", op.fmt(), src.fmt(), dst.fmt())),
            Idiv(op) => em.line(&format!("idivl {}", op.fmt())),
            Cdq => em.line("cdq"),
        }
    }
}

impl AsmUnaryOperator {
    pub fn fmt(&self) -> impl std::fmt::Display + '_ {
        struct Disp<'a>(&'a AsmUnaryOperator);
        impl<'a> std::fmt::Display for Disp<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    AsmUnaryOperator::Neg => write!(f, "negl"),
                    AsmUnaryOperator::Not => write!(f, "notl"),
                }
            }
        }
        Disp(self)
    }
}

impl AsmBinaryOperator {
    pub fn fmt(&self) -> impl std::fmt::Display + '_ {
        struct Disp<'a>(&'a AsmBinaryOperator);
        impl<'a> std::fmt::Display for Disp<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    // instructions operate on 32-bit values, so they get l suffixes
                    AsmBinaryOperator::Add => write!(f, "addl"),
                    AsmBinaryOperator::Sub => write!(f, "subl"),
                    AsmBinaryOperator::Mult => write!(f, "imull"),
                }
            }
        }
        Disp(self)
    }
}

impl AsmOperand {
    pub fn fmt(&self) -> impl std::fmt::Display + '_ {
        struct Disp<'a>(&'a AsmOperand);
        impl<'a> std::fmt::Display for Disp<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    AsmOperand::Register(Reg::AX) => f.write_str("%eax"),
                    AsmOperand::Register(Reg::DX) => f.write_str("%edx"),
                    AsmOperand::Register(Reg::R10) => f.write_str("%r10d"),
                    AsmOperand::Register(Reg::R11) => f.write_str("%r11d"),
                    AsmOperand::Stack(offset) => write!(f, "{}(%rbp)", offset),
                    AsmOperand::Imm(num) => write!(f, "${}", num),
                    AsmOperand::Pseudo(id) => {
                        println!("id: {}", id.value);
                        write!(f, "{}", id.value) // o fallo si no debería aparecer
                    }
                }
            }
        }
        Disp(self)
    }
}
