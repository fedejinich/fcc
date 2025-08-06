use clap::Parser;

use crate::{driver::CompilerDriver, title::Title};

mod asm;
mod driver;
mod lexer;
mod parser;
mod title;
mod util;

fn main() {
    println!("{}\n", Title::title2());
    let driver = CompilerDriver::parse();
    println!("{:?}", driver);
    driver
        .create_program()
        .expect("fcc failed to create program");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lex() {
        let args = vec!["fcc", "--lex", "return_2.c"];
        let driver = CompilerDriver::parse_from(args);
        println!("{:?}", driver);
        driver
            .create_program()
            .expect("fcc failed to create program");
    }
}
