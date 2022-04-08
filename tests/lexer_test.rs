#[cfg(test)]
mod test {
    use fcc::{lexer::*, file_reader};

    #[test]
    fn return_2() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(2),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("return_2.c"));
    }

    #[test]
    fn return_0() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/valid/return_0.c"));
    }

    #[test]
    fn newlines() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/valid/newlines.c"));
    }

    #[test]
    fn no_newlines() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/valid/no_newlines.c"));
    }

    #[test]
    fn spaces() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/valid/spaces.c"));
    }

    #[test]
    fn multi_digit() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(100),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/valid/multi_digit.c"));
    }

    #[test]
    fn wrong_case() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::Invalid(String::from("RETURN")),
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/wrong_case.c")); 
    }

    #[test]
    fn exceed_keyword() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis, 
            Token::CloseParenthesis, 
            Token::OpenBrace,
            Token::Invalid(String::from("returne")),
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/exceed_keyword.c")); 
    }

    #[test]
    fn missing_paren() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/missing_paren.c")); 
    }

    #[test]
    fn missing_retval() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/missing_retval.c")); 
    }

    #[test]
    fn no_brace() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::Semicolon
        ], lex_by_file_path("tests/resources/stage_1/invalid/no_brace.c")); 
    }

    #[test]
    fn no_semicolon() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral(0),
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/no_semicolon.c")); 
    }

    #[test]
    fn no_space() {
        assert_eq!(vec![
            Token::IntKeyword,
            Token::Identifier(String::from("main")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Invalid(String::from("return0")),
            Token::Semicolon,
            Token::CloseBrace
        ], lex_by_file_path("tests/resources/stage_1/invalid/no_space.c")); 
    }

    fn lex_by_file_path(file_path: &str) -> Vec<Token> {
        let program = file_reader::read_file_to_string(file_path).unwrap();
        lex(program.as_slice(), Vec::new())
    }
} 