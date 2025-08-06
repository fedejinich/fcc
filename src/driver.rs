use std::{fs, path::Path, process::Command};

use clap::Parser;

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
    code_gen: bool,

    #[arg(short, value_name = "S")]
    s: bool,
}

impl CompilerDriver {
    pub fn create_program(&self) -> Result<(), String> {
        println!("creating program");

        let preprocessed_file = self.preprocess(&self.program_path)?;
        let assembly_file = self.compile(preprocessed_file.as_str())?;
        let exit_code = self.assemble_and_link(assembly_file)?;

        println!("exit code: {exit_code}");

        std::process::exit(exit_code);
    }

    pub fn preprocess(&self, source_file: &str) -> Result<String, String> {
        println!("preprocessing {source_file}");

        // todo(fede) this should be validated in another place
        if !source_file.ends_with(".c") {
            return Err(String::from("SOURCE_FILE should have a .c file extension"));
        }

        // todo(fede) this should be tested and is not necesary here
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
        println!("compiling {preprocessed_file}");

        // complile

        if Path::new(preprocessed_file).exists() {
            fs::remove_file(preprocessed_file).expect("couldn't remove preprocessed file");
            println!("file removed");
        } else {
            return Err(String::from("couldn't compile, preprocessed file does not exist"));
        }

        let assembly_file = preprocessed_file.replace(".i", ".asm");

        Ok(assembly_file)
    }

    fn assemble_and_link(&self, assembly_file: String) -> Result<i32, String> {
        println!("assemblying and linking {assembly_file}");

        if !Path::new(&assembly_file).exists() {
            return Err(String::from("source file does not exist"));
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
