use crate::token::Token;
use regex::Regex;
use std::collections::HashMap;

const KEYWORD_TOKEN: [(&str, Token); 2] =
    [("return", Token::ReturnKeyword), ("int", Token::IntKeyword)];

pub struct Lexer;

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn lex(&self, code_program: &[char], tokens: Vec<Token>) -> Vec<Token> {
        let result = match code_program {
            [] => Vec::new(), // base case
            ['{', tail @ ..] => self
                .add(Token::OpenBrace, self.lex(tail, tokens))
                .to_owned(),
            ['}', tail @ ..] => self
                .add(Token::CloseBrace, self.lex(tail, tokens))
                .to_owned(),
            ['(', tail @ ..] => self
                .add(Token::OpenParenthesis, self.lex(tail, tokens))
                .to_owned(),
            [')', tail @ ..] => self
                .add(Token::CloseParenthesis, self.lex(tail, tokens))
                .to_owned(),
            [';', tail @ ..] => self
                .add(Token::Semicolon, self.lex(tail, tokens))
                .to_owned(),
            [' ', tail @ ..] | ['\n', tail @ ..] => self.lex(tail, tokens).to_owned(), // skips white spaces and end lines
            code => {
                let (token, end_index) =
                    self.identifier_or_num(code.to_vec().into_iter().collect());

                self.add(token, self.lex(&code[end_index..], tokens))
                    .to_owned()
            }
        };

        result
    }

    pub fn add(&self, token: Token, mut token_list: Vec<Token>) -> Vec<Token> {
        token_list.insert(0, token);
        token_list
    }

    // todo(fedejinich) duplicated logic
    fn identifier_or_num(&self, code: String) -> (Token, usize) {
        let is_alpha_regex = Regex::new(r"[a-zA-Z]\w*").unwrap();
        let find_alpha_result = is_alpha_regex.find(code.as_str());

        if find_alpha_result.is_some() {
            let result = find_alpha_result.unwrap();
            let start = result.start();
            let end = result.end();

            let identified = &code[start..end];

            return (
                self.keyword_or_identifier_token(identified),
                find_alpha_result.unwrap().end(),
            );
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

    fn keyword_or_identifier_token(&self, identified: &str) -> Token {
        let keyword_token_map: HashMap<&str, Token> = HashMap::from(KEYWORD_TOKEN);

        if self.is_keyword(identified) {
            return keyword_token_map.get(identified).unwrap().to_owned();
        } else {
            // making it case sensitive
            let contains_keyword = (&keyword_token_map)
                .iter()
                .map(|keyword_token| keyword_token.0.to_owned())
                .any(|k| String::from(identified).to_lowercase().contains(k));

            if contains_keyword {
                return Token::Invalid(String::from(identified));
                // panic!("found invalid keyword: {}", String::from(identified));
            }
        }

        Token::Identifier(String::from(identified))
    }

    fn is_keyword(&self, word: &str) -> bool {
        let keyword_token_map: HashMap<&str, Token> = HashMap::from(KEYWORD_TOKEN);

        for c in word.chars() {
            if c.is_uppercase() {
                return false;
            }
        }

        keyword_token_map.get(word).is_some()
    }
}
