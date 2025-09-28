use crate::ast::program::{
    BinaryOperator, BlockItem, Declaration, Expression, FunctionDefinition, Identifier, Program,
    Statement, UnaryOperator,
};
use crate::codegen::x64::asm::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmIdetifier, AsmInstruction,
    AsmOperand, AsmProgram, AsmUnaryOperator, Reg,
};
use crate::tacky::program::{
    TackyBinaryOperator, TackyFunctionDefinition, TackyIdentifier, TackyInstruction, TackyProgram,
    TackyUnaryOperator, TackyValue,
};

/// A folder is a trait that can be used to fold an AST into another AST.
pub trait Folder {
    fn create() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    fn fold_program(&mut self, program: &Program) -> Result<Program, String> {
        Ok(Program::new(self.fold_function_definition(&program.function_definition)?))
    }

    fn fold_function_definition(&mut self, function: &FunctionDefinition) -> Result<FunctionDefinition, String> {
        let body: Result<Vec<_>, String> = function
            .body
            .iter()
            .map(|item| self.fold_block_item(item))
            .collect();
        Ok(FunctionDefinition::new(
            self.fold_identifier(&function.name)?,
            body?,
        ))
    }

    fn fold_block_item(&mut self, item: &BlockItem) -> Result<BlockItem, String> {
        use BlockItem::*;
        match item {
            S(statement) => Ok(S(self.fold_statement(statement)?)),
            D(declaration) => Ok(D(self.fold_declaration(declaration)?)),
        }
    }

    fn fold_declaration(&mut self, declaration: &Declaration) -> Result<Declaration, String> {
        let initializer = match &declaration.initializer {
            Some(expr) => Some(self.fold_expression(expr)?),
            None => None,
        };
        Ok(Declaration::new(
            self.fold_identifier(&declaration.name)?,
            initializer,
        ))
    }

    fn fold_statement(&mut self, statement: &Statement) -> Result<Statement, String> {
        use Statement::*;
        match statement {
            Return(expr) => Ok(Return(self.fold_expression(expr)?)),
            Expression(expr) => Ok(Expression(self.fold_expression(expr)?)),
            Null => Ok(Null),
        }
    }

    fn fold_expression(&mut self, expression: &Expression) -> Result<Expression, String> {
        use Expression::*;
        match expression {
            Constant(value) => Ok(Constant(*value)),
            Var(identifier) => Ok(Var(self.fold_identifier(identifier)?)),
            Unary(op, expr) => Ok(Unary(
                self.fold_unary_operator(op)?,
                Box::new(self.fold_expression(expr)?),
            )),
            Binary(op, left, right) => Ok(Binary(
                self.fold_binary_operator(op)?,
                Box::new(self.fold_expression(left)?),
                Box::new(self.fold_expression(right)?),
            )),
            Assignment(left, right) => Ok(Assignment(
                Box::new(self.fold_expression(left)?),
                Box::new(self.fold_expression(right)?),
            )),
        }
    }

    fn fold_identifier(&mut self, identifier: &Identifier) -> Result<Identifier, String> {
        Ok(identifier.clone())
    }

    fn fold_unary_operator(&mut self, operator: &UnaryOperator) -> Result<UnaryOperator, String> {
        Ok(operator.clone())
    }

    fn fold_binary_operator(&mut self, operator: &BinaryOperator) -> Result<BinaryOperator, String> {
        Ok(operator.clone())
    }
}

/// Another folder trait that can be used to fold Tacky AST into another Tacky AST.
pub trait FolderTacky {
    fn create() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    fn fold_program(&mut self, program: &TackyProgram) -> Result<TackyProgram, String> {
        Ok(TackyProgram::new(self.fold_function_definition(&program.function_definition)?))
    }

    fn fold_function_definition(
        &mut self,
        function: &TackyFunctionDefinition,
    ) -> Result<TackyFunctionDefinition, String> {
        let instructions: Result<Vec<_>, String> = function
            .instructions
            .iter()
            .map(|i| self.fold_instruction(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());
        Ok(TackyFunctionDefinition::new(
            self.fold_identifier(&function.name)?,
            instructions?,
        ))
    }

    fn fold_instruction(&mut self, instruction: &TackyInstruction) -> Result<Vec<TackyInstruction>, String> {
        use TackyInstruction::*;
        let res = match instruction {
            Return(value) => Return(self.fold_value(value)?),
            Unary(op, src, dst) => Unary(
                self.fold_unary_operator(op)?,
                self.fold_value(src)?,
                self.fold_value(dst)?,
            ),
            Binary(op, src1, src2, dst) => Binary(
                self.fold_binary_operator(op)?,
                self.fold_value(src1)?,
                self.fold_value(src2)?,
                self.fold_value(dst)?,
            ),
            Copy(src, dst) => Copy(self.fold_value(src)?, self.fold_value(dst)?),
            Jump(identifier) => Jump(self.fold_identifier(identifier)?),
            JumpIfZero(value, identifier) => {
                JumpIfZero(self.fold_value(value)?, self.fold_identifier(identifier)?)
            }
            JumpIfNotZero(value, identifier) => {
                JumpIfNotZero(self.fold_value(value)?, self.fold_identifier(identifier)?)
            }
            Label(identifier) => Label(self.fold_identifier(identifier)?),
        };

        Ok(vec![res])
    }

    fn fold_value(&mut self, value: &TackyValue) -> Result<TackyValue, String> {
        use TackyValue::*;
        match value {
            Constant(val) => Ok(Constant(*val)),
            Var(identifier) => Ok(Var(self.fold_identifier(identifier)?)),
        }
    }

    fn fold_identifier(&mut self, identifier: &TackyIdentifier) -> Result<TackyIdentifier, String> {
        Ok(identifier.clone())
    }

    fn fold_unary_operator(&mut self, operator: &TackyUnaryOperator) -> Result<TackyUnaryOperator, String> {
        Ok(operator.clone())
    }

    fn fold_binary_operator(&mut self, operator: &TackyBinaryOperator) -> Result<TackyBinaryOperator, String> {
        Ok(operator.clone())
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

    fn fold_program(&mut self, program: &AsmProgram) -> Result<AsmProgram, String> {
        Ok(AsmProgram::new(self.fold_function_definition(&program.function_definition)?))
    }

    fn fold_function_definition(
        &mut self,
        function: &AsmFunctionDefinition,
    ) -> Result<AsmFunctionDefinition, String> {
        let instructions: Result<Vec<_>, String> = function
            .instructions
            .iter()
            .map(|i| self.fold_instruction(i))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| v.into_iter().flatten().collect());
        Ok(AsmFunctionDefinition::new(
            self.fold_identifier(&function.name)?,
            instructions?,
        ))
    }

    fn fold_instruction(&mut self, instruction: &AsmInstruction) -> Result<Vec<AsmInstruction>, String> {
        use AsmInstruction::*;
        let res = match instruction {
            Comment(comment) => Comment(comment.clone()),
            Mov(src, dst) => Mov(self.fold_operand(src)?, self.fold_operand(dst)?),
            Unary(op, operand) => Unary(self.fold_unary_operator(op)?, self.fold_operand(operand)?),
            Binary(op, src, dst) => Binary(
                self.fold_binary_operator(op)?,
                self.fold_operand(src)?,
                self.fold_operand(dst)?,
            ),
            Cmp(src, dst) => Cmp(self.fold_operand(src)?, self.fold_operand(dst)?),
            Idiv(operand) => Idiv(self.fold_operand(operand)?),
            Cdq => Cdq,
            Jmp(identifier) => Jmp(self.fold_identifier(identifier)?),
            JmpCC(code, identifier) => {
                JmpCC(self.fold_cond_code(code)?, self.fold_identifier(identifier)?)
            }
            SetCC(code, operand) => SetCC(self.fold_cond_code(code)?, self.fold_operand(operand)?),
            Label(identifier) => Label(self.fold_identifier(identifier)?),
            AllocateStack(size) => AllocateStack(*size),
            Ret => Ret,
        };

        Ok(vec![res])
    }

    fn fold_operand(&mut self, operand: &AsmOperand) -> Result<AsmOperand, String> {
        use AsmOperand::*;
        match operand {
            Imm(value) => Ok(Imm(*value)),
            Register(reg) => Ok(Register(self.fold_reg(reg)?)),
            Pseudo(identifier) => Ok(Pseudo(self.fold_identifier(identifier)?)),
            Stack(size) => Ok(Stack(*size)),
        }
    }

    fn fold_identifier(&mut self, identifier: &AsmIdetifier) -> Result<AsmIdetifier, String> {
        Ok(identifier.clone())
    }

    fn fold_unary_operator(&mut self, operator: &AsmUnaryOperator) -> Result<AsmUnaryOperator, String> {
        Ok(operator.clone())
    }

    fn fold_binary_operator(&mut self, operator: &AsmBinaryOperator) -> Result<AsmBinaryOperator, String> {
        Ok(operator.clone())
    }

    fn fold_cond_code(&mut self, code: &AsmCondCode) -> Result<AsmCondCode, String> {
        Ok(code.clone())
    }
    fn fold_reg(&mut self, reg: &Reg) -> Result<Reg, String> {
        Ok(reg.clone())
    }
}
