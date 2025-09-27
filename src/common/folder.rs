use crate::codegen::x64::asm::{
    AsmBinaryOperator, AsmCondCode, AsmFunctionDefinition, AsmIdetifier, AsmInstruction,
    AsmOperand, AsmProgram, AsmUnaryOperator, Reg,
};

/// A folder is a trait that can be used to fold an AST into another AST.
pub trait FolderAsm {
    fn create() -> Self;

    fn fold_program(&mut self, program: &AsmProgram) -> AsmProgram {
        AsmProgram::new(self.fold_function_definition(&program.function_definition))
    }

    fn fold_function_definition(
        &mut self,
        function: &AsmFunctionDefinition,
    ) -> AsmFunctionDefinition {
        AsmFunctionDefinition::new(
            self.fold_identifier(&function.name),
            function
                .instructions
                .iter()
                .flat_map(|i| self.fold_instruction(i))
                .collect(),
        )
    }

    fn fold_instruction(&mut self, instruction: &AsmInstruction) -> Vec<AsmInstruction> {
        use AsmInstruction::*;
        let res = match instruction {
            Comment(comment) => Comment(comment.clone()),
            Mov(src, dst) => Mov(self.fold_operand(src), self.fold_operand(dst)),
            Unary(op, operand) => Unary(self.fold_unary_operator(op), self.fold_operand(operand)),
            Binary(op, src, dst) => Binary(
                self.fold_binary_operator(op),
                self.fold_operand(src),
                self.fold_operand(dst),
            ),
            Cmp(src, dst) => Cmp(self.fold_operand(src), self.fold_operand(dst)),
            Idiv(operand) => Idiv(self.fold_operand(operand)),
            Cdq => Cdq,
            Jmp(identifier) => Jmp(self.fold_identifier(identifier)),
            JmpCC(code, identifier) => {
                JmpCC(self.fold_cond_code(code), self.fold_identifier(identifier))
            }
            SetCC(code, operand) => SetCC(self.fold_cond_code(code), self.fold_operand(operand)),
            Label(identifier) => Label(self.fold_identifier(identifier)),
            AllocateStack(size) => AllocateStack(*size),
            Ret => Ret,
        };

        vec![res]
    }

    fn fold_operand(&mut self, operand: &AsmOperand) -> AsmOperand {
        use AsmOperand::*;
        match operand {
            Imm(value) => Imm(*value),
            Register(reg) => Register(self.fold_reg(reg)),
            Pseudo(identifier) => Pseudo(self.fold_identifier(identifier)),
            Stack(size) => Stack(*size),
        }
    }

    fn fold_identifier(&mut self, identifier: &AsmIdetifier) -> AsmIdetifier {
        identifier.clone()
    }

    fn fold_unary_operator(&mut self, operator: &AsmUnaryOperator) -> AsmUnaryOperator {
        operator.clone()
    }

    fn fold_binary_operator(&mut self, operator: &AsmBinaryOperator) -> AsmBinaryOperator {
        operator.clone()
    }

    fn fold_cond_code(&mut self, code: &AsmCondCode) -> AsmCondCode {
        code.clone()
    }
    fn fold_reg(&mut self, reg: &Reg) -> Reg {
        reg.clone()
    }
}
