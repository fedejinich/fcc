#[cfg(test)]
mod test {
    use fcc::{parser::{*, self}, lexer::Token};

    #[test]
    fn parse() {
        let _program = parser::parse(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(2),
            Token::Semicolon,
            Token::CloseBrace
        ]);

        assert_eq!(1,1)
    }
}