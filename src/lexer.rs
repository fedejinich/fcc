use log::{debug, trace};
use regex::Regex;

#[derive(Clone)]
pub struct TokenRule {
    pub constructor: fn(String) -> Token,
    pub regex: &'static str,
}

impl TokenRule {
    fn new(constructor: fn(String) -> Token, regex: &'static str) -> Self {
        Self { constructor, regex }
    }

    fn matches<'a>(&self, code: &'a str) -> Option<regex::Match<'a>> {
        Regex::new(self.regex).unwrap().find(code)
    }
}

fn build_token_rules() -> Vec<TokenRule> {
    vec![
        TokenRule::new(|s| Token::Identifier(s), r"^[a-zA-Z_]\w*\b"),
        TokenRule::new(|s| Token::Constant(s), r"^[0-9]+\b"),
        TokenRule::new(|s| Token::Int(s), r"^int\b"),
        TokenRule::new(|_| Token::Void, r"^void\b"),
        TokenRule::new(|_| Token::Return, r"^return\b"),
        TokenRule::new(|_| Token::OpenParen, r"^\("),
        TokenRule::new(|_| Token::CloseParen, r"^\)"),
        TokenRule::new(|_| Token::OpenBrace, r"^\{"),
        TokenRule::new(|_| Token::CloseBrace, r"^\}"),
        TokenRule::new(|_| Token::Semicolon, r"^;"),
    ]
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Int(String),
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

pub fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    debug!("lexing code");

    if code.is_empty() {
        return Ok(vec![]);
    }

    code = code.trim_start();

    let mut tokens = vec![];
    while !code.is_empty() {
        // find longest match at start of input for any token rule
        let mut longest_match: Option<(fn(String) -> Token, String)> = None;
        for rule in build_token_rules().iter() {
            if let Some(new_match) = rule.matches(code) {
                if let Some((_, ref longest_match_value)) = longest_match {
                    if new_match.len() > longest_match_value.len() {
                        longest_match = Some((rule.constructor, String::from(new_match.as_str())));
                    }
                } else if longest_match.is_none() {
                    longest_match = Some((rule.constructor, String::from(new_match.as_str())));
                }
            }
        }

        if longest_match.is_none() {
            return Err(String::from("couldn't find longest match"));
        }

        let (constructor, value) = longest_match.unwrap();
        let new_token = constructor(value.clone());

        trace!("token: {:?}", new_token);

        tokens.push(new_token);

        code = code.strip_prefix(value.as_str()).unwrap().trim_start();
    }

    Ok(tokens)
}
