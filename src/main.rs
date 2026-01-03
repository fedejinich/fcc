//! fcc: A toy C compiler targeting x86_64.
//!
//! This project is a rust implemetation of the book "Writing a C Compiler" by Nora Sandler.
//! It implements a subset of C, compiling to x86_64 assembly on macOS/Linux.
//!
//! # Compiler Pipeline
//!
//! ```text
//!
//!                               program.c
//!                                   │
//!                                   ▼
//!                          ┌────────────────┐
//!                          │     Lexer      │
//!                          └────────┬───────┘
//!                                   │
//!                              Token list
//!                                   │
//!                                   ▼
//!                          ┌────────────────┐
//!                          │     Parser     │
//!                          └────────┬───────┘
//!                                   │
//!                                 AST
//!                                   │
//!                                   ▼
//!                       ┌──────────────────────┐
//!                       │  Semantic analysis   │
//!                       │ ┌──────────────────┐ │
//!                       │ │  Variable        │ │
//!                       │ │  Resolution      │ │
//!                       │ ├──────────────────┤ │
//!                       │ │  Loop labeling   │ │
//!                       │ └──────────────────┘ │
//!                       └───────────┬──────────┘
//!                                   │
//!                                   │
//!                           Transformed AST
//!                                   │
//!                                   ▼
//!                       ┌──────────────────────┐
//!                       │   TACKY generation   │
//!                       └───────────┬──────────┘
//!                                   │
//!                                TACKY
//!                                   │
//!                                   ▼
//!                       ┌──────────────────────┐
//!                       │ Assembly generation  │
//!                       │ ┌──────────────────┐ │
//!                       │ │ TACKY to assembly│ │
//!                       │ ├──────────────────┤ │
//!                       │ │  Replacing       │ │
//!                       │ │  pseudoregisters │ │
//!                       │ ├──────────────────┤ │
//!                       │ │ Instruction fix  │ │
//!                       │ └──────────────────┘ │
//!                       └───────────┬──────────┘
//!                                   │
//!                               Assembly
//!                                   │
//!                                   ▼
//!                       ┌──────────────────────┐
//!                       │    Code emission     │
//!                       └───────────┬──────────┘
//!                                   │
//!                                   ▼
//!                               program.s
//! ```

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
