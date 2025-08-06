use clap::Parser;
use log::debug;

use crate::{driver::CompilerDriver, title::Title};

mod asm;
mod driver;
mod lexer;
mod parser;
mod title;
mod util;

fn main() {
    let driver = CompilerDriver::parse();
    driver.init_logging();

    println!("{}\n", Title::title2());

    debug!("{:?}", driver);
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
        debug!("{:?}", driver);
        driver
            .create_program()
            .expect("fcc failed to create program");
    }
}
