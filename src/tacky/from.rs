//! This module contains the logic to lower the AST to tacky IR

use log::{debug, info, trace};

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
        trace!("[tacky] <program>");
        TackyProgram::new(TackyFunctionDefinition::from(
            program.function_definition().clone(),
        ))
    }
}

impl From<FunctionDefinition> for TackyFunctionDefinition {
    fn from(fd: FunctionDefinition) -> Self {
        trace!("[tacky] <function> {}", fd.name().value());
        let mut instructions = TackyInstruction::from_block(fd.body().clone());
        instructions.push(TackyInstruction::Return(TackyValue::Constant(0)));
        info!("[tacky] {} instructions", instructions.len());
        TackyFunctionDefinition::new(TackyIdentifier::from(fd.name().clone()), instructions)
    }
}

impl From<Identifier> for TackyIdentifier {
    fn from(value: Identifier) -> Self {
        TackyIdentifier {
            value: value.value().to_string(),
        }
    }
}

impl TackyInstruction {
    fn from_block(block: Block) -> Vec<TackyInstruction> {
        block
            .block_items()
            .clone()
            .into_iter()
            .flat_map(TackyInstruction::from_block_item)
            .collect()
    }

    fn from_block_item(block_item: BlockItem) -> Vec<TackyInstruction> {
        match block_item {
            BlockItem::S(s) => TackyInstruction::from_st(s),
            BlockItem::D(d) => TackyInstruction::from_decl(d),
        }
    }

    fn from_st(statement: Statement) -> Vec<TackyInstruction> {
        let mut instructions = vec![];
        match statement {
            Statement::Return(expr) => {
                trace!("[tacky] <statement> return");
                let v = TackyInstruction::from_expr(expr, &mut instructions);
                instructions.push(TackyInstruction::Return(v));
                instructions
            }
            Statement::Expression(expr) => {
                trace!("[tacky] <statement> expression");
                let _ = TackyInstruction::from_expr(expr, &mut instructions);
                instructions
            }
            Statement::If(cond, then, el) => {
                trace!("[tacky] <statement> if");
                let else_label = TackyIdentifier::new("else_label");
                let end_label = TackyIdentifier::new("end");
                let cond_result = TackyInstruction::from_expr(*cond, &mut instructions);
                let c = TackyValue::Var(TackyIdentifier::new("c"));
                instructions.push(TackyInstruction::Copy(cond_result, c.clone()));
                instructions.push(TackyInstruction::JumpIfZero(c, else_label.clone()));
                instructions.extend(TackyInstruction::from_st(*then));
                instructions.push(TackyInstruction::Jump(end_label.clone()));
                instructions.push(TackyInstruction::Label(else_label));
                if let Some(e) = el {
                    debug!("[tacky] else branch");
                    instructions.extend(TackyInstruction::from_st(*e));
                }
                instructions.push(TackyInstruction::Label(end_label));
                instructions
            }
            Statement::Compound(block) => {
                trace!("[tacky] <statement> compound");
                instructions.extend(TackyInstruction::from_block(*block));
                instructions
            }
            Statement::Break(_id) => todo!("to be implemented"),
            Statement::Continue(_id) => todo!("to be implemented"),
            Statement::While(_cond, _body, _id) => todo!("to be implemented"),
            Statement::DoWhile(_body, _cond, _id) => todo!("to be implemented"),
            Statement::For(_init, _cond, _post, _body, _id) => todo!("to be implemented"),
            Statement::Null => vec![],
        }
    }

    fn from_decl(declaration: Declaration) -> Vec<TackyInstruction> {
        let mut instructions = vec![];
        let Some(initializer) = declaration.initializer().cloned() else {
            return instructions;
        };
        let v = TackyInstruction::from_expr(initializer, &mut instructions);
        instructions.push(TackyInstruction::Copy(
            v,
            TackyValue::Var(TackyIdentifier::from(declaration.name().clone())),
        ));
        instructions
    }

    fn from_expr(expr: Expression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
        match expr {
            Expression::Conditional(cond, then, el) => {
                trace!("[tacky] <exp> conditional");
                let result = TackyValue::Var(TackyIdentifier::new("result"));
                let c_result = TackyValue::Var(TackyIdentifier::new("c_result"));
                let e2_label = TackyIdentifier::new("e2_label");
                let end_label = TackyIdentifier::new("end");

                let cond_val = TackyInstruction::from_expr(*cond, instructions);
                instructions.push(TackyInstruction::Copy(cond_val, c_result.clone()));
                instructions.push(TackyInstruction::JumpIfZero(c_result, e2_label.clone()));

                let e1 = TackyInstruction::from_expr(*then, instructions);
                let v1 = TackyValue::Var(TackyIdentifier::new("v1"));
                instructions.push(TackyInstruction::Copy(e1, v1.clone()));
                instructions.push(TackyInstruction::Copy(v1, result.clone()));
                instructions.push(TackyInstruction::Jump(end_label.clone()));

                instructions.push(TackyInstruction::Label(e2_label));
                let e2 = TackyInstruction::from_expr(*el, instructions);
                let v2 = TackyValue::Var(TackyIdentifier::new("v2"));
                instructions.push(TackyInstruction::Copy(e2, v2.clone()));
                instructions.push(TackyInstruction::Copy(v2, result.clone()));
                instructions.push(TackyInstruction::Label(end_label));
                result
            }
            Expression::Assignment(left, right) => {
                trace!("[tacky] <exp> assignment");
                let res = TackyInstruction::from_expr(*right, instructions);
                let left_var = match *left {
                    Expression::Var(id) => TackyValue::Var(TackyIdentifier::from(id)),
                    _ => panic!("invalid lvalue in assignment"),
                };
                instructions.push(TackyInstruction::Copy(res, left_var.clone()));
                left_var
            }
            Expression::Var(id) => TackyValue::Var(TackyIdentifier::from(id)),
            Expression::Constant(c) => TackyValue::Constant(c),
            Expression::Unary(op, inner) => {
                trace!("[tacky] <exp> unary {op:?}");
                let src = TackyInstruction::from_expr(*inner, instructions);
                let dst = TackyValue::Var(TackyIdentifier::new("unary_op"));
                instructions.push(TackyInstruction::Unary(
                    TackyUnaryOperator::from(op),
                    src,
                    dst.clone(),
                ));
                dst
            }
            Expression::Binary(op, left, right) => {
                trace!("[tacky] <exp> binary {op:?}");
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
                let v1 = TackyInstruction::from_expr(*left, instructions);
                instructions.push(TackyInstruction::JumpIfZero(v1, false_label.clone()));
                let v2 = TackyInstruction::from_expr(*right, instructions);
                instructions.push(TackyInstruction::JumpIfZero(v2, false_label.clone()));
                instructions.push(TackyInstruction::Copy(
                    TackyValue::Constant(1),
                    result.clone(),
                ));
                instructions.push(TackyInstruction::Jump(end_label.clone()));
                instructions.push(TackyInstruction::Label(false_label));
                instructions.push(TackyInstruction::Copy(
                    TackyValue::Constant(0),
                    result.clone(),
                ));
                instructions.push(TackyInstruction::Label(end_label));
                result
            }
            BinaryOperator::Or => {
                let result = TackyValue::Var(TackyIdentifier::new("or_result"));
                let true_label = TackyIdentifier::new("true_label");
                let end_label = TackyIdentifier::new("end");
                let v1 = TackyInstruction::from_expr(*left, instructions);
                instructions.push(TackyInstruction::JumpIfNotZero(v1, true_label.clone()));
                let v2 = TackyInstruction::from_expr(*right, instructions);
                instructions.push(TackyInstruction::JumpIfNotZero(v2, true_label.clone()));
                instructions.push(TackyInstruction::Copy(
                    TackyValue::Constant(0),
                    result.clone(),
                ));
                instructions.push(TackyInstruction::Jump(end_label.clone()));
                instructions.push(TackyInstruction::Label(true_label));
                instructions.push(TackyInstruction::Copy(
                    TackyValue::Constant(1),
                    result.clone(),
                ));
                instructions.push(TackyInstruction::Label(end_label));
                result
            }
            _ => {
                let v1 = TackyInstruction::from_expr(*left, instructions);
                let v2 = TackyInstruction::from_expr(*right, instructions);
                let dst = TackyValue::Var(TackyIdentifier::new("binary_op"));
                instructions.push(TackyInstruction::Binary(
                    TackyBinaryOperator::from(op),
                    v1,
                    v2,
                    dst.clone(),
                ));
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
            UnaryOperator::Not => TackyUnaryOperator::Not,
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
            BinaryOperator::BitwiseAnd => TackyBinaryOperator::BitwiseAnd,
            BinaryOperator::BitwiseOr => TackyBinaryOperator::BitwiseOr,
            BinaryOperator::BitwiseXor => TackyBinaryOperator::BitwiseXor,
            BinaryOperator::LeftShift => TackyBinaryOperator::LeftShift,
            BinaryOperator::RightShift => TackyBinaryOperator::RightShift,
            BinaryOperator::Equal => TackyBinaryOperator::Equal,
            BinaryOperator::NotEqual => TackyBinaryOperator::NotEqual,
            BinaryOperator::GreaterThan => TackyBinaryOperator::GreaterThan,
            BinaryOperator::LessThan => TackyBinaryOperator::LessThan,
            BinaryOperator::GreaterThanOrEqual => TackyBinaryOperator::GreaterThanOrEqual,
            BinaryOperator::LessThanOrEqual => TackyBinaryOperator::LessThanOrEqual,
            BinaryOperator::And | BinaryOperator::Or => {
                panic!("short-circuit ops handled separately")
            }
        }
    }
}
