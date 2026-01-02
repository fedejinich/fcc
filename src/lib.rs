// Library crate for the fcc compiler
//
// This file exposes the public API of the fcc compiler for use in tests
// and other external crates. It makes all the main modules available
// for unit testing and integration.

pub mod c_ast;
pub mod codegen;
pub mod common;
pub mod driver;
pub mod lexer;
pub mod tacky;
