use fcc::lexer::{lex, Token};

#[test]
fn test_empty_input() {
    let result = lex("");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![]);
}

#[test]
fn test_whitespace_only() {
    let result = lex("   \t\n  ");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![]);
}

#[test]
fn test_single_identifier() {
    let result = lex("main");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![Token::Identifier("main".to_string())]);
}

#[test]
fn test_keywords() {
    let result = lex("int void return");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Int,
        Token::Void,
        Token::Return
    ]);
}

#[test]
fn test_constants() {
    let result = lex("42 0 123");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Constant("42".to_string()),
        Token::Constant("0".to_string()),
        Token::Constant("123".to_string())
    ]);
}

#[test]
fn test_symbols() {
    let result = lex("() {} ;");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,
        Token::CloseBrace,
        Token::Semicolon
    ]);
}

#[test]
fn test_unary_operators() {
    let result = lex("~ - !");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Complement,
        Token::Negate,
        Token::Not
    ]);
}

#[test]
fn test_binary_operators() {
    let result = lex("+ - * / %");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Plus,
        Token::Negate,
        Token::Multiply,
        Token::Divide,
        Token::Remainder
    ]);
}

#[test]
fn test_bitwise_operators() {
    let result = lex("& | ^ << >>");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::BitwiseAnd,
        Token::BitwiseOr,
        Token::BitwiseXor,
        Token::LeftShift,
        Token::RightShift
    ]);
}

#[test]
fn test_logical_operators() {
    let result = lex("&& ||");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::And,
        Token::Or
    ]);
}

#[test]
fn test_relational_operators() {
    let result = lex("== != < <= > >=");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Equal,
        Token::NotEqual,
        Token::LessThan,
        Token::LessThanOrEqual,
        Token::GreaterThan,
        Token::GreaterThanOrEqual
    ]);
}

#[test]
fn test_assignment_and_decrement() {
    let result = lex("= --");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Assignment,
        Token::Decrement
    ]);
}

#[test]
fn test_simple_function() {
    let result = lex("int main() { return 42; }");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Int,
        Token::Identifier("main".to_string()),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,
        Token::Return,
        Token::Constant("42".to_string()),
        Token::Semicolon,
        Token::CloseBrace
    ]);
}

#[test]
fn test_complex_expression() {
    let result = lex("x = (a + b) * c - d");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
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
    ]);
}

#[test]
fn test_identifier_with_underscore() {
    let result = lex("_var var_name _123 var123");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Identifier("_var".to_string()),
        Token::Identifier("var_name".to_string()),
        Token::Identifier("_123".to_string()),
        Token::Identifier("var123".to_string())
    ]);
}

#[test]
fn test_operator_precedence_tokens() {
    let result = lex("a << b + c & d");
    assert!(result.is_ok());
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Identifier("a".to_string()),
        Token::LeftShift,
        Token::Identifier("b".to_string()),
        Token::Plus,
        Token::Identifier("c".to_string()),
        Token::BitwiseAnd,
        Token::Identifier("d".to_string())
    ]);
}

#[test]
fn test_invalid_character() {
    let result = lex("@");
    assert!(result.is_err());
    assert_eq!(result.expect_err("Expected lexing to fail"), "couldn't find any match");
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
    assert_eq!(result.expect("Lexing should succeed"), vec![
        Token::Decrement,
        Token::Negate,
        Token::LessThanOrEqual,
        Token::LessThan
    ]);
}