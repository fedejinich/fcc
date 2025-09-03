use std::{fs, path::Path, process::Command};

use clap::Parser;
use log::{debug, info};

use crate::ast::program::Program;
use crate::codegen::x64::pipe::AsmPipe;
use crate::lexer::lex;
use crate::tacky::program::TackyProgram;
use crate::util::replace_c_with_i;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CompilerDriver {
    program_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    tacky: bool,

    #[arg(long)]
    codegen: bool,

    #[arg(short, value_name = "S")]
    s: bool,

    #[arg(long, help = "Enable debug logging")]
    debug: bool,

    #[arg(long, help = "Enable trace logging (most verbose)")]
    trace: bool,

    #[arg(long, help = "Prints AST")]
    print_ast: bool,

    #[arg(long, help = "Prints TACKY AST")]
    print_tacky: bool,
}

impl CompilerDriver {
    pub fn init_logging(&self) {
        use std::env;

        let log_level = if self.trace {
            "trace"
        } else if self.debug {
            "debug"
        } else {
            "info"
        };

        unsafe {
            env::set_var("RUST_LOG", log_level);
        }
        env_logger::init();
    }

    pub fn build_program(&self) -> Result<(), String> {
        info!("building program");

        let preprocessed_file = self.preprocess(&self.program_path)?;
        let assembly_file = self.compile(preprocessed_file.as_str())?;
        let exit_code = self.assemble_and_link(assembly_file)?;

        info!("exit code: {exit_code}");

        std::process::exit(exit_code);
    }

    pub fn preprocess(&self, source_file: &str) -> Result<String, String> {
        info!("preprocessing {source_file}");

        // TODO: this should be validated in another place
        if !source_file.ends_with(".c") {
            return Err(String::from("SOURCE_FILE should have a .c file extension"));
        }

        // TODO: this should be tested and is not necesary here\
        let preprocessed_file = replace_c_with_i(source_file);
        if !preprocessed_file.ends_with(".i") {
            return Err(String::from(
                "PREPROCESSED_FILE should have a .i file extension",
            ));
        }

        if !Path::new(source_file).exists() {
            return Err(String::from("source file does not exist"));
        }

        Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(source_file)
            .arg("-o")
            .arg(&preprocessed_file)
            .output()
            .expect("failed to execute preprocessor");

        Ok(preprocessed_file.to_string())
    }

    fn compile(&self, preprocessed_file: &str) -> Result<String, String> {
        info!("compiling {preprocessed_file}");

        let preprocessed_file_path = Path::new(preprocessed_file);
        if !preprocessed_file_path.exists() {
            return Err(String::from(
                "couldn't compile, preprocessed file does not exist",
            ));
        }

        let code =
            fs::read_to_string(preprocessed_file_path).expect("couldn't read preprocessed file");

        // get tokens
        let tokens = lex(code.as_str())?;

        // lex only
        if self.lex {
            std::process::exit(0);
        }

        // parse tokens into ast
        let c_program = Program::try_from(tokens)?;
        if self.print_ast {
            println!("{c_program}");
        }

        // parse only
        if self.parse {
            std::process::exit(0);
        }

        let tacky_program = TackyProgram::from(c_program);

        if self.print_tacky {
            println!("{}", tacky_program.pretty_print());
        }

        if self.tacky {
            std::process::exit(0);
        }

        // generate assembly
        let assembly_file_name = preprocessed_file.replace(".i", ".asm");

        let assembly_program = AsmPipe::from(tacky_program)
            .replace_pseudoregisters()
            .fix_instructions()
            .out();

        if self.codegen {
            std::process::exit(0);
        }

        let code = assembly_program.code_emit();
        fs::write(&assembly_file_name, &code).expect("couldn't write assembly file");

        debug!("\n{code}");

        fs::remove_file(preprocessed_file).expect("couldn't remove preprocessed file");
        debug!("file removed");

        Ok(assembly_file_name)
    }

    fn assemble_and_link(&self, assembly_file: String) -> Result<i32, String> {
        info!("assemblying and linking {assembly_file}");

        if !Path::new(&assembly_file).exists() {
            return Err(String::from("asm file does not exist"));
        }

        let output_file = assembly_file.replace(".asm", "");
        let result = Command::new("gcc")
            .arg(assembly_file)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to assemble and link");

        result
            .status
            .code()
            .ok_or(String::from("failed to get status code"))
    }
}
