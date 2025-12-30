//! This module contains the logic to lower the AST to tacky IR

use log::{debug, trace};

use crate::{
    c_ast::ast::{
        BinaryOperator, Block, BlockItem, Declaration, Expression, FunctionDefinition, Identifier,
        Program, Statement, UnaryOperator,
    },
    tacky::ast::{
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
        debug!("<function>: {}", function_definition.name.value());

        let mut instructions = TackyInstruction::from_block(function_definition.body);

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
        trace!("Converting <identifier>: {}", value.value());
        TackyIdentifier {
            value: value.value().to_string(),
        }
    }
}

impl TackyInstruction {
    fn from_block(block: Block) -> Vec<TackyInstruction> {
        trace!("Converting <block> to Tacky instructions");

        block
            .block_items()
            .clone()
            .into_iter()
            .flat_map(TackyInstruction::from_block_item) // TODO: should use a Block here
            .collect()
    }

    fn from_block_item(block_item: BlockItem) -> Vec<TackyInstruction> {
        trace!("Converting <block_item> to Tacky instructions");

        let i = match block_item {
            BlockItem::S(s) => TackyInstruction::from_st(s),
            BlockItem::D(d) => TackyInstruction::from_decl(d),
        };

        debug!("Generated Tacky instructions: {i:?}");

        i
    }

    fn from_st(statement: Statement) -> Vec<TackyInstruction> {
        trace!("Converting <statement> to Tacky instructions");
        let mut instructions = vec![];
        let i = match statement {
            Statement::Return(expr) => {
                trace!("Converting <statement>: return");
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));

                instructions
            }
            Statement::Expression(expr) => {
                trace!("Converting <statement>: expression");

                let _ = TackyInstruction::from_expr(expr, &mut instructions);

                instructions
            }
            // TODO: this can be optimized by having a special function to handle ifs with else
            // clauses (in that case we won't use the else_label)
            Statement::If(cond, then, el) => {
                trace!("Converting <statement>: if");

                let else_label_id = TackyIdentifier::new("else_label");
                let end_label_id = TackyIdentifier::new("end");

                // instructions for condition
                let cond_result = TackyInstruction::from_expr(*cond, &mut instructions);

                let c = TackyValue::Var(TackyIdentifier::new("c"));
                instructions.push(TackyInstruction::Copy(cond_result, c.clone()));
                instructions.push(TackyInstruction::JumpIfZero(c, else_label_id.clone()));

                // instructions for statement_1
                instructions.push(TackyInstruction::Comment(
                    "instruction for statement_1".to_string(),
                ));
                for ins_statement_1 in TackyInstruction::from_st(*then) {
                    instructions.push(ins_statement_1);
                }

                instructions.push(TackyInstruction::Jump(end_label_id.clone()));

                instructions.push(TackyInstruction::Label(else_label_id));
                if let Some(e) = el {
                    // instructions for statement_2
                    instructions.push(TackyInstruction::Comment(
                        "instruction for statement_2".to_string(),
                    ));
                    for ins_statement_2 in TackyInstruction::from_st(*e) {
                        instructions.push(ins_statement_2);
                    }
                }
                instructions.push(TackyInstruction::Label(end_label_id));

                instructions
            }
            Statement::Compound(block) => {
                trace!("Converting <statement>: compound");

                let inst = TackyInstruction::from_block(*block);
                for i in inst {
                    instructions.push(i);
                }
                instructions
            }
            Statement::Null => {
                trace!("No need to convert <statement>: null");

                vec![]
            }
        };

        debug!("Generated Tacky instructions: {i:?}");

        i
    }

    fn from_decl(declaration: Declaration) -> Vec<TackyInstruction> {
        trace!("Converting <declaration> to Tacky instructions");

        let mut instructions = vec![];

        let Some(initializer) = declaration.initializer else {
            trace!("No initializer");

            return instructions;
        };

        let v = TackyInstruction::from_expr(initializer, &mut instructions);
        instructions.push(TackyInstruction::Copy(
            v,
            TackyValue::Var(TackyIdentifier::from(declaration.name)),
        ));

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
                let result_id = TackyIdentifier::new("result");
                let c_result_id = TackyIdentifier::new("c_result");
                let e2_label_id = TackyIdentifier::new("e2_label");
                let v1_id = TackyIdentifier::new("v1");
                let v2_id = TackyIdentifier::new("v2");
                let end_label_id = TackyIdentifier::new("end");

                instructions.push(TackyInstruction::Comment(
                    "instruction for condition".to_string(),
                ));
                // instructions for condition
                let cond = TackyInstruction::from_expr(*cond, instructions);

                let cond_result = TackyValue::Var(c_result_id);

                instructions.push(TackyInstruction::Copy(cond, cond_result.clone()));
                instructions.push(TackyInstruction::JumpIfZero(
                    cond_result,
                    e2_label_id.clone(),
                ));

                let result = TackyValue::Var(result_id);

                // instructions to calculate e1
                let e1_result = TackyInstruction::from_expr(*then, instructions);

                let v1 = TackyValue::Var(v1_id);

                instructions.push(TackyInstruction::Copy(e1_result, v1.clone()));
                instructions.push(TackyInstruction::Copy(v1, result.clone()));
                instructions.push(TackyInstruction::Jump(end_label_id.clone()));

                instructions.push(TackyInstruction::Label(e2_label_id));

                // instructions to calculate e2
                let e2_result = TackyInstruction::from_expr(*el, instructions);

                let v2 = TackyValue::Var(v2_id);

                instructions.push(TackyInstruction::Copy(e2_result, v2.clone()));
                instructions.push(TackyInstruction::Copy(v2, result.clone()));

                instructions.push(TackyInstruction::Label(end_label_id));

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
                TackyInstruction::from_bin_op(instructions, op, left, right)
            }
        }
    }

    fn from_bin_op(
        instructions: &mut Vec<TackyInstruction>,
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> TackyValue {
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

                let copy_1 = TackyInstruction::Copy(TackyValue::Constant(1), result.clone());
                let copy_0 = TackyInstruction::Copy(TackyValue::Constant(0), result.clone());

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

                let copy_1 = TackyInstruction::Copy(TackyValue::Constant(1), result.clone());
                let copy_0 = TackyInstruction::Copy(TackyValue::Constant(0), result.clone());

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
