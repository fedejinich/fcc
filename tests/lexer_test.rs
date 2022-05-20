#[cfg(test)]
mod test {
    use fcc::{file_util::FileUtil, lexer::Lexer, token::Token};

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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/return_2.c")
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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/return_0.c")
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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/newlines.c")
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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/no_newlines.c")
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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/spaces.c")
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
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/valid/multi_digit.c")
        );
    }

    #[test]
    #[should_panic(expected = "found invalid word: RETURN")]
    fn invalid_wrong_case() {
        lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/invalid/wrong_case.c");
    }

    #[test]
    #[should_panic(expected = "found invalid word: returne")]
    fn invalid_exceed_keyword() {
        lex_by_file_path("tests/resources/stage_1/invalid/exceed_keyword.c"); // this is not taken from the test suite
    }

    #[test]
    #[should_panic(expected = "found invalid word: return0")]
    fn invalid_no_space() {
        lex_by_file_path("tests/resources/write_a_c_compiler/stage_1/invalid/no_space.c");
    }

    // unary operators

    #[test]
    fn valid_bitwise_zero() {
        assert_eq!(
            vec![
                Token::IntKeyword,
                Token::Identifier(String::from("main")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::ReturnKeyword,
                Token::Tilde,
                Token::IntegerLiteral(0),
                Token::Semicolon,
                Token::CloseBrace
            ],
            lex_by_file_path("tests/resources/write_a_c_compiler/stage_2/valid/bitwise_zero.c")
        );
    }

    fn lex_by_file_path(file_path: &str) -> Vec<Token> {
        let program = FileUtil::new().read_path_buff_to_string(&file_path);
        Lexer::new().lex(program.as_slice(), Vec::new())
    }
}
