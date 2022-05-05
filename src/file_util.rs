use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_path_buff_to_string(path_buf: &PathBuf) -> Vec<char> {
    let path = path_buf.to_str().unwrap();

    let mut code = String::new();
    let input = File::open(path); // todo(fedejinich) error handling

    let result: Option<Vec<char>> = if input.is_err() {
        panic!("could't read .c file {:?}", path_buf);
    } else {
        // reads to 'code'
        input.unwrap().read_to_string(&mut code).unwrap();

        Some(code.chars().collect::<Vec<char>>())
    };

    result.unwrap()
}
