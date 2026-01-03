use std::fmt::{self};

use crate::codegen::x64::ast::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmInstruction, AsmOperand, AsmProgram,
    AsmUnaryOperator, Reg,
};

const FUNCTION_PROLOGUE: &[&str] = &[
    "pushq %rbp # initialize stack frame with base pointer",
    "movq %rsp, %rbp # move stack pointer to base pointer",
];

const FUNCTION_EPILOGUE: &[&str] = &["movq %rbp, %rsp", "popq %rbp", "ret"];

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
        self.emit_to(&mut em)?;
        Ok(em.finish())
    }
}

impl AsmFunctionDefinition {
    pub fn emit_to<W: fmt::Write>(&self, em: &mut Emitter<W>) -> fmt::Result {
        em.line(&format!(".globl _{}", self.name.value))?;
        em.line(&format!("_{}:", self.name.value))?;
        em.indented(|em| {
            for line in FUNCTION_PROLOGUE {
                em.line(line)?;
            }
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
        use AsmOperand::*;
        match self {
            Comment(s) => em.line(&format!("# {s}")),
            Mov(src, dst) => self.emit_binary_op(em, "movl", src, dst),
            Unary(op, x) => em.line(&format!("{op} {x}")),
            AllocateStack(n) => em.line(&format!("subq ${n}, %rsp # allocate stack space")),
            Ret => self.emit_lines(em, FUNCTION_EPILOGUE),
            Binary(op, src, dst) => self.emit_binary_op(em, &op.to_string(), src, dst),
            Idiv(op) => em.line(&format!("idivl {op}")),
            Cdq => em.line("cdq"),
            Cmp(op_1, op_2) => self.emit_binary_op(em, "cmpl", op_1, op_2),
            Jmp(label) => em.line(&format!("jmp L{}", label.value)),
            JmpCC(cond_code, id) => em.line(&format!("j{} L{}", cond_code, id.value)),
            SetCC(cond_code, op) => match op {
                Stack(_) => em.line(&format!("set{cond_code} {op}")),
                Register(_) => em.line(&format!("set{} {}", cond_code, op.byte_fmt())),
                _ => panic!("this should never happen"),
            },
            Label(label) => em.line(&format!("L{}: # LABEL", label.value)),
        }
    }

    fn emit_binary_op<W: fmt::Write>(
        &self,
        em: &mut Emitter<W>,
        opcode: &str,
        src: &AsmOperand,
        dst: &AsmOperand,
    ) -> fmt::Result {
        em.line(&format!("{opcode} {src}, {dst}"))
    }

    fn emit_lines<W: fmt::Write>(&self, em: &mut Emitter<W>, lines: &[&str]) -> fmt::Result {
        for line in lines {
            em.line(line)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for AsmUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmUnaryOperator::Neg => write!(f, "negl"),
            AsmUnaryOperator::Not => write!(f, "notl"),
        }
    }
}

impl std::fmt::Display for AsmBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

impl std::fmt::Display for AsmOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmOperand::Register(reg) => write!(f, "{}", reg.fmt_32bit()),
            AsmOperand::Stack(offset) => write!(f, "{offset}(%rbp)"),
            AsmOperand::Imm(num) => write!(f, "${num}"),
            AsmOperand::Pseudo(id) => write!(f, "{}", id.value),
        }
    }
}

impl AsmOperand {
    pub fn byte_fmt(&self) -> &str {
        match self {
            AsmOperand::Register(reg) => reg.fmt_8bit(),
            _ => panic!("byte_fmt() called on non-register operand: {self:?}"),
        }
    }
}

impl Reg {
    pub fn fmt_32bit(&self) -> &'static str {
        match self {
            Reg::AX => "%eax",
            Reg::DX => "%edx",
            Reg::CX => "%ecx",
            Reg::CL => "%cl",
            Reg::R10 => "%r10d",
            Reg::R11 => "%r11d",
        }
    }

    pub fn fmt_8bit(&self) -> &'static str {
        match self {
            Reg::AX => "%al",
            Reg::DX => "%dl",
            Reg::R10 => "%r10b",
            Reg::R11 => "%r11b",
            _ => panic!("fmt_8bit() called on non-byte register: {self:?}"),
        }
    }
}

impl std::fmt::Display for AsmCondCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmCondCode::E => write!(f, "e"),
            AsmCondCode::NE => write!(f, "ne"),
            AsmCondCode::L => write!(f, "l"),
            AsmCondCode::LE => write!(f, "le"),
            AsmCondCode::G => write!(f, "g"),
            AsmCondCode::GE => write!(f, "ge"),
        }
    }
}
