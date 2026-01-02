#![allow(clippy::uninlined_format_args)]

use crate::driver::CompilerDriver;
use clap::Parser;

mod c_ast;
mod codegen;
mod common;
mod driver;
mod lexer;
mod tacky;

pub fn title() -> &'static str {
    r"
███████╗ ██████╗ ██████╗
██╔════╝██╔════╝██╔════╝
█████╗  ██║     ██║     
██╔══╝  ██║     ██║     
██║     ╚██████╗╚██████╗
╚═╝      ╚═════╝ ╚═════╝
"
}

fn main() {
    let driver = CompilerDriver::parse();
    driver.init_logging();
    println!("{}\n", title());
    driver.build_program().expect("fcc")
}
