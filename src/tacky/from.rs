//! This module contains the logic to lower the AST to TACKY IR.

use log::{debug, info, trace};

use crate::{
    c_ast::ast::{
        BinaryOperator, Block, BlockItem, Declaration, Expression, ForInit, FunctionDefinition,
        Identifier, Program, Statement, UnaryOperator,
    },
    tacky::{
        ast::{
            TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction,
            TackyProgram, TackyUnaryOperator, TackyValue,
        },
        builder::TackyBuilder,
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

        let mut builder = TackyBuilder::new();
        emit_block(fd.body().clone(), &mut builder);

        // add return 0 as last instruction (it's gonna be fixed in Part III)
        builder.emit_return(TackyValue::Constant(0));

        let instructions = builder.finish();
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

// ============================================================================
// Lowering functions using TackyBuilder
// ============================================================================

fn emit_block(block: Block, builder: &mut TackyBuilder) {
    for item in block.block_items().clone() {
        emit_block_item(item, builder);
    }
}

fn emit_block_item(block_item: BlockItem, builder: &mut TackyBuilder) {
    match block_item {
        BlockItem::S(s) => emit_statement(s, builder),
        BlockItem::D(d) => emit_declaration(d, builder),
    }
}

fn emit_statement(statement: Statement, builder: &mut TackyBuilder) {
    match statement {
        Statement::Return(expr) => {
            trace!("[tacky] <statement> return");
            let v = emit_expr(expr, builder);
            builder.emit_return(v);
        }
        Statement::Expression(expr) => {
            trace!("[tacky] <statement> expression");
            let _ = emit_expr(expr, builder);
        }
        Statement::If(cond, then, el) => {
            trace!("[tacky] <statement> if");

            let else_label = builder.fresh_label("else");
            let end_label = builder.fresh_label("end");

            // emit condition
            let cond_result = emit_expr(*cond, builder);
            let c = builder.fresh_temp("cond");
            builder.emit_copy(cond_result, c.clone());
            builder.emit_jump_if_zero(c, else_label.clone());

            // emit then branch
            emit_statement(*then, builder);
            builder.emit_jump(end_label.clone());

            // emit else branch
            builder.emit_label(else_label);
            if let Some(e) = el {
                debug!("[tacky] else branch");
                emit_statement(*e, builder);
            }
            builder.emit_label(end_label);
        }
        Statement::Compound(block) => {
            trace!("[tacky] <statement> compound");
            emit_block(*block, builder);
        }
        Statement::Break(label) => {
            trace!("[tacky] <statement> break");
            let break_label = builder.label_with_prefix("break_", &label);
            builder.emit_jump(break_label);
        }
        Statement::Continue(label) => {
            trace!("[tacky] <statement> continue");
            let continue_label = builder.label_with_prefix("continue_", &label);
            builder.emit_jump(continue_label);
        }
        Statement::While(cond, body, label) => {
            trace!("[tacky] <statement> while");

            let continue_label = builder.label_with_prefix("continue_", &label);
            let break_label = builder.label_with_prefix("break_", &label);

            builder.emit_label(continue_label.clone());

            let res = emit_expr(*cond, builder);
            let v = builder.fresh_temp("while_cond");
            builder.emit_copy(res, v.clone());
            builder.emit_jump_if_zero(v, break_label.clone());

            emit_statement(*body, builder);
            builder.emit_jump(continue_label);
            builder.emit_label(break_label);
        }
        Statement::DoWhile(body, cond, label) => {
            trace!("[tacky] <statement> do-while");

            let start_label = builder.label_with_prefix("start_", &label);
            let continue_label = builder.label_with_prefix("continue_", &label);
            let break_label = builder.label_with_prefix("break_", &label);

            builder.emit_label(start_label.clone());
            emit_statement(*body, builder);
            builder.emit_label(continue_label);

            let res = emit_expr(*cond, builder);
            let v = builder.fresh_temp("dowhile_cond");
            builder.emit_copy(res, v.clone());
            builder.emit_jump_if_not_zero(v, start_label);
            builder.emit_label(break_label);
        }
        Statement::For(for_init, cond, post, body, label) => {
            trace!("[tacky] <statement> for");

            let start_label = builder.label_with_prefix("start_", &label);
            let continue_label = builder.label_with_prefix("continue_", &label);
            let break_label = builder.label_with_prefix("break_", &label);

            // emit init
            trace!("[tacky] for init");
            emit_for_init(*for_init, builder);

            builder.emit_label(start_label.clone());

            // emit condition (if present)
            if let Some(cond) = cond {
                trace!("[tacky] for cond");
                let res = emit_expr(*cond, builder);
                let v = builder.fresh_temp("for_cond");
                builder.emit_copy(res, v.clone());
                builder.emit_jump_if_zero(v, break_label.clone());
            }

            // emit body
            emit_statement(*body, builder);
            builder.emit_label(continue_label);

            // emit post (if present)
            if let Some(post) = post {
                trace!("[tacky] for post");
                let _ = emit_expr(*post, builder);
            }

            builder.emit_jump(start_label);
            builder.emit_label(break_label);
        }
        Statement::Null => {}
    }
}

fn emit_declaration(declaration: Declaration, builder: &mut TackyBuilder) {
    let Some(initializer) = declaration.initializer().cloned() else {
        return;
    };

    let v = emit_expr(initializer, builder);
    let dst = TackyValue::Var(TackyIdentifier::from(declaration.name().clone()));
    builder.emit_copy(v, dst);
}

fn emit_for_init(for_init: ForInit, builder: &mut TackyBuilder) {
    match for_init {
        ForInit::InitDecl(declaration) => {
            trace!("[tacky] for init with declaration");
            emit_declaration(*declaration, builder);
        }
        ForInit::InitExp(expression) => {
            let Some(expression) = expression else {
                trace!("[tacky] for init with no expression");
                return;
            };
            trace!("[tacky] for init with expression");
            let _ = emit_expr(*expression, builder);
        }
    }
}

/// Lowers an Expression to TACKY.
/// Emits instructions via builder and returns a `TackyValue` identifying the result.
fn emit_expr(expr: Expression, builder: &mut TackyBuilder) -> TackyValue {
    match expr {
        Expression::Conditional(cond, then, el) => {
            trace!("[tacky] <exp> conditional");

            let result = builder.fresh_temp("ternary_result");
            let else_label = builder.fresh_label("ternary_else");
            let end_label = builder.fresh_label("ternary_end");

            // emit condition
            let cond_val = emit_expr(*cond, builder);
            let c_result = builder.fresh_temp("ternary_cond");
            builder.emit_copy(cond_val, c_result.clone());
            builder.emit_jump_if_zero(c_result, else_label.clone());

            // emit then expression
            let e1 = emit_expr(*then, builder);
            builder.emit_copy(e1, result.clone());
            builder.emit_jump(end_label.clone());

            // emit else expression
            builder.emit_label(else_label);
            let e2 = emit_expr(*el, builder);
            builder.emit_copy(e2, result.clone());
            builder.emit_label(end_label);

            result
        }
        Expression::Assignment(left, right) => {
            trace!("[tacky] <exp> assignment");

            let res = emit_expr(*right, builder);
            let left_var = match *left {
                Expression::Var(id) => TackyValue::Var(TackyIdentifier::from(id)),
                _ => panic!("invalid lvalue in assignment"),
            };
            builder.emit_copy(res, left_var.clone());

            left_var
        }
        Expression::Var(id) => TackyValue::Var(TackyIdentifier::from(id)),
        Expression::Constant(c) => TackyValue::Constant(c),
        Expression::Unary(op, inner) => {
            trace!("[tacky] <exp> unary {op:?}");

            let src = emit_expr(*inner, builder);
            let dst = builder.fresh_temp("unary");
            builder.emit(TackyInstruction::Unary(
                TackyUnaryOperator::from(op),
                src,
                dst.clone(),
            ));

            dst
        }
        Expression::Binary(op, left, right) => {
            trace!("[tacky] <exp> binary {op:?}");
            emit_binary_op(op, *left, *right, builder)
        }
    }
}

fn emit_binary_op(
    op: BinaryOperator,
    left: Expression,
    right: Expression,
    builder: &mut TackyBuilder,
) -> TackyValue {
    match op {
        // Short-circuit AND
        BinaryOperator::And => {
            let result = builder.fresh_temp("and_result");
            let false_label = builder.fresh_label("and_false");
            let end_label = builder.fresh_label("and_end");

            let v1 = emit_expr(left, builder);
            builder.emit_jump_if_zero(v1, false_label.clone());

            let v2 = emit_expr(right, builder);
            builder.emit_jump_if_zero(v2, false_label.clone());

            builder.emit_copy(TackyValue::Constant(1), result.clone());
            builder.emit_jump(end_label.clone());

            builder.emit_label(false_label);
            builder.emit_copy(TackyValue::Constant(0), result.clone());

            builder.emit_label(end_label);

            result
        }
        // Short-circuit OR
        BinaryOperator::Or => {
            let result = builder.fresh_temp("or_result");
            let true_label = builder.fresh_label("or_true");
            let end_label = builder.fresh_label("or_end");

            let v1 = emit_expr(left, builder);
            builder.emit_jump_if_not_zero(v1, true_label.clone());

            let v2 = emit_expr(right, builder);
            builder.emit_jump_if_not_zero(v2, true_label.clone());

            builder.emit_copy(TackyValue::Constant(0), result.clone());
            builder.emit_jump(end_label.clone());

            builder.emit_label(true_label);
            builder.emit_copy(TackyValue::Constant(1), result.clone());

            builder.emit_label(end_label);

            result
        }
        // Regular binary operators
        _ => {
            let v1 = emit_expr(left, builder);
            let v2 = emit_expr(right, builder);
            let dst = builder.fresh_temp("binop");

            builder.emit(TackyInstruction::Binary(
                TackyBinaryOperator::from(op),
                v1,
                v2,
                dst.clone(),
            ));

            dst
        }
    }
}

impl From<UnaryOperator> for TackyUnaryOperator {
    fn from(op: UnaryOperator) -> Self {
        match op {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
            // logical unary operators
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

