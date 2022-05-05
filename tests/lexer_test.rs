#[cfg(test)]
mod test {
    use fcc::{file_util::FileUtil, lexer::lex, token::Token};
    use std::path::PathBuf;

    #[test]
    fn valid_return_2() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(2),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/return_2.c")
        );
    }

    #[test]
    fn valid_return_0() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/return_0.c")
        );
    }

    #[test]
    fn valid_newlines() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/newlines.c")
        );
    }

    #[test]
    fn valid_no_newlines() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/no_newlines.c")
        );
    }

    #[test]
    fn valid_spaces() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/spaces.c")
        );
    }

    #[test]
    fn valid_multi_digit() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(100),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/valid/multi_digit.c")
        );
    }

    #[test]
    fn invalid_wrong_case() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Invalid(String::from("RETURN")),
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/wrong_case.c")
        );
    }

    #[test]
    fn invalid_exceed_keyword() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Invalid(String::from("returne")),
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/exceed_keyword.c")
        );
    }

    #[test]
    fn invalid_missing_paren() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/missing_paren.c")
        );
    }

    #[test]
    fn invalid_missing_retval() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/missing_retval.c")
        );
    }

    #[test]
    fn invalid_no_brace() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::Semicolon
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/no_brace.c")
        );
    }

    #[test]
    fn invalid_no_semicolon() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::IntegerLiteral(0),
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/no_semicolon.c")
        );
    }

    #[test]
    fn invalid_no_space() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Invalid(String::from("return0")),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/stage_1/invalid/no_space.c")
        );
    }

    fn lex_by_file_path(file_path: &str) -> Vec<Token> {
        let mut path_buff = PathBuf::new();
        path_buff.push(file_path);

        let program = FileUtil::new().read_path_buff_to_string(&path_buff);
        lex(program.as_slice(), Vec::new())
    }
}
