use crate::ast::print::Printable;
use clap::Parser; // why do i need to do this? shouldn't be imported from cli.rs?
use fcc::cli::Cli;

mod assembly_emitter;
mod ast;
mod cli;
mod file_reader;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("fcc COMPILER");

    println!("--------------------------------");

    let cli = Cli::parse();

    println!("- reading source file {:?}", cli.source_path);

    let code = file_reader::read_path_buff_to_string(&cli.source_path); // todo(fedejinich) error handling

    println!("- lexing source code");

    let token_vec = lexer::lex(code.as_slice(), Vec::new());

    if cli.lex {
        println!("\n");
        token_vec.iter().for_each(|t| println!("{:?}", t));
        return;
    }

    let c_program = parser::c_parser::parse(token_vec);

    println!("- parsing");

    let assembly_program = parser::assembly_parser::parse_program(c_program);

    if cli.parse {
        println!("\n{:?}", assembly_program.print());
        return;
    }
}
