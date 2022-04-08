use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(path: &str) -> Option<Vec<char>> {
    let mut code = String::new();
    let input = File::open(&path);

    if input.is_err() {
        return None;
    }

    // reads to 'code'
    input.unwrap().read_to_string(&mut code).unwrap();

    Some(code.chars().collect::<Vec<char>>())
}
