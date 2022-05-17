#[cfg(test)]
mod test {
    use fcc::ast::{
        c_ast::{
            expression::Expression, function_definition::FunctionDefinition, program::Program,
            statement::Statement,
        },
        print::Printable,
    };

    #[test]
    fn pretty_print() {
        let expression = Expression::Constant(2);
        let statement = Statement::ReturnStatement { expression };
        let function_declaration = FunctionDefinition::new(String::from("main"), vec![statement]);
        let program = Program::new(function_declaration);

        println!("{}", program.print());

        assert_eq!(
            "Program(\n  Function(\n    name=\"main\",\n    body=Return(\n      Const(2)\n    )\n)",
            program.print()
        )
    }
}
