use log::trace;

use crate::c_ast::{ast::Program, semantic::var_res::VariableResolver};
use crate::common::folder::FolderC;

pub fn validate_semantics(program: Program) -> Result<Program, String> {
    trace!("validating program semantics");

    VariableResolver::new().fold_program(program)
}

