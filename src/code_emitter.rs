use std::{fs::File, io::Write};

use crate::ast::assembly_ast::assembly_ast::AssemblyAST;

pub struct CodeEmitter;

impl CodeEmitter {
    pub fn new() -> CodeEmitter {
        CodeEmitter {}
    }
    pub fn emit_assembly(&self, assembly_ast: &impl AssemblyAST, file_name: &str) {
        let assembly_str = assembly_ast.assembly_str();

        let mut file = match File::create(file_name) {
            Ok(it) => it,
            Err(_) => panic!("couldn't emit assembly file"),
        };

        match write!(file, "{}", assembly_str) {
            Ok(_) => assembly_str,
            Err(_) => panic!("couldn't write"),
        };
    }
}
