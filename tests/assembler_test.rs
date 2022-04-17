#[cfg(test)]
mod test {
    use fcc::{
        code_generator,
        items::{
            expression::Expression, function_declaration::FunctionDeclaration, program::Program,
            statement::Statement,
        },
    };

    #[test]
    fn return_2() {
        let expression = Expression::new(2);
        let statement = Statement::new(expression);
        let function_declaration = FunctionDeclaration::new(String::from("main"), statement);

        let program = Program::new(function_declaration);

        assert_eq!(
            " .globl _main\n_main:\nmovl    $2, %eax\nret",
            code_generator::generate(program, String::from("return_2.s"))
        )
    }
}
