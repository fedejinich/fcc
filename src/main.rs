use clap::Parser;
use log::debug;

use crate::driver::CompilerDriver;

mod asm;
mod ast;
mod driver;
mod lexer;
mod util;

pub fn title() -> String {
    String::from(
        "
███████╗ ██████╗ ██████╗
██╔════╝██╔════╝██╔════╝
█████╗  ██║     ██║     
██╔══╝  ██║     ██║     
██║     ╚██████╗╚██████╗
╚═╝      ╚═════╝ ╚═════╝
                        
",
    )
}

fn main() {
    let driver = CompilerDriver::parse();
    driver.init_logging();

    println!("{}\n", title());

    debug!("{:?}", driver);
    driver.build_program().expect("fcc failed to build program");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lex() {
        let args = vec!["fcc", "--lex", "return_2.c"];
        let driver = CompilerDriver::parse_from(args);
        debug!("{:?}", driver);
        driver
            .build_program()
            .expect("fcc failed to create program");
    }

    #[test]
    pub fn test_parse() {
        let args = vec!["fcc", "--parse", "return_2.c"];
        let driver = CompilerDriver::parse_from(args);
        debug!("{:?}", driver);
        driver
            .build_program()
            .expect("fcc failed to create program");
    }

    #[test]
    pub fn test_parse_invalid() {
        let args = vec!["fcc", "--parse", "extra_junk.c"];
        let driver = CompilerDriver::parse_from(args);
        debug!("{:?}", driver);
        driver
            .build_program()
            .expect("fcc failed to create program");
    }

    #[test]
    pub fn test_parse_tabs() {
        let args = vec!["fcc", "--parse", "tabs.c"];
        let driver = CompilerDriver::parse_from(args);
        debug!("{:?}", driver);
        driver
            .build_program()
            .expect("fcc failed to create program");
    }
}
