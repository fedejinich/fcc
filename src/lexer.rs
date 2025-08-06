use regex::Regex;

enum TokenId {
    Identifier,
    Constant,
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

const TOKEN_RULES: [(TokenId, &str); 10] = [
    (TokenId::Identifier, r"^[a-zA-Z_]\w*\b"),
    (TokenId::Constant, r"^[0-9]+\b"),
    (TokenId::Int, r"^int\b"),
    (TokenId::Void, r"^void\b"),
    (TokenId::Return, r"^return\b"),
    (TokenId::OpenParen, r"^\("),
    (TokenId::CloseParen, r"^\)"),
    (TokenId::OpenBrace, r"^\{"),
    (TokenId::CloseBrace, r"^\}"),
    (TokenId::Semicolon, r"^;"),
];

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

pub fn lex(code: &str) -> Result<Vec<Token>, String> {
    println!("lexing code");

    if code.is_empty() {
        return Ok(vec![]);
    }

    let mut code = code.trim_start();

    let mut tokens = vec![];
    while !code.is_empty() {
        // find longest match at start of input for any token rule
        let mut longest_match: Option<(&TokenId, &str)> = None;
        for (token_id, regex) in TOKEN_RULES.iter() {
            if let Some(new_match) = Regex::new(regex).unwrap().find(code) {
                if let Some((_, longest_match_value)) = longest_match {
                    if new_match.len() > longest_match_value.len() {
                        longest_match = Some((token_id, new_match.as_str()));
                    }
                }
            }
        }

        if longest_match.is_none() {
            return Err(String::from("couldn't find longest match"));
        }

        let (token_id, value) = longest_match.unwrap();
        code = code.strip_prefix(value).unwrap();

        let new_token = match_token(token_id, value);

        println!("token: {:?}", new_token);

        tokens.push(new_token);
    }

    Ok(tokens)
}

fn match_token(token_id: &TokenId, m: &str) -> Token {
    match *token_id {
        TokenId::Identifier => Token::Identifier(m.to_string()),
        TokenId::Constant => Token::Constant(m.to_string()),
        TokenId::Int => Token::Int(m.to_string()),
        TokenId::Void => Token::Void,
        TokenId::Return => Token::Return,
        TokenId::OpenParen => Token::OpenParen,
        TokenId::CloseParen => Token::CloseParen,
        TokenId::OpenBrace => Token::OpenBrace,
        TokenId::CloseBrace => Token::CloseBrace,
        TokenId::Semicolon => Token::Semicolon,
    }
}
