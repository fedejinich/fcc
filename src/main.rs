use std::{path::PathBuf, process::Command};

use crate::{
    ast::assembly_ast::assembly_ast::AssemblyAST, cli::Cli, file_util::FileUtil, lexer::Lexer,
    parser::assembly_parser::AssemblyParser, parser::c_parser::parse,
};
use clap::Parser;

mod assembly_emitter;
mod ast;
mod cli;
mod file_util;
mod lexer;
mod parser;
mod token;

fn main() {
    let cli = Cli::parse();

    println!("fcc COMPILER");

    println!("--------------------------------");

    let path = cli.source_path;

    println!("- reading source file {:?}", path);

    let file_util = FileUtil::new();

    let code = file_util.read_path_buff_to_string(&path);

    println!("- lexing source code");

    // lex tokens from the source code

    let token_vec = Lexer::new().lex(code.as_slice(), Vec::new());

    if cli.lex.is_some() {
        println!("\n");
        token_vec.iter().for_each(|t| println!("{:?}", t));
        return;
    }

    // parse to c program

    let c_program = parse(token_vec);

    println!("- parsing");

    // parse to assembly program

    let assembly_program = AssemblyParser::new().parse_program(c_program);

    if cli.parse.is_some() {
        println!("\n{:?}", assembly_program);
        return;
    }

    println!("- emitting assembly code");

    // todo(fedejinich) this smells, should find a proper way to retrieve absolute_file_path.s
    let assembly_file_name = PathBuf::from(path.clone())
        .canonicalize()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .replace(".c", ".s");

    match file_util.write_assembly_file(&assembly_program.assembly_str(), &assembly_file_name) {
        Ok(_) => (),
        Err(err) => panic!("coudln't emit assembly file {}", err), // todo(fedejinich) this might be converted to exit(1)
    }

    let final_name = assembly_file_name.replace(".s", "");

    // assemble and link

    println!("- assembling '{}' & linking\n", assembly_file_name);

    // todo(fedejinich) should do arch switch only in M1 processors
    let mut command = Command::new("arch"); // invoke for arch '-x86_64'
    command
        .arg("-x86_64")
        .arg("gcc") // gcc ASSEMBLY_FILE -o OUTPUT_FILE
        .arg(assembly_file_name)
        .arg("-o")
        .arg(final_name);

    let exit_code = command.status().unwrap().code().unwrap();

    println!("\nexit code: {}", exit_code.to_string());

    std::process::exit(exit_code);
}
