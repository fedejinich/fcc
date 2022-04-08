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
    Invalid(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::OpenBrace => "{".to_string(),
            Token::CloseBrace => "}".to_string(),
            Token::OpenParenthesis => "(".to_string(),
            Token::CloseParenthesis => ")".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::IntKeyword => "int".to_string(),
            Token::ReturnKeyword => "return".to_string(),
            Token::Identifier(id) => id.to_string(),
            Token::IntegerLiteral(num) => num.to_string(),
            Token::Invalid(invalid) => invalid.to_string(),
        }
    }
}
