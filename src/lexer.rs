use regex::Regex;
use log::{info, debug, trace};

fn build_token_rules() -> Vec<(fn(String) -> Token, &'static str)> {
    vec![
        (|s| Token::Identifier(s), r"^[a-zA-Z_]\w*\b"),
        (|s| Token::Constant(s), r"^[0-9]+\b"),
        (|s| Token::Int(s), r"^int\b"),
        (|_| Token::Void, r"^void\b"),
        (|_| Token::Return, r"^return\b"),
        (|_| Token::OpenParen, r"^\("),
        (|_| Token::CloseParen, r"^\)"),
        (|_| Token::OpenBrace, r"^\{"),
        (|_| Token::CloseBrace, r"^\}"),
        (|_| Token::Semicolon, r"^;"),
    ]
}

#[derive(Debug)]
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

type TokenDef = (fn(String) -> Token, String);

pub fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    debug!("lexing code");

    if code.is_empty() {
        return Ok(vec![]);
    }

    code = code.trim_start();

    let mut tokens = vec![];
    while !code.is_empty() {
        // find longest match at start of input for any token rule
        let mut longest_match: Option<TokenDef> = None;
        for (constructor, regex) in build_token_rules().iter() {
            if let Some(new_match) = Regex::new(regex).unwrap().find(code) {
                if let Some((_, ref longest_match_value)) = longest_match {
                    if new_match.len() > longest_match_value.len() {
                        longest_match = Some((*constructor, String::from(new_match.as_str())));
                    }
                } else if longest_match.is_none() {
                    longest_match = Some((*constructor, String::from(new_match.as_str())));
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
