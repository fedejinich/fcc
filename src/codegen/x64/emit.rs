use std::fmt::{self};

use log::debug;

use crate::codegen::x64::asm::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram,
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

    pub fn to_string_asm(&self) -> Result<String, fmt::Error> {
        let mut em = Emitter::new(String::new());
        self.emit_to(&mut em);
        Ok(em.finish())
    }
}

impl AsmFunctionDefinition {
    pub fn emit_to<W: fmt::Write>(&self, em: &mut Emitter<W>) -> fmt::Result {
        em.line(&format!(".globl _{}", self.name.value))?;
        em.line(&format!("_{}:", self.name.value))?;
        em.indented(|em| {
            em.line("pushq %rbp # initialize stack frame with base pointer")?;
            em.line("movq %rsp, %rbp # move stack pointer to base pointer")?;
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
            AllocateStack(n) => em.line(&format!("subq ${n}, %rsp # allocate stack space")),
            Ret => {
                em.line("movq %rbp, %rsp")?;
                em.line("popq %rbp")?;
                em.line("ret")
            }
            Binary(op, src, dst) => em.line(&format!("{} {}, {}", op.fmt(), src.fmt(), dst.fmt())),
            Idiv(op) => em.line(&format!("idivl {}", op.fmt())),
            Cdq => em.line("cdq"),
            Cmp(op_1, op_2) => em.line(&format!("cmpl {}, {}", op_1.fmt(), op_2.fmt())),
            Jmp(label) => em.line(&format!("jmp L{}", label.value)),
            JmpCC(cond_code, id) => em.line(&format!("j{} L{}", cond_code.fmt(), id.value)),
            SetCC(cond_code, op) => match op {
                AsmOperand::Stack(_) => em.line(&format!("set{} {}", cond_code.fmt(), op.fmt())),
                AsmOperand::Register(_) => {
                    em.line(&format!("set{} {}", cond_code.fmt(), op.byte_fmt()))
                }
                _ => panic!("this should never happen"),
            },
            Label(label) => em.line(&format!("L{}: # LABEL", label.value)),
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
                    // bitwise operators
                    AsmBinaryOperator::BitwiseAnd => write!(f, "andl"),
                    AsmBinaryOperator::BitwiseOr => write!(f, "orl"),
                    AsmBinaryOperator::BitwiseXor => write!(f, "xorl"),
                    AsmBinaryOperator::LeftShift => write!(f, "shll"),
                    // right bitshift of negative value is implementation-defined;
                    // we follow GCC and use sign extension
                    // (see https://gcc.gnu.org/onlinedocs/gcc/Integers-implementation.html)
                    AsmBinaryOperator::RightShift => write!(f, "sarl"),
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
                    AsmOperand::Register(Reg::CX) => f.write_str("%ecx"),
                    AsmOperand::Register(Reg::CL) => f.write_str("%cl"),
                    AsmOperand::Register(Reg::R10) => f.write_str("%r10d"),
                    AsmOperand::Register(Reg::R11) => f.write_str("%r11d"),
                    AsmOperand::Stack(offset) => write!(f, "{}(%rbp)", offset),
                    AsmOperand::Imm(num) => write!(f, "${}", num),
                    AsmOperand::Pseudo(id) => write!(f, "{}", id.value),
                }
            }
        }
        Disp(self)
    }

    pub fn byte_fmt(&self) -> &str {
        match self {
            AsmOperand::Register(Reg::AX) => "%al",
            AsmOperand::Register(Reg::DX) => "%dl",
            AsmOperand::Register(Reg::R10) => "%r10b",
            AsmOperand::Register(Reg::R11) => "%r11b",
            _ => {
                debug!("AsmOperand: {:?}", self);
                panic!("byte_fmt() called on non-byte register")
            }
        }
    }
}

impl AsmCondCode {
    pub fn fmt(&self) -> impl std::fmt::Display + '_ {
        struct Disp<'a>(&'a AsmCondCode);
        impl<'a> std::fmt::Display for Disp<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    AsmCondCode::E => write!(f, "e"),
                    AsmCondCode::NE => write!(f, "ne"),
                    AsmCondCode::L => write!(f, "l"),
                    AsmCondCode::LE => write!(f, "le"),
                    AsmCondCode::G => write!(f, "g"),
                    AsmCondCode::GE => write!(f, "ge"),
                }
            }
        }
        Disp(self)
    }
}
