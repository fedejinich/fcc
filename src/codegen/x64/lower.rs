use log::debug;

use crate::{ast::{CExpression, CFunctionDefinition, CIdentifier, CProgram, CStatement, CUnaryOperator}, tacky::{TackyProgram, TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyValue, TackyUnaryOperator}};


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
        let mut instructions = vec![];
        let i = match statement {
            CStatement::Return(expr) => {
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));

                instructions
            }
        };

        debug!("Instructions: {i:?}");

        i
    }

    fn from_expr(expr: CExpression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        match expr {
            CExpression::Constant(c) => TackyValue::Constant(c),
            CExpression::Unary(op, inner_exp) => {
                let src = TackyInstruction::from_expr(*inner_exp, instructions);
                // todo(fede) provide a more descriptive name
                let dst = TackyValue::Var(TackyIdentifier::new("tmp"));
                let unary_op = TackyUnaryOperator::from(op);

                // todo(fede) this is a clone(hack?)
                debug!("Moving {src:?} to {dst:?}");
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

