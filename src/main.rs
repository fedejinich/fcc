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
    CompilerDriver::parse()
        .create_program()
        .expect("fcc failed to create program");
}
