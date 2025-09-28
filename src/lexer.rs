use log::{debug, trace};
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(String),

    // keywords
    Int,
    Void,
    Return,

    // symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    // unary operators
    Complement,
    Negate, // this is also the minus operator but we don't disitinguish this in lexing stage
    Not,

    // ops
    Decrement,
    Assignment,

    // binary operators
    Plus,
    Multiply,
    Divide,
    Remainder,

    // bitwise (binary) operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // logical operators
    And,
    Or,

    // relational operators
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // if statement and conditional expressions
    If,
    Else,
    QuestionMark,
    DoubleDot,
}

pub fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    debug!("lexing code");

    if code.is_empty() {
        return Ok(vec![]);
    }

    code = code.trim_start();

    let mut tokens = vec![];
    while !code.is_empty() {
        let mut longest_match: TokenMatch = None;
        for matcher in token_matchers().iter() {
            if let Some(new_match) = matcher.match_longest(code, &longest_match) {
                longest_match = Some((matcher.token_builder, String::from(new_match.as_str())));
            }
        }

        let Some((constructor, value)) = longest_match else {
            return Err(String::from("couldn't find any match"));
        };

        let new_token = constructor(value.clone());

        trace!("token: {new_token:?}");

        tokens.push(new_token);

        if let Some(c) = code.strip_prefix(value.as_str()) {
            code = c.trim_start();
        }
    }

    Ok(tokens)
}

type TokenBuilder = fn(String) -> Token;
type TokenMatch = Option<(TokenBuilder, String)>;

fn token_matchers() -> Vec<TokenMatcher> {
    use Token::*;
    vec![
        TokenMatcher::new(build_identifier_or_keyword, r"^[a-zA-Z_]\w*\b"),
        TokenMatcher::new(Constant, r"^[0-9]+\b"),
        TokenMatcher::new(|_| OpenParen, r"^\("),
        TokenMatcher::new(|_| CloseParen, r"^\)"),
        TokenMatcher::new(|_| OpenBrace, r"^\{"),
        TokenMatcher::new(|_| CloseBrace, r"^\}"),
        TokenMatcher::new(|_| Semicolon, r"^;"),
        TokenMatcher::new(|_| Complement, r"^\~"),
        TokenMatcher::new(|_| Negate, r"^\-"),
        TokenMatcher::new(|_| Decrement, r"^\--"),
        TokenMatcher::new(|_| Plus, r"^\+"),
        TokenMatcher::new(|_| Multiply, r"^\*"),
        TokenMatcher::new(|_| Divide, r"^\/"),
        TokenMatcher::new(|_| Remainder, r"^\%"),
        TokenMatcher::new(|_| BitwiseAnd, r"^\&"),
        TokenMatcher::new(|_| BitwiseOr, r"^\|"),
        TokenMatcher::new(|_| BitwiseXor, r"^\^"),
        TokenMatcher::new(|_| LeftShift, r"^<<"),
        TokenMatcher::new(|_| RightShift, r"^>>"),
        TokenMatcher::new(|_| Not, r"^!"),
        TokenMatcher::new(|_| And, r"^&&"),
        TokenMatcher::new(|_| Or, r"^\|\|"),
        TokenMatcher::new(|_| Equal, r"^=="),
        TokenMatcher::new(|_| NotEqual, r"^!="),
        TokenMatcher::new(|_| LessThan, r"^<"),
        TokenMatcher::new(|_| LessThanOrEqual, r"^<="),
        TokenMatcher::new(|_| GreaterThan, r"^>"),
        TokenMatcher::new(|_| GreaterThanOrEqual, r"^>="),
        TokenMatcher::new(|_| Assignment, r"^="),
        TokenMatcher::new(|_| If, r"^if"),
        TokenMatcher::new(|_| Else, r"^else"),
        TokenMatcher::new(|_| QuestionMark, r"^\?"),
        TokenMatcher::new(|_| DoubleDot, r"^\:"),
    ]
}

fn build_identifier_or_keyword(s: String) -> Token {
    match s.as_str() {
        "int" => Token::Int,
        "void" => Token::Void,
        "return" => Token::Return,
        _ => Token::Identifier(s),
    }
}

#[derive(Clone)]
pub struct TokenMatcher {
    pub regex: &'static str,
    pub token_builder: TokenBuilder,
}

impl TokenMatcher {
    fn new(token_builder: fn(String) -> Token, regex: &'static str) -> Self {
        Self {
            regex,
            token_builder,
        }
    }

    fn match_longest<'a>(
        &self,
        code: &'a str,
        longest_match: &TokenMatch,
    ) -> Option<regex::Match<'a>> {
        let Ok(regex) = Regex::new(self.regex) else {
            panic!("couldn't create regex");
        };
        let m = regex.find(code)?;

        debug!("match: {m:?}");

        if longest_match.is_none() {
            return Some(m);
        }

        let Some(longest_match_value) = longest_match.clone() else {
            panic!("couldn't find longest_match_value");
        };
        let longest_match_value = longest_match_value.1;

        debug!("match_value: {longest_match_value:?}");

        // match if longer than longest match
        if m.len() > longest_match_value.len() {
            return Some(m);
        }

        None
    }
}

pub fn binary_operators() -> Vec<Token> {
    vec![
        Token::Plus,
        Token::Negate,
        Token::Multiply,
        Token::Divide,
        Token::Remainder,
        // bitwise operators
        Token::BitwiseAnd,
        Token::BitwiseOr,
        Token::BitwiseXor,
        Token::LeftShift,
        Token::RightShift,
        // logical operators
        Token::And,
        Token::Or,
        // relational operators
        Token::Equal,
        Token::NotEqual,
        Token::GreaterThan,
        Token::LessThan,
        Token::GreaterThanOrEqual,
        Token::LessThanOrEqual,
        // Assignment
        Token::Assignment,
        // conditional operators
        Token::QuestionMark,
    ]
}

#[allow(dead_code)]
pub fn unary_operators() -> Vec<Token> {
    vec![Token::Complement, Token::Negate, Token::Not]
}

mod tests {
    #[allow(unused_imports)]
    use crate::lexer::{binary_operators, unary_operators};

    #[test]
    fn binary_operator_count_test() {
        let bin_op = binary_operators();
        assert_eq!(bin_op.len(), 19);
    }

    #[test]
    fn unary_operator_count_test() {
        let un_op = unary_operators();
        assert_eq!(un_op.len(), 3);
    }
}
