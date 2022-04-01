use regex::Regex;
use std::collections::HashMap;

const KEYWORD_TOKEN: [(&str, Token); 2] = [
    ("return", Token::ReturnKeyword),
    ("int", Token::IntKeyword)
];

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier(String),
    IntegerLiteral(u32),
    Invalid(String)
}

pub fn lex(code_program: &[char], tokens: Vec<Token>) -> Vec<Token> {
    let result = match  code_program {
        [] => Vec::new(), // base case 
        [' ', tail @ ..] | ['\n', tail @ ..] => lex(tail, tokens).to_owned(), // skips white spaces and end lines
        ['{', tail @ ..] => add(Token::OpenBrace, lex(tail, tokens)).to_owned(),
        ['}', tail @ ..] => add(Token::CloseBrace, lex(tail, tokens)).to_owned(),
        ['(', tail @ ..] => add(Token::OpenParenthesis, lex(tail, tokens)).to_owned(),
        [')', tail @ ..] => add(Token::CloseParenthesis, lex(tail, tokens)).to_owned(),
        [';', tail @ ..] => add(Token::Semicolon, lex(tail, tokens)).to_owned(),
        code => { 
            let (token, end_index) = identifier_or_num(code.to_vec().into_iter().collect());
        
            add(token, lex(&code[end_index..], tokens)).to_owned()
        }
    };

    result
}

fn add(token: Token, mut token_list: Vec<Token>) -> Vec<Token> {
    token_list.insert(0, token);
    token_list
}

fn identifier_or_num(code: String) -> (Token, usize) {
    let is_alpha_regex = Regex::new(r"[a-zA-Z]\w*").unwrap();
    // let is_alpha_regex = Regex::new(r"[[:alpha:]]\w*").unwrap(); this succeeds,
    // let is_alpha_regex = Regex::new(r"[:alpha:]\w*").unwrap(); but then this fails, why?
    let find_alpha_result = is_alpha_regex.find(code.as_str());

    if find_alpha_result.is_some() {
        let result = find_alpha_result.unwrap();
        let start = result.start();
        let end = result.end();
        
        let identified = &code[start..end];
        
        return (keyword_or_identifier_token(identified), find_alpha_result.unwrap().end());
    }

    let is_num_regex = Regex::new(r"[0-9]+").unwrap();
    let find_num_result = is_num_regex.find(code.as_str());

    if find_num_result.is_some() {
        let result = find_num_result.unwrap();
        let start = result.start();
        let end = result.end();

        let identified = &code[start..end];
        let identified_u32 = identified.parse::<u32>().unwrap();

        return (Token::IntegerLiteral(identified_u32), end);
    }

    panic!("should not reach here");
}

fn keyword_or_identifier_token(identified: &str) -> Token {
    let keyword_token_map: HashMap<&str, Token> = HashMap::from(KEYWORD_TOKEN);
    
    if is_keyword(identified) {
        return keyword_token_map.get(identified).unwrap().to_owned();    
    } else {
        // making it case sensitive
        let contains_keyword = (&keyword_token_map).iter()
            .map(|keyword_token| keyword_token.0.to_owned())
            .any(|k| String::from(identified).to_lowercase().contains(k));

        if contains_keyword {
            return Token::Invalid(String::from(identified));
            // panic!("found invalid keyword: {}", String::from(identified));
        }
    }
    
    Token::Identifier(String::from(identified))
}

fn is_keyword(word: &str) -> bool {
    let keyword_token_map: HashMap<&str, Token> = HashMap::from(KEYWORD_TOKEN);
    
    for c in word.chars() {
        if c.is_uppercase() {
            return false
        }
    }

    keyword_token_map.get(word).is_some()
}