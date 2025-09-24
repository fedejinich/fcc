use std::collections::HashMap;

use log::trace;

use crate::ast::{
    program::{BlockItem, FunctionDefinition, Program},
    semantic::var_res::{resolve_declaration, resolve_statement},
};

pub fn validate_semantics(program: &Program) -> Result<Program, String> {
    trace!("validating program semantics");

    Ok(Program::new(validate_function_definition(
        &program.function_definition,
    )?))
}

fn validate_function_definition(
    function_definition: &FunctionDefinition,
) -> Result<FunctionDefinition, String> {
    trace!("validating function definition");

    let mut variable_map: HashMap<String, String> = HashMap::new();
    let validate_body = function_definition
        .body
        .iter()
        .map(|b| validate_block_item(b, &mut variable_map))
        .collect::<Result<Vec<BlockItem>, String>>()?;

    Ok(FunctionDefinition::new(
        function_definition.name.clone(),
        validate_body,
    ))
}

fn validate_block_item(
    block_item: &BlockItem,
    variable_map: &mut HashMap<String, String>,
) -> Result<BlockItem, String> {
    trace!("validating block item");

    let res = match block_item {
        BlockItem::S(statement) => BlockItem::S(resolve_statement(statement, variable_map)?),
        BlockItem::D(declaration) => BlockItem::D(resolve_declaration(declaration, variable_map)?),
    };

    Ok(res)
}
