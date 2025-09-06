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
            Expression::Assignment(left, right) => todo!(),
            Expression::Var(id) => todo!(),
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
                match op {
                    BinaryOperator::And => {
                        let result = TackyValue::Var(TackyIdentifier::new("and_result"));

                        let false_label = TackyIdentifier::new("false_label");
                        let end_label = TackyIdentifier::new("end");

                        trace!("converting left expression");
                        // TODO: extract this to a function
                        let v1 = TackyInstruction::from_expr(*left, instructions);

                        let jump_if_v1 = TackyInstruction::JumpIfZero(v1, false_label.clone());

                        instructions.push(jump_if_v1.clone());

                        trace!("converting right expression");
                        let v2 = TackyInstruction::from_expr(*right, instructions);
                        let jump_if_v2 = TackyInstruction::JumpIfZero(v2, false_label.clone());

                        instructions.push(jump_if_v2.clone());

                        let copy_1 =
                            TackyInstruction::Copy(TackyValue::Constant(1), result.clone());
                        let copy_0 =
                            TackyInstruction::Copy(TackyValue::Constant(0), result.clone());

                        instructions.push(copy_1.clone());
                        instructions.push(TackyInstruction::Jump(end_label.clone()));
                        instructions.push(TackyInstruction::Label(false_label));
                        instructions.push(copy_0);
                        instructions.push(TackyInstruction::Label(end_label));

                        result
                    }
                    BinaryOperator::Or => {
                        let result = TackyValue::Var(TackyIdentifier::new("or_result"));
                        let false_label = TackyIdentifier::new("false_label");
                        let end_label = TackyIdentifier::new("end");

                        trace!("converting left expression");
                        // TODO: extract this to a function
                        let v1 = TackyInstruction::from_expr(*left, instructions);

                        instructions.push(TackyInstruction::JumpIfNotZero(v1, false_label.clone()));

                        trace!("converting right expression");
                        let v2 = TackyInstruction::from_expr(*right, instructions);

                        instructions.push(TackyInstruction::JumpIfNotZero(v2, false_label.clone()));

                        let copy_1 =
                            TackyInstruction::Copy(TackyValue::Constant(1), result.clone());
                        let copy_0 =
                            TackyInstruction::Copy(TackyValue::Constant(0), result.clone());

                        instructions.push(copy_0.clone());
                        instructions.push(TackyInstruction::Jump(end_label.clone()));
                        instructions.push(TackyInstruction::Label(false_label));
                        instructions.push(copy_1);
                        instructions.push(TackyInstruction::Label(end_label));

                        result
                    }
                    _ => {
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
    }
}

impl From<UnaryOperator> for TackyUnaryOperator {
    fn from(op: UnaryOperator) -> Self {
        trace!("Converting <unop>: {:?} to Tacky", op);
        let tacky_op = match op {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
            UnaryOperator::Not => TackyUnaryOperator::Not,
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
            BinaryOperator::BitwiseAnd => TackyBinaryOperator::BitwiseAnd,
            BinaryOperator::BitwiseOr => TackyBinaryOperator::BitwiseOr,
            BinaryOperator::BitwiseXor => TackyBinaryOperator::BitwiseXor,
            BinaryOperator::LeftShift => TackyBinaryOperator::LeftShift,
            BinaryOperator::RightShift => TackyBinaryOperator::RightShift,
            // logical operators
            BinaryOperator::Equal => TackyBinaryOperator::Equal,
            BinaryOperator::NotEqual => TackyBinaryOperator::NotEqual,
            BinaryOperator::GreaterThan => TackyBinaryOperator::GreaterThan,
            BinaryOperator::LessThan => TackyBinaryOperator::LessThan,
            BinaryOperator::GreaterThanOrEqual => TackyBinaryOperator::GreaterThanOrEqual,
            BinaryOperator::LessThanOrEqual => TackyBinaryOperator::LessThanOrEqual,
            BinaryOperator::And | BinaryOperator::Or => panic!("this should never happen"),
        };
        trace!("<binop> conversion completed: {:?}", tacky_op);
        tacky_op
    }
}
