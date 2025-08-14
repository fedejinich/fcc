#![allow(warnings)]

use crate::ast::program::{CFunctionDefinition, CIdentifier, CProgram, CStatement};

use super::program::{CExpression, CUnaryOperator};

pub struct TackyProgram {
    function_definition: TackyFunctionDefinition,
}

pub struct TackyFunctionDefinition {
    name: TackyIdentifier,
    instructions: Vec<TackyInstruction>,
}

pub enum TackyInstruction {
    Return(TackyValue),
    Unary(TackyUnaryOperator, TackyValue, TackyValue),
}

#[derive(Clone)]
pub struct TackyIdentifier {
    value: String,
}

impl TackyIdentifier {
    fn tmp_name() -> TackyIdentifier {
        let num = 0;
        // todo(fede) this should be generated using an identity
        TackyIdentifier { value: format!("tmp.{}", num) }
    }
}

#[derive(Clone)]
pub enum TackyValue {
    Constant(i32),
    Var(TackyIdentifier),
}

pub enum TackyUnaryOperator {
    Complement,
    Negate,
}

impl From<CProgram> for TackyProgram {
    fn from(program: CProgram) -> Self {
        TackyProgram {
            function_definition: TackyFunctionDefinition::from(program.function_definition),
        }
    }
}

impl From<CFunctionDefinition> for TackyFunctionDefinition {
    fn from(function_definition: CFunctionDefinition) -> Self {
        let instructions = function_definition
            .body
            .into_iter()
            .flat_map(|s| TackyInstruction::from(s))
            .collect();

        TackyFunctionDefinition {
            name: TackyIdentifier::from(function_definition.name),
            instructions,
        }
    }
}

impl From<CIdentifier> for TackyIdentifier {
    fn from(value: CIdentifier) -> Self {
        TackyIdentifier { value: value.value }
    }
}

impl TackyInstruction {
    fn from(statement: CStatement) -> Vec<TackyInstruction> {
        let mut instructions: Vec<TackyInstruction> = vec![];
        match statement {
            CStatement::Return(expr) => {
                let _ = TackyInstruction::from_expr(expr, &mut instructions);
                instructions
            }
        }
    }

    fn from_expr(expr: CExpression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        match expr {
            CExpression::Constant(c) => TackyValue::Constant(c),
            CExpression::Unary(op, inner_exp, ) => {
                let src = TackyInstruction::from_expr(*inner_exp, instructions);
                let dst_name = TackyIdentifier::tmp_name();
                let dst = TackyValue::Var(dst_name);
                let unary_op = TackyUnaryOperator::from(op);
                // todo(fede) this is a clone(hack?)
                instructions.push(TackyInstruction::Unary(unary_op, src, dst.clone()));

                dst
            }
        }
    }
}

impl From<CUnaryOperator> for TackyUnaryOperator {
    fn from(op: CUnaryOperator) -> Self {
        match op {
            CUnaryOperator::Complement => TackyUnaryOperator::Complement,
            CUnaryOperator::Negate => TackyUnaryOperator::Negate,
        }
    }
}

