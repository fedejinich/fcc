//! This module contains the logic to apply actions on the tacky AST and pipe them into a new AST

use crate::{
    codegen::x64::{
        asm::AsmProgram,
        pipes::{
            instruction_fix::fix_function_definition, reg_replace::replace_pseudoregisters_program,
        },
    },
    tacky::program::TackyProgram,
};

pub struct AsmPipe {
    program: AsmProgram,
    last_offset: Option<i32>,
}

impl From<TackyProgram> for AsmPipe {
    fn from(tp: TackyProgram) -> Self {
        Self {
            program: AsmProgram::from(tp),
            last_offset: None,
        }
    }
}

impl AsmPipe {
    /// replaces pseudoregisters with stack slots and returns the last stack memory address
    pub fn replace_pseudoregisters(mut self) -> Self {
        let (program, last_offset) = replace_pseudoregisters_program(&self.program);
        // let (program, last_offset) = &self.program.reg_replace();
        self.program = program;
        self.last_offset = Some(last_offset);

        self
    }

    /// fixes Mov instructions
    pub fn fix_instructions(mut self) -> Self {
        let last_offset = self
            .last_offset
            .expect("should call replace_pseudoregisters first");
        self.program = AsmProgram::new(fix_function_definition(
            &self.program.function_definition,
            last_offset,
        ));

        self
    }

    pub fn out(self) -> AsmProgram {
        self.program
    }
}
