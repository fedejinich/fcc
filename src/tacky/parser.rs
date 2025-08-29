//! This module contains the logic to lower the AST to tacky AST

use log::{debug, trace};

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
        trace!("Entering <program> conversion to Tacky IR");
        debug!("Converting C AST to Tacky IR");

        trace!("Converting <function> definition to Tacky");
        let p = TackyProgram {
            function_definition: TackyFunctionDefinition::from(program.function_definition),
        };

        trace!("<program> conversion to Tacky IR completed successfully");

        p
    }
}

impl From<FunctionDefinition> for TackyFunctionDefinition {
    fn from(function_definition: FunctionDefinition) -> Self {
        trace!("Entering <function> conversion to Tacky");
        debug!("Found <function>: {}", function_definition.name.value);
        trace!("Converting <function> body statements to Tacky instructions");
        let instructions = function_definition
            .body
            .into_iter()
            .flat_map(TackyInstruction::from)
            .collect();

        trace!("<function> conversion to Tacky completed successfully");
        TackyFunctionDefinition {
            name: TackyIdentifier::from(function_definition.name),
            instructions,
        }
    }
}

impl From<Identifier> for TackyIdentifier {
    fn from(value: Identifier) -> Self {
        trace!("Converting <identifier>: {}", value.value);
        TackyIdentifier { value: value.value }
    }
}

impl TackyInstruction {
    fn from(statement: Statement) -> Vec<TackyInstruction> {
        trace!("Entering <statement> conversion to Tacky instructions");
        let i = match statement {
            Statement::Return(expr) => {
                trace!("Found <statement>: return");
                let mut instructions = vec![];
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));
                trace!("<statement> return conversion completed");

                instructions
            }
        };

        debug!("Generated Tacky instructions: {i:?}");
        trace!("<statement> conversion to Tacky instructions completed successfully");

        i
    }

    fn from_expr(expr: Expression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        trace!("Entering <exp> conversion to Tacky");
        match expr {
            Expression::Constant(c) => {
                trace!("Found <constant>: {}", c);
                TackyValue::Constant(c)
            }
            Expression::Unary(op, inner_exp) => {
                trace!("Found <unop>: {:?}", op);
                let src = TackyInstruction::from_expr(*inner_exp, instructions);
                // TODO: provide a more descriptive name
                let dst = TackyValue::Var(TackyIdentifier::new("unary_op"));
                let unary_op = TackyUnaryOperator::from(op);

                debug!(
                    "Unary {} {} to {}",
                    src.pretty_print(),
                    unary_op.pretty_print(),
                    dst.pretty_print()
                );
                instructions.push(TackyInstruction::Unary(unary_op, src, dst.clone()));
                trace!("<unop> conversion completed");

                dst
            }
            Expression::Binary(op, left, right) => {
                trace!("Found <binop>: {:?}", op);
                let v1 = TackyInstruction::from_expr(*left, instructions);
                let v2 = TackyInstruction::from_expr(*right, instructions);
                let dst = TackyValue::Var(TackyIdentifier::new("binary_op"));
                let binary_op = TackyBinaryOperator::from(op);

                debug!(
                    "Binary {} {} to {}",
                    v1.pretty_print(),
                    v2.pretty_print(),
                    dst.pretty_print()
                );
                instructions.push(TackyInstruction::Binary(binary_op, v1, v2, dst.clone()));
                trace!("<binop> conversion completed");

                dst
            }
        }
    }
}

impl From<UnaryOperator> for TackyUnaryOperator {
    fn from(op: UnaryOperator) -> Self {
        trace!("Converting <unop>: {:?} to Tacky", op);
        let tacky_op = match op {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
        };
        trace!("<unop> conversion completed: {:?}", tacky_op);
        tacky_op
    }
}

impl From<BinaryOperator> for TackyBinaryOperator {
    fn from(op: BinaryOperator) -> Self {
        trace!("Converting <binop>: {:?} to Tacky", op);
        let tacky_op = match op {
            BinaryOperator::Add => TackyBinaryOperator::Add,
            BinaryOperator::Divide => TackyBinaryOperator::Divide,
            BinaryOperator::Multiply => TackyBinaryOperator::Multiply,
            BinaryOperator::Remainder => TackyBinaryOperator::Remainder,
            BinaryOperator::Subtract => TackyBinaryOperator::Subtract,
            BinaryOperator::BitwiseAnd => TackyBinaryOperator::And,
            BinaryOperator::BitwiseOr => TackyBinaryOperator::Or,
            BinaryOperator::BitwiseXor => TackyBinaryOperator::Xor,
            BinaryOperator::LeftShift => TackyBinaryOperator::LeftShift,
            BinaryOperator::RightShift => TackyBinaryOperator::RightShift,
        };
        trace!("<binop> conversion completed: {:?}", tacky_op);
        tacky_op
    }
}
