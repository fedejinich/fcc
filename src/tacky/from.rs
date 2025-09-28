//! This module contains the logic to lower the AST to tacky IR

use log::{debug, trace};

use crate::{
    ast::program::{
        BinaryOperator, BlockItem, Declaration, Expression, FunctionDefinition, Identifier,
        Program, Statement, UnaryOperator,
    },
    tacky::program::{
        TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction,
        TackyProgram, TackyUnaryOperator, TackyValue,
    },
};

impl From<Program> for TackyProgram {
    fn from(program: Program) -> Self {
        trace!("Converting C AST to Tacky IR");

        let p = TackyProgram::new(TackyFunctionDefinition::from(program.function_definition));

        trace!("<program> conversion to Tacky IR completed successfully");

        p
    }
}

impl From<FunctionDefinition> for TackyFunctionDefinition {
    fn from(function_definition: FunctionDefinition) -> Self {
        trace!("Converting <function> body block items to Tacky instructions");
        debug!("<function>: {}", function_definition.name.value);
        let mut instructions: Vec<TackyInstruction> = function_definition
            .body
            .into_iter()
            .flat_map(TackyInstruction::from_bi)
            .collect();

        // add return 0 as last instruction (it's gonna be fixed in Part III)
        instructions.push(TackyInstruction::Return(TackyValue::Constant(0)));

        TackyFunctionDefinition::new(
            TackyIdentifier::from(function_definition.name),
            instructions,
        )
    }
}

impl From<Identifier> for TackyIdentifier {
    fn from(value: Identifier) -> Self {
        trace!("Converting <identifier>: {}", value.value);
        TackyIdentifier { value: value.value }
        // TackyIdentifier::new(value.value.as_str())
    }
}

impl TackyInstruction {
    fn from_bi(block_item: BlockItem) -> Vec<TackyInstruction> {
        trace!("Converting <block_item> to Tacky instructions");

        let i = match block_item {
            BlockItem::S(statement) => TackyInstruction::from_st(statement),
            BlockItem::D(declaration) => TackyInstruction::from_decl(declaration),
        };

        debug!("Generated Tacky instructions: {i:?}");

        i
    }

    fn from_st(statement: Statement) -> Vec<TackyInstruction> {
        trace!("Converting <statement> to Tacky instructions");
        let i = match statement {
            Statement::Return(expr) => {
                trace!("Converting <statement>: return");
                let mut instructions = vec![];
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));

                instructions
            }
            Statement::Expression(expr) => {
                trace!("Converting <statement>: expression");
                let mut instructions = vec![];
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Copy(
                    v,
                    TackyValue::Var(TackyIdentifier::new("result")),
                ));

                instructions
            }
            Statement::Null => {
                trace!("No need to convert <statement>: null");
                vec![]
            }
            Statement::If(cond, then, el) => {
                trace!("Converting <statement>: if");
                let mut instructions = vec![];
                let src = TackyInstruction::from_expr(*cond, &mut instructions);
                let c = TackyValue::Var(TackyIdentifier::new("c"));
                instructions.push(TackyInstruction::Copy(src, c.clone()));

                // evaluate condition
                instructions.push(TackyInstruction::JumpIfZero(
                    c,
                    TackyIdentifier::new("else"),
                ));

                // resolve then
                let statement_1 = TackyInstruction::from_st(*then);
                instructions.extend(statement_1);

                // resolve else
                if let Some(e) = el {
                    instructions.push(TackyInstruction::Jump(TackyIdentifier::new("end")));
                    instructions.push(TackyInstruction::Label(TackyIdentifier::new("else")));
                    let statement_2 = TackyInstruction::from_st(*e);
                    instructions.extend(statement_2);
                    instructions.push(TackyInstruction::Label(TackyIdentifier::new("end")));
                }

                instructions
            }
        };

        debug!("Generated Tacky instructions: {i:?}");

        i
    }

    fn from_decl(declaration: Declaration) -> Vec<TackyInstruction> {
        trace!("Converting <declaration> to Tacky instructions");
        let mut instructions = vec![];
        if let Some(initializer) = declaration.initializer {
            let v = TackyInstruction::from_expr(initializer, &mut instructions);
            instructions.push(TackyInstruction::Copy(
                v,
                TackyValue::Var(TackyIdentifier::from(declaration.name)),
            ));
        } else {
            trace!("No initializer");
        }

        // TODO: this might be wrong
        instructions
    }

    /// Lowers an Expression to TACKY.
    /// Appends the emitted instructions to `instructions` and returns
    /// a `TackyValue` that identifies where the expression's result
    /// now lives (a constant or a temporary/pseudo variable).
    // emits tacky instructions
    fn from_expr(expr: Expression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        trace!("Converting <exp> to Tacky instructions");
        match expr {
            Expression::Conditional(cond, then, el) => {
                trace!("Converting Conditional to Tacky instruction");
                let cond = TackyInstruction::from_expr(*cond, instructions);
                // TODO: handle variable names properly
                let c = TackyValue::Var(TackyIdentifier::new("c"));
                instructions.push(TackyInstruction::Copy(cond, c.clone()));
                instructions.push(TackyInstruction::JumpIfZero(
                    c,
                    TackyIdentifier::new("e2_label"),
                ));
                let result = TackyValue::Var(TackyIdentifier::new("result"));
                let e1 = TackyInstruction::from_expr(*then, instructions);
                let v1 = TackyValue::Var(TackyIdentifier::new("v1"));
                instructions.push(TackyInstruction::Copy(e1, v1.clone()));
                instructions.push(TackyInstruction::Copy(v1, result.clone()));
                instructions.push(TackyInstruction::Jump(TackyIdentifier::new("end")));
                instructions.push(TackyInstruction::Label(TackyIdentifier::new("e2_label")));
                let e2 = TackyInstruction::from_expr(*el, instructions);
                let v2 = TackyValue::Var(TackyIdentifier::new("v2"));
                instructions.push(TackyInstruction::Copy(e2, v2.clone()));
                instructions.push(TackyInstruction::Copy(v2, result.clone()));
                instructions.push(TackyInstruction::Label(TackyIdentifier::new("end")));

                result
            }
            Expression::Assignment(left, right) => {
                trace!("Converting <assignment> to Tacky instruction");
                let res = TackyInstruction::from_expr(*right, instructions);
                let left_var = match *left {
                    Expression::Var(id) => TackyValue::Var(TackyIdentifier::from(id)),
                    _ => panic!("this should never happen"),
                };
                instructions.push(TackyInstruction::Copy(res, left_var.clone()));
                left_var
            }
            Expression::Var(id) => {
                trace!("Converting <var> to Tacky instruction");
                TackyValue::Var(TackyIdentifier::from(id))
            }
            Expression::Constant(c) => {
                trace!("Converting <constant>: {c}");
                TackyValue::Constant(c)
            }
            Expression::Unary(op, inner_exp) => {
                trace!("Converting <unop>: {op:?}");
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

                dst
            }
            Expression::Binary(op, left, right) => {
                trace!("Converting <binop>: {op:?}");
                match op {
                    BinaryOperator::And => {
                        let result = TackyValue::Var(TackyIdentifier::new("and_result"));

                        let false_label = TackyIdentifier::new("false_label");
                        let end_label = TackyIdentifier::new("end");

                        trace!("Converting left expression");
                        // TODO: extract this to a function
                        let v1 = TackyInstruction::from_expr(*left, instructions);

                        let jump_if_v1 = TackyInstruction::JumpIfZero(v1, false_label.clone());

                        instructions.push(jump_if_v1.clone());

                        trace!("Converting right expression");
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

                        debug!("Generated result: {result:?}");

                        result
                    }
                    BinaryOperator::Or => {
                        let result = TackyValue::Var(TackyIdentifier::new("or_result"));
                        let false_label = TackyIdentifier::new("false_label");
                        let end_label = TackyIdentifier::new("end");

                        trace!("Converting left expression");
                        // TODO: extract this to a function
                        let v1 = TackyInstruction::from_expr(*left, instructions);

                        instructions.push(TackyInstruction::JumpIfNotZero(v1, false_label.clone()));

                        trace!("Converting right expression");
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

                        debug!("Generated result: {result:?}");

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

                        dst
                    }
                }
            }
        }
    }
}

impl From<UnaryOperator> for TackyUnaryOperator {
    fn from(op: UnaryOperator) -> Self {
        trace!("Converting <unop>: {op:?} to Tacky");
        match op {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
            UnaryOperator::Not => TackyUnaryOperator::Not,
        }
    }
}

impl From<BinaryOperator> for TackyBinaryOperator {
    fn from(op: BinaryOperator) -> Self {
        trace!("Converting <binop>: {op:?} to Tacky");
        match op {
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
        }
    }
}
