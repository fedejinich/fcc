#![allow(clippy::uninlined_format_args)]

use clap::Parser;
use log::debug;

use crate::driver::CompilerDriver;

mod c_ast;
mod codegen;
mod driver;
mod lexer;
mod tacky;
mod common;

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

    let Ok(_) = driver.build_program() else {
        panic!("fcc failed to build program");
    };
}
