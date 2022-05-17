#[cfg(test)]
mod test {
    use fcc::{
        ast::c_ast::{
            expression::Expression, function_definition::FunctionDefinition, program::Program,
            statement::Statement,
        },
        parser::{self},
        token::Token,
    };

    #[test]
    fn valid_parse() {
        let program = parser::c_parser::parse(vec![
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

        let expression = Expression::Constant(2);
        let statement = Statement::ReturnStatement { expression };
        let function_declaration = FunctionDefinition::new(String::from("main"), vec![statement]);
        let expected_program = Program::new(function_declaration);

        assert_eq!(expected_program, program)
    }

    #[test]
    #[should_panic = "expected token: 'return', found: 'RETURN'"]
    fn invalid_wrong_case() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Invalid(String::from("RETURN")),
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }

    #[test]
    #[should_panic = "expected token: 'return', found: 'returne'"]
    fn invalid_exceed_keyword() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Invalid(String::from("returne")),
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }

    #[test]
    #[should_panic = "expected token: ')', found: '{'"]
    fn invalid_missing_paren() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }

    #[test]
    #[should_panic = "expected token: 'IntegerLiteral', found: ';'"]
    fn invalid_missing_retval() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }

    #[test]
    #[should_panic = "expected token: '}', found: 'END'"]
    fn invalid_no_brace() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
        ]);
    }

    #[test]
    #[should_panic = "expected token: ';', found: '}'"]
    fn invalid_no_semicolon() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::CloseBrace,
        ]);
    }

    #[test]
    #[should_panic = "expected token: 'return', found: 'return0'"]
    fn invalid_no_space() {
        parser::c_parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Invalid(String::from("return0")),
            Token::Semicolon,
            Token::CloseBrace,
        ]);
    }
}
