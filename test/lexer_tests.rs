use fcc::lexer::{Token, lex};

#[test]
fn test_empty_input() {
    let result = lex("");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![]);
    }
}

#[test]
fn test_whitespace_only() {
    let result = lex("   \t\n  ");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![]);
    }
}

#[test]
fn test_single_identifier() {
    let result = lex("main");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Identifier("main".to_string())]);
    }
}

#[test]
fn test_keywords() {
    let result = lex("int void return");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Int, Token::Void, Token::Return]);
    }
}

#[test]
fn test_constants() {
    let result = lex("42 0 123");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Constant("42".to_string()),
                Token::Constant("0".to_string()),
                Token::Constant("123".to_string())
            ]
        );
    }
}

#[test]
fn test_symbols() {
    let result = lex("() {} ;");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::CloseParen,
                Token::OpenBrace,
                Token::CloseBrace,
                Token::Semicolon
            ]
        );
    }
}

#[test]
fn test_unary_operators() {
    let result = lex("~ - !");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Complement, Token::Negate, Token::Not]);
    }
}

#[test]
fn test_binary_operators() {
    let result = lex("+ - * / %");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Negate,
                Token::Multiply,
                Token::Divide,
                Token::Remainder
            ]
        );
    }
}

#[test]
fn test_bitwise_operators() {
    let result = lex("& | ^ << >>");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::BitwiseAnd,
                Token::BitwiseOr,
                Token::BitwiseXor,
                Token::LeftShift,
                Token::RightShift
            ]
        );
    }
}

#[test]
fn test_logical_operators() {
    let result = lex("&& ||");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::And, Token::Or]);
    }
}

#[test]
fn test_relational_operators() {
    let result = lex("== != < <= > >=");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Equal,
                Token::NotEqual,
                Token::LessThan,
                Token::LessThanOrEqual,
                Token::GreaterThan,
                Token::GreaterThanOrEqual
            ]
        );
    }
}

#[test]
fn test_assignment_and_decrement() {
    let result = lex("= --");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Assignment, Token::Decrement]);
    }
}

#[test]
fn test_simple_function() {
    let result = lex("int main() { return 42; }");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".to_string()),
                Token::OpenParen,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Constant("42".to_string()),
                Token::Semicolon,
                Token::CloseBrace
            ]
        );
    }
}

#[test]
fn test_complex_expression() {
    let result = lex("x = (a + b) * c - d");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x".to_string()),
                Token::Assignment,
                Token::OpenParen,
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Identifier("b".to_string()),
                Token::CloseParen,
                Token::Multiply,
                Token::Identifier("c".to_string()),
                Token::Negate,
                Token::Identifier("d".to_string())
            ]
        );
    }
}

#[test]
fn test_identifier_with_underscore() {
    let result = lex("_var var_name _123 var123");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("_var".to_string()),
                Token::Identifier("var_name".to_string()),
                Token::Identifier("_123".to_string()),
                Token::Identifier("var123".to_string())
            ]
        );
    }
}

#[test]
fn test_operator_precedence_tokens() {
    let result = lex("a << b + c & d");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::LeftShift,
                Token::Identifier("b".to_string()),
                Token::Plus,
                Token::Identifier("c".to_string()),
                Token::BitwiseAnd,
                Token::Identifier("d".to_string())
            ]
        );
    }
}

#[test]
fn test_invalid_character() {
    let result = lex("@");
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, "couldn't find any match");
    }
}

#[test]
fn test_mixed_valid_invalid() {
    let result = lex("int main @ return");
    assert!(result.is_err());
}

#[test]
fn test_longest_match_priority() {
    let result = lex("-- - <= <");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Decrement,
                Token::Negate,
                Token::LessThanOrEqual,
                Token::LessThan
            ]
        );
    }
}

// =============================================================================
// IF/ELSE KEYWORDS
// =============================================================================

#[test]
fn test_if_keyword() {
    let result = lex("if");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::If]);
    }
}

#[test]
fn test_else_keyword() {
    let result = lex("else");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Else]);
    }
}

#[test]
fn test_if_else_statement() {
    let result = lex("if (1) return 0; else return 1;");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::OpenParen,
                Token::Constant("1".to_string()),
                Token::CloseParen,
                Token::Return,
                Token::Constant("0".to_string()),
                Token::Semicolon,
                Token::Else,
                Token::Return,
                Token::Constant("1".to_string()),
                Token::Semicolon
            ]
        );
    }
}

#[test]
fn test_if_identifier_not_keyword() {
    // 'ifdef' should be identifier, not 'if' + 'def'
    let result = lex("ifdef");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::Identifier("ifdef".to_string())]);
    }
}

// =============================================================================
// TERNARY OPERATOR TOKENS
// =============================================================================

#[test]
fn test_question_mark() {
    let result = lex("?");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::QuestionMark]);
    }
}

#[test]
fn test_colon() {
    let result = lex(":");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(tokens, vec![Token::DoubleDot]);
    }
}

#[test]
fn test_ternary_expression() {
    let result = lex("a ? b : c");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::QuestionMark,
                Token::Identifier("b".to_string()),
                Token::DoubleDot,
                Token::Identifier("c".to_string())
            ]
        );
    }
}

#[test]
fn test_ternary_with_constants() {
    let result = lex("x ? 1 : 0");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x".to_string()),
                Token::QuestionMark,
                Token::Constant("1".to_string()),
                Token::DoubleDot,
                Token::Constant("0".to_string())
            ]
        );
    }
}

// =============================================================================
// CONSECUTIVE OPERATORS WITHOUT SPACES
// =============================================================================

#[test]
fn test_consecutive_and_no_space() {
    let result = lex("a&&b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::And,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_or_no_space() {
    let result = lex("a||b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::Or,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_less_equal_no_space() {
    let result = lex("a<=b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::LessThanOrEqual,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_greater_equal_no_space() {
    let result = lex("a>=b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::GreaterThanOrEqual,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_equal_no_space() {
    let result = lex("a==b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::Equal,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_not_equal_no_space() {
    let result = lex("a!=b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::NotEqual,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_consecutive_shift_no_space() {
    let result = lex("a<<b>>c");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::LeftShift,
                Token::Identifier("b".to_string()),
                Token::RightShift,
                Token::Identifier("c".to_string())
            ]
        );
    }
}

#[test]
fn test_triple_plus() {
    // a+++b should tokenize as a, ++, +, b or a, +, +, +, b depending on lexer
    // Since we have Decrement (--) but no Increment (++), this should be +, +, +
    let result = lex("a+++b");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        // Without ++ token, should be: a, +, +, +, b
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Plus,
                Token::Plus,
                Token::Identifier("b".to_string())
            ]
        );
    }
}

#[test]
fn test_complex_no_spaces() {
    let result = lex("a+b*c-d/e%f");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Identifier("b".to_string()),
                Token::Multiply,
                Token::Identifier("c".to_string()),
                Token::Negate,
                Token::Identifier("d".to_string()),
                Token::Divide,
                Token::Identifier("e".to_string()),
                Token::Remainder,
                Token::Identifier("f".to_string())
            ]
        );
    }
}

#[test]
fn test_ternary_no_spaces() {
    let result = lex("a?b:c");
    assert!(result.is_ok());
    if let Ok(tokens) = result {
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::QuestionMark,
                Token::Identifier("b".to_string()),
                Token::DoubleDot,
                Token::Identifier("c".to_string())
            ]
        );
    }
}

