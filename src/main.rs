#![allow(clippy::uninlined_format_args)]

use clap::Parser;
use log::debug;

use crate::driver::CompilerDriver;

mod c_ast;
mod codegen;
mod common;
mod driver;
mod lexer;
mod tacky;

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
    let comp_driver = CompilerDriver::parse();
    comp_driver.init_logging();

    println!("{}\n", title());

    debug!("{:?}", comp_driver);

    comp_driver.build_program().expect("fcc")
}
