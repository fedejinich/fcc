use crate::c_ast::{ast::Program, semantic::var_res::VariableResolver};
use crate::common::folder::FolderC;

pub fn validate_semantics(program: Program) -> Result<Program, String> {
    VariableResolver::new().fold_prog(program)
}

