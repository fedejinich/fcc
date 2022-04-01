mod lexer;
mod file_reader;

fn main() {
    // let code = file_reader::read_file_to_string("return_2.c").unwrap();
    let code = file_reader::read_file_to_string("tests/resources/stage_1/invalid/wrong_case.c").unwrap();

    let token_vec = lexer::lex(code.as_slice(), Vec::new());

    for t in token_vec.iter() {
        println!("{:?}", t);
    }
}