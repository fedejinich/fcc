use std::{path::Path, process::Command};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CompilerDriver {
    program_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    code_gen: bool,
}

impl CompilerDriver {
    pub fn create_program(&self) -> Result<(), String> {
        let preprocessed_file = self.preprocess(&self.program_path)?;
        let assembly_file = self.compile(preprocessed_file.as_str())?;
        let exit_code = self.assemble_and_link(assembly_file)?;
        std::process::exit(exit_code);
    }

    pub fn preprocess(&self, source_file: &str) -> Result<String, String> {
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
        // complile
        // delete preprocessed file
        Ok(preprocessed_file.replace(".i", ".asm"))
    }

    fn assemble_and_link(&self, assembly_file: String) -> Result<i32, String> {
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

fn replace_c_with_i(file: &str) -> String {
    if let Some(stripped) = file.strip_suffix(".c") {
        format!("{stripped}.i")
    } else {
        file.to_string()
    }
}

fn main() {
    let driver = CompilerDriver::parse();
    driver.create_program().expect("failed to create program");
    println!("done");
}
