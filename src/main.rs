use clap::Parser; // why do i need to do this? shouldn't be imported from cli.rs?
use fcc::cli::Cli;

mod cli;
mod code_generator;
mod file_reader;
mod items;
mod lexer;
mod parser;
mod token;

fn main() {
    let cli = Cli::parse();

    let code_opt = file_reader::read_path_buff_to_string(&cli.source_path);
    if code_opt.is_some() {
        let code = code_opt.unwrap(); // todo(fedejinich) error handling
        let token_vec = lexer::lex(code.as_slice(), Vec::new());

        let program = parser::parse(token_vec);

        let _generated_assembly = code_generator::generate(program, String::from("return_2.s"));
    } else {
        panic!("could't read .c file {:?}", cli.source_path);
    }
}
