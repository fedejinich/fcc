#[cfg(test)]
mod test {
    use fcc::{
        ast::{
            expression::Expression, function_declaration::FunctionDeclaration, program::Program,
            statement::Statement,
        },
        parser::{self},
        token::Token,
    };

    #[test]
    fn parse() {
        let program = parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(2),
            Token::Semicolon,
            Token::CloseBrace,
        ]);

        let expression = Expression::new(2);
        let statement = Statement::new(expression);
        let function_declaration = FunctionDeclaration::new(String::from("main"), statement);
        let expected_program = Program::new(function_declaration);

        assert_eq!(expected_program, program)
    }
}
