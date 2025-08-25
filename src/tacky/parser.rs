//! This module contains the logic to lower the AST to tacky AST

use log::debug;

use crate::{
    ast::program::{
        BinaryOperator, Expression, FunctionDefinition, Identifier, Program, Statement,
        UnaryOperator,
    },
    tacky::program::{
        TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction,
        TackyProgram, TackyUnaryOperator, TackyValue,
    },
};

impl From<Program> for TackyProgram {
    fn from(program: Program) -> Self {
        TackyProgram {
            function_definition: TackyFunctionDefinition::from(program.function_definition),
        }
    }
}

impl From<FunctionDefinition> for TackyFunctionDefinition {
    fn from(function_definition: FunctionDefinition) -> Self {
        let instructions = function_definition
            .body
            .into_iter()
            .flat_map(TackyInstruction::from)
            .collect();

        TackyFunctionDefinition {
            name: TackyIdentifier::from(function_definition.name),
            instructions,
        }
    }
}

impl From<Identifier> for TackyIdentifier {
    fn from(value: Identifier) -> Self {
        TackyIdentifier { value: value.value }
    }
}

impl TackyInstruction {
    fn from(statement: Statement) -> Vec<TackyInstruction> {
        let i = match statement {
            Statement::Return(expr) => {
                let mut instructions = vec![];
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));

                instructions
            }
        };

        debug!("Instructions: {i:?}");

        i
    }

    fn from_expr(expr: Expression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        match expr {
            Expression::Constant(c) => TackyValue::Constant(c),
            Expression::Unary(op, inner_exp) => {
                let src = TackyInstruction::from_expr(*inner_exp, instructions);
                // TODO: provide a more descriptive name
                let dst = TackyValue::Var(TackyIdentifier::new("unary_op"));
                let unary_op = TackyUnaryOperator::from(op);

                debug!("Moving {src:?} to {dst:?}");
                instructions.push(TackyInstruction::Unary(unary_op, src, dst.clone()));

                dst
            }
            Expression::Binary(op, left, right) => {
                let v1 = TackyInstruction::from_expr(*left, instructions);
                let v2 = TackyInstruction::from_expr(*right, instructions);
                let dst = TackyValue::Var(TackyIdentifier::new("binary_op"));
                let binary_op = TackyBinaryOperator::from(op);
                instructions.push(TackyInstruction::Binary(binary_op, v1, v2, dst.clone()));

                dst
            }
        }
    }
}

impl From<UnaryOperator> for TackyUnaryOperator {
    fn from(op: UnaryOperator) -> Self {
        match op {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
        }
    }
}

impl From<BinaryOperator> for TackyBinaryOperator {
    fn from(op: BinaryOperator) -> Self {
        match op {
            BinaryOperator::Add => TackyBinaryOperator::Add,
            BinaryOperator::Divide => TackyBinaryOperator::Divide,
            BinaryOperator::Multiply => TackyBinaryOperator::Multiply,
            BinaryOperator::Remainder => TackyBinaryOperator::Remainder,
            BinaryOperator::Subtract => TackyBinaryOperator::Subtract,
        }
    }
}
