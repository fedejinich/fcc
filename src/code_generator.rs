use std::fs::File;
use std::io::Write;

use crate::items::{ast_item::ASTItem, program::Program};

pub fn generate(program: Program, file_name: String) -> String {
    let generated = program.generate_assembly();

    let mut file = match File::create(file_name) {
        Ok(it) => it,
        Err(_) => panic!("couldn't create assembly file"),
    };

    match write!(file, "{}", generated) {
        Ok(_) => generated,
        Err(_) => panic!("couldn't write"),
    }
}

pub fn _generate_2(program: Program) -> String {
    program.generate_assembly()
}
