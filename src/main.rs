use crate::{
    ast::assembly_ast::assembly_ast::AssemblyAST, cli::Cli, file_util::FileUtil, lexer::Lexer,
    parser::assembly_parser::AssemblyParser, parser::c_parser::parse,
};
use clap::Parser; // why do i need to do this? shouldn't be imported from cli.rs?

mod assembly_emitter;
mod ast;
mod cli;
mod file_util;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("fcc COMPILER");

    println!("--------------------------------");

    let cli = Cli::parse();

    let path_buf = cli.source_path;

    println!("- reading source file {:?}", path_buf);

    let file_util = FileUtil::new();

    let code = file_util.read_path_buff_to_string(&path_buf); // todo(fedejinich) error handling

    println!("- lexing source code");

    let token_vec = Lexer::new().lex(code.as_slice(), Vec::new());

    if cli.lex {
        println!("\n");
        token_vec.iter().for_each(|t| println!("{:?}", t));
        return;
    }

    let c_program = parse(token_vec); // todo(fedejinich) should be renamed to parse_c_program

    println!("- parsing");

    let assembly_program = AssemblyParser::new().parse_program(c_program); // todo(fedejinich) should be renamed to parse_assembly_program

    if cli.parse {
        println!("\n{:?}", assembly_program);
        return;
    }

    println!("- emitting assembly code");

    let assembly_file_name = path_buf
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".c", ".s");

    println!("{}", assembly_file_name);

    match file_util.write_assembly_file(&assembly_program.assembly_str(), &assembly_file_name) {
        Ok(_) => (),
        Err(err) => panic!("coudln't emit assembly file {}", err), // todo(fedejinich) this might be converted to exit(1)
    }
}
