use std::{fs, path::Path, process::Command};

use clap::Parser;
use log::{debug, error, info};

use crate::c_ast::ast::Program;
use crate::c_ast::semantic::validate::validate_semantics;
use crate::codegen::x64::ast::AsmProgram;
use crate::codegen::x64::fixer::instruction_fix::InstructionFixer;
use crate::codegen::x64::fixer::reg_replace::PseudoRegisterReplacer;
use crate::common::folder::FolderAsm;
use crate::common::util::replace_c_with_i;
use crate::lexer::lex;
use crate::tacky::ast::TackyProgram;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CompilerDriver {
    program_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    validate: bool,

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
        info!("[driver] building {}", self.program_path);

        let preprocessed_file = self.preprocess(&self.program_path)?;
        let assembly_file = self.compile(preprocessed_file.as_str())?;
        let exit_code = self.assemble_and_link(assembly_file)?;

        info!("[driver] completed with exit code {exit_code}");

        std::process::exit(exit_code);
    }

    pub fn preprocess(&self, source_file: &str) -> Result<String, String> {
        info!("[driver] preprocessing");

        if !source_file.ends_with(".c") {
            error!("[driver] source file must have .c extension");

            return Err(String::from("SOURCE_FILE should have a .c file extension"));
        }

        let preprocessed_file = replace_c_with_i(source_file);
        if !preprocessed_file.ends_with(".i") {
            error!("[driver] preprocessed file must have .i extension");

            return Err(String::from(
                "PREPROCESSED_FILE should have a .i file extension",
            ));
        }

        if !Path::new(source_file).exists() {
            error!("[driver] source file does not exist: {source_file}");

            return Err(String::from("source file does not exist"));
        }

        if Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(source_file)
            .arg("-o")
            .arg(&preprocessed_file)
            .output()
            .is_err()
        {
            error!("[driver] preprocessor failed");

            return Err(String::from("failed to execute preprocessor"));
        }

        Ok(preprocessed_file)
    }

    fn compile(&self, preprocessed_file_name: &str) -> Result<String, String> {
        info!("[driver] compiling");

        let preprocessed_file_path = Path::new(preprocessed_file_name);
        if !preprocessed_file_path.exists() {
            error!("[driver] preprocessed file does not exist");

            return Err(String::from(
                "couldn't compile, preprocessed file does not exist",
            ));
        }

        let Ok(code) = fs::read_to_string(preprocessed_file_path) else {
            error!("[driver] couldn't read preprocessed file");

            return Err(String::from("couldn't read preprocessed file"));
        };

        info!("[driver] lexing");

        let tokens = lex(code.as_str())?;
        if self.lex {
            std::process::exit(0);
        }

        info!("[driver] parsing");

        let mut c_program = Program::try_from(tokens)?;
        if self.print_ast {
            println!("{c_program}");
        }
        if self.parse {
            std::process::exit(0);
        }

        info!("[driver] validating");

        c_program = validate_semantics(c_program)?;
        if self.validate {
            std::process::exit(0);
        }

        info!("[driver] generating tacky");

        let tacky_program = TackyProgram::from(c_program);
        if self.print_tacky {
            println!("{}", tacky_program.pretty_print());
        }
        if self.tacky {
            std::process::exit(0);
        }

        info!("[driver] generating assembly");

        let mut assembly_program = AsmProgram::from(tacky_program);
        assembly_program = self.do_asm_passes(assembly_program)?;
        if self.codegen {
            std::process::exit(0);
        }

        info!("[driver] emitting");

        let Ok(code) = assembly_program.to_string_asm() else {
            error!("[driver] couldn't convert to assembly string");

            return Err(String::from("couldn't convert to assembly string"));
        };

        let assembly_file_name = preprocessed_file_name.replace(".i", ".asm");
        if fs::write(&assembly_file_name, &code).is_err() {
            error!("[driver] couldn't write assembly file");

            return Err(String::from("couldn't write assembly file"));
        }

        debug!("[driver] assembly:\n{code}");

        if fs::remove_file(preprocessed_file_name).is_err() {
            error!("[driver] couldn't remove preprocessed file");

            return Err(String::from("couldn't remove preprocessed file"));
        }

        Ok(assembly_file_name)
    }

    fn do_asm_passes(&self, program: AsmProgram) -> Result<AsmProgram, String> {
        let mut replacer = PseudoRegisterReplacer::create();
        let assembly_program = replacer.fold_prog(program)?;

        let last_offset = replacer.last_offset();
        let mut fixer = InstructionFixer::create().with(last_offset);

        fixer.fold_prog(assembly_program)
    }

    fn assemble_and_link(&self, assembly_file: String) -> Result<i32, String> {
        info!("[driver] assembling and linking");

        if !Path::new(&assembly_file).exists() {
            error!("[driver] assembly file does not exist");

            return Err(String::from("asm file does not exist"));
        }

        let output_file = assembly_file.replace(".asm", "");
        let Ok(result) = Command::new("gcc")
            .arg(&assembly_file)
            .arg("-o")
            .arg(&output_file)
            .output()
        else {
            error!("[driver] failed to assemble and link");

            return Err(String::from("failed to assemble and link"));
        };

        result.status.code().ok_or_else(|| {
            error!("[driver] failed to get exit code");

            String::from("failed to get status code")
        })
    }
}
