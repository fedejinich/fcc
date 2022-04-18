#[cfg(test)]
mod test {
    use fcc::ast::ast_item::ASTItem;
    use fcc::ast::c_ast::{
        expression::Expression, function_declaration::FunctionDeclaration, program::Program,
        statement::Statement,
    };

    #[test]
    fn pretty_print() {
        let expression = Expression::new(2);
        let statement = Statement::new(expression);
        let function_declaration = FunctionDeclaration::new(String::from("main"), statement);
        let program = Program::new(function_declaration);

        println!("{}", program.pretty_print());

        assert_eq!(
            "PROGRAM:\n  FUN main:\n    RETURN Int<2>",
            program.pretty_print()
        );

        println!("{}", program.pretty_print_2());

        assert_eq!(
            "Program(\n  Function(\n    name=\"main\",\n    body=Return(\n      Const(2)\n    )\n)",
            program.pretty_print_2()
        )
    }
}
