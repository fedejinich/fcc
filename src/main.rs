use std::process::Command;

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
        let _preprocessed_file = self.preprocess(&self.program_path)?;
        Ok(())
        // let assembly_file = self.compile(preprocessed_file.as_str())?;
        // self.assemble_and_link(assembly_file)
    }

    pub fn preprocess(&self, source_file: &str) -> Result<String, String> {
        if !source_file.ends_with(".c") {
            return Err(String::from("SOURCE_FILE should have a .c file extension"));
        }

        let preprocessed_file = replace_c_with_i(source_file);
        if !preprocessed_file.ends_with(".i") {
            return Err(String::from(
                "PREPROCESSED_FILE should have a .i file extension",
            ));
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

    fn assemble_and_link(&self, assembly_file: String) -> Result<(), String> {
        let output_file = assembly_file.replace(".asm", "");
        Command::new("gcc")
            .arg(assembly_file)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to assemble and link");

        Ok(())
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
