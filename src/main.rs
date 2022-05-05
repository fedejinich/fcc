use crate::{
    cli::Cli, code_emitter::CodeEmitter, file_reader::read_path_buff_to_string,
    parser::assembly_parser::parse_program, parser::c_parser::parse,
};
use clap::Parser; // why do i need to do this? shouldn't be imported from cli.rs?

mod assembly_emitter;
mod ast;
mod cli;
mod code_emitter;
mod file_reader;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("fcc COMPILER");

    println!("--------------------------------");

    let cli = Cli::parse();

    let path_buf = cli.source_path;

    println!("- reading source file {:?}", path_buf);

    let code = read_path_buff_to_string(&path_buf); // todo(fedejinich) error handling

    println!("- lexing source code");

    let token_vec = lexer::lex(code.as_slice(), Vec::new());

    if cli.lex {
        println!("\n");
        token_vec.iter().for_each(|t| println!("{:?}", t));
        return;
    }

    let c_program = parse(token_vec); // todo(fedejinich) should be renamed to parse_c_program

    println!("- parsing");

    let assembly_program = parse_program(c_program); // todo(fedejinich) should be renamed to parse_assembly_program

    if cli.parse {
        println!("\n{:?}", assembly_program);
        return;
    }

    println!("- emitting assembly code");

    let file_name = path_buf
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".c", ".s");

    println!("{}", file_name);

    CodeEmitter::new().emit_assembly(&assembly_program, &file_name);
}
