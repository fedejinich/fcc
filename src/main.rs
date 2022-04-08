use token::Token;

mod ast;
mod file_reader;
mod lexer;
mod parser;
mod token;

fn main() {
    // let code = file_reader::read_file_to_string("return_2.c").unwrap();
    let code =
        file_reader::read_file_to_string("tests/resources/stage_1/invalid/wrong_case.c").unwrap();

    let token_vec = lexer::lex(code.as_slice(), Vec::new());

    for t in token_vec.iter() {
        println!("{:?}", t);
    }

    parser::parse(vec![
        Token::IntKeyword,
        Token::Identifier(String::from("main")),
        Token::OpenParenthesis,
        Token::CloseParenthesis,
        Token::OpenBrace,
        Token::ReturnKeyword,
        Token::IntegerLiteral(2),
        Token::Semicolon,
        Token::CloseBrace,
    ]);
}
