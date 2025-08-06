use clap::Parser;

use crate::{driver::CompilerDriver, title::Title};

mod title;
mod util;
mod driver;

fn main() {
    println!("{}", Title::title2());
    println!("-----------------------------------------------------");
    CompilerDriver::parse()
        .create_program()
        .expect("fcc failed to create program");
}
