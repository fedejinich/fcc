use crate::c_ast::ast::{
    BinaryOperator, Block, BlockItem, Declaration, Expression, ForInit, FunctionDefinition,
    Identifier, Program, Statement, UnaryOperator,
};
use crate::codegen::x64::ast::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmIdetifier, AsmInstruction,
    AsmOperand, AsmProgram, AsmUnaryOperator, Reg,
};
use crate::tacky::ast::{
    TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram,
    TackyUnaryOperator, TackyValue,
};

/// A 'folder' is a trait that can be used to fold a C AST into another C AST.
/// In this trait we provide the default implementation to traverse the entire AST and return
/// another AST. This is useful when we want to traverse the AST and perform some operation on a
/// specific node type.
pub trait FolderC {
    fn fold_prog(&mut self, program: Program) -> Result<Program, String> {
        Ok(Program::new(
            self.fold_fun_def(program.function_definition().clone())?,
        ))
    }

    fn fold_fun_def(&mut self, function: FunctionDefinition) -> Result<FunctionDefinition, String> {
        Ok(FunctionDefinition::new(
            self.fold_id(function.name().clone())?,
            self.fold_block(function.body().clone())?,
        ))
    }

    fn fold_block(&mut self, block: Block) -> Result<Block, String> {
        let folded: Result<Vec<_>, String> = block
            .block_items()
            .clone()
            .into_iter()
            .map(|item| self.fold_block_i(item))
            .collect();
        Ok(Block::new(folded?))
    }

    fn fold_block_i(&mut self, item: BlockItem) -> Result<BlockItem, String> {
        match item {
            BlockItem::D(declaration) => Ok(BlockItem::D(self.fold_decl(declaration)?)),
            BlockItem::S(statement) => Ok(BlockItem::S(self.fold_st(statement)?)),
        }
    }

    fn fold_decl(&mut self, declaration: Declaration) -> Result<Declaration, String> {
        let initializer = match declaration.initializer() {
            Some(expr) => Some(self.fold_expr(expr.clone())?),
            None => None,
        };
        Ok(Declaration::new(
            self.fold_id(declaration.name().clone())?,
            initializer,
        ))
    }

    fn fold_for_init(&mut self, init: ForInit) -> Result<ForInit, String> {
        match init {
            ForInit::InitDecl(decl) => Ok(ForInit::InitDecl(Box::new(self.fold_decl(*decl)?))),
            ForInit::InitExp(expr) => Ok(ForInit::InitExp(Box::new(self.fold_expr(*expr)?))),
        }
    }

    fn fold_st(&mut self, statement: Statement) -> Result<Statement, String> {
        self.default_fold_st(statement)
    }

    fn default_fold_st(&mut self, statement: Statement) -> Result<Statement, String> {
        match statement {
            Statement::Return(expr) => Ok(Statement::Return(self.fold_expr(expr)?)),
            Statement::Expression(expr) => Ok(Statement::Expression(self.fold_expr(expr)?)),
            Statement::If(expr, then, el) => Ok(Statement::If(
                Box::new(self.fold_expr(*expr)?),
                Box::new(self.fold_st(*then)?),
                if let Some(el) = el {
                    Some(Box::new(self.fold_st(*el)?))
                } else {
                    None
                },
            )),
            Statement::Compound(block) => {
                Ok(Statement::Compound(Box::new(self.fold_block(*block)?)))
            }
            Statement::Break => Ok(Statement::Break),
            Statement::Continue => Ok(Statement::Continue),
            Statement::While(cond, body) => Ok(Statement::While(
                Box::new(self.fold_expr(*cond)?),
                Box::new(self.fold_st(*body)?),
            )),
            Statement::DoWhile(body, cond) => Ok(Statement::DoWhile(
                Box::new(self.fold_st(*body)?),
                Box::new(self.fold_expr(*cond)?),
            )),
            Statement::For(for_init, cond, post, body) => {
                let for_init = Box::new(self.fold_for_init(*for_init)?);
                let cond = if let Some(cond) = cond {
                    Some(Box::new(self.fold_expr(*cond)?))
                } else {
                    None
                };
                let post = if let Some(post) = post {
                    Some(Box::new(self.fold_expr(*post)?))
                } else {
                    None
                };
                let body = Box::new(self.fold_st(*body)?);

                Ok(Statement::For(for_init, cond, post, body))
            }
            Statement::Null => Ok(Statement::Null),
        }
    }

    fn fold_expr(&mut self, expression: Expression) -> Result<Expression, String> {
        match expression {
            Expression::Constant(value) => Ok(Expression::Constant(value)),
            Expression::Var(identifier) => Ok(Expression::Var(self.fold_id(identifier)?)),
            Expression::Unary(op, expr) => Ok(Expression::Unary(
                self.fold_un_op(op)?,
                Box::new(self.fold_expr(*expr)?),
            )),
            Expression::Binary(op, left, right) => Ok(Expression::Binary(
                self.fold_bin_op(op)?,
                Box::new(self.fold_expr(*left)?),
                Box::new(self.fold_expr(*right)?),
            )),
            Expression::Assignment(left, right) => Ok(Expression::Assignment(
                Box::new(self.fold_expr(*left)?),
                Box::new(self.fold_expr(*right)?),
            )),
            Expression::Conditional(cond, then, el) => Ok(Expression::Conditional(
                Box::new(self.fold_expr(*cond)?),
                Box::new(self.fold_expr(*then)?),
                Box::new(self.fold_expr(*el)?),
            )),
        }
    }

    fn fold_id(&mut self, identifier: Identifier) -> Result<Identifier, String> {
        Ok(identifier)
    }

    fn fold_un_op(&mut self, operator: UnaryOperator) -> Result<UnaryOperator, String> {
        Ok(operator)
    }

    fn fold_bin_op(&mut self, operator: BinaryOperator) -> Result<BinaryOperator, String> {
        Ok(operator)
    }
}

/// Another folder trait that can be used to fold Tacky AST into another Tacky AST.
#[allow(unused)]
pub trait FolderTacky {
    fn create() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    fn fold_prog(&mut self, program: TackyProgram) -> Result<TackyProgram, String> {
        Ok(TackyProgram::new(
            self.fold_fun_def(program.function_definition)?,
        ))
    }

    fn fold_fun_def(
        &mut self,
        function: TackyFunctionDefinition,
    ) -> Result<TackyFunctionDefinition, String> {
        let instructions: Result<Vec<_>, String> = function
            .instructions
            .into_iter()
            .map(|i| self.fold_instruction(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());
        Ok(TackyFunctionDefinition::new(
            self.fold_id(function.name)?,
            instructions?,
        ))
    }

    fn fold_instruction(
        &mut self,
        instruction: TackyInstruction,
    ) -> Result<Vec<TackyInstruction>, String> {
        use TackyInstruction::*;
        let res = match instruction {
            Comment(comment) => Comment(comment),
            Return(value) => Return(self.fold_val(value)?),
            Unary(op, src, dst) => Unary(
                self.fold_un_op(op)?,
                self.fold_val(src)?,
                self.fold_val(dst)?,
            ),
            Binary(op, src1, src2, dst) => Binary(
                self.fold_bin_op(op)?,
                self.fold_val(src1)?,
                self.fold_val(src2)?,
                self.fold_val(dst)?,
            ),
            Copy(src, dst) => Copy(self.fold_val(src)?, self.fold_val(dst)?),
            Jump(identifier) => Jump(self.fold_id(identifier)?),
            JumpIfZero(value, identifier) => {
                JumpIfZero(self.fold_val(value)?, self.fold_id(identifier)?)
            }
            JumpIfNotZero(value, identifier) => {
                JumpIfNotZero(self.fold_val(value)?, self.fold_id(identifier)?)
            }
            Label(identifier) => Label(self.fold_id(identifier)?),
        };

        Ok(vec![res])
    }

    fn fold_val(&mut self, value: TackyValue) -> Result<TackyValue, String> {
        match value {
            TackyValue::Constant(val) => Ok(TackyValue::Constant(val)),
            TackyValue::Var(identifier) => Ok(TackyValue::Var(self.fold_id(identifier)?)),
        }
    }

    fn fold_id(&mut self, identifier: TackyIdentifier) -> Result<TackyIdentifier, String> {
        Ok(identifier)
    }

    fn fold_un_op(&mut self, operator: TackyUnaryOperator) -> Result<TackyUnaryOperator, String> {
        Ok(operator)
    }

    fn fold_bin_op(
        &mut self,
        operator: TackyBinaryOperator,
    ) -> Result<TackyBinaryOperator, String> {
        Ok(operator)
    }
}

/// Another folder trait that can be used to fold Asm AST into another Asm AST.
pub trait FolderAsm {
    fn create() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    fn fold_prog(&mut self, program: AsmProgram) -> Result<AsmProgram, String> {
        Ok(AsmProgram::new(
            self.fold_fun_def(program.function_definition)?,
        ))
    }

    fn fold_fun_def(
        &mut self,
        function: AsmFunctionDefinition,
    ) -> Result<AsmFunctionDefinition, String> {
        let instructions: Result<Vec<_>, String> = function
            .instructions
            .into_iter()
            .map(|i| self.fold_ins(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());
        Ok(AsmFunctionDefinition::new(
            self.fold_id(function.name)?,
            instructions?,
        ))
    }

    fn fold_ins(&mut self, instruction: AsmInstruction) -> Result<Vec<AsmInstruction>, String> {
        use AsmInstruction::*;
        let res = match instruction {
            Comment(comment) => Comment(comment),
            Mov(src, dst) => Mov(self.fold_op(src)?, self.fold_op(dst)?),
            Unary(op, operand) => Unary(self.fold_un_op(op)?, self.fold_op(operand)?),
            Binary(op, src, dst) => Binary(
                self.fold_bin_op(op)?,
                self.fold_op(src)?,
                self.fold_op(dst)?,
            ),
            Cmp(src, dst) => Cmp(self.fold_op(src)?, self.fold_op(dst)?),
            Idiv(operand) => Idiv(self.fold_op(operand)?),
            Cdq => Cdq,
            Jmp(identifier) => Jmp(self.fold_id(identifier)?),
            JmpCC(code, identifier) => JmpCC(self.fold_cond_code(code)?, self.fold_id(identifier)?),
            SetCC(code, operand) => SetCC(self.fold_cond_code(code)?, self.fold_op(operand)?),
            Label(identifier) => Label(self.fold_id(identifier)?),
            AllocateStack(size) => AllocateStack(size),
            Ret => Ret,
        };

        Ok(vec![res])
    }

    fn fold_op(&mut self, operand: AsmOperand) -> Result<AsmOperand, String> {
        use AsmOperand::*;
        match operand {
            Imm(value) => Ok(Imm(value)),
            Register(reg) => Ok(Register(self.fold_reg(reg)?)),
            Pseudo(identifier) => Ok(Pseudo(self.fold_id(identifier)?)),
            Stack(size) => Ok(Stack(size)),
        }
    }

    fn fold_id(&mut self, identifier: AsmIdetifier) -> Result<AsmIdetifier, String> {
        Ok(identifier)
    }

    fn fold_un_op(&mut self, operator: AsmUnaryOperator) -> Result<AsmUnaryOperator, String> {
        Ok(operator)
    }

    fn fold_bin_op(&mut self, operator: AsmBinaryOperator) -> Result<AsmBinaryOperator, String> {
        Ok(operator)
    }

    fn fold_cond_code(&mut self, code: AsmCondCode) -> Result<AsmCondCode, String> {
        Ok(code)
    }
    fn fold_reg(&mut self, reg: Reg) -> Result<Reg, String> {
        Ok(reg)
    }
}
