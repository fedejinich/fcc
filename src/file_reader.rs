use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_path_buff_to_string(path_buf: &PathBuf) -> Option<Vec<char>> {
    let path = path_buf.to_str().unwrap();

    let mut code = String::new();
    let input = File::open(path); // todo(fedejinich) error handling

    if input.is_err() {
        return None;
    }

    // reads to 'code'
    input.unwrap().read_to_string(&mut code).unwrap();

    Some(code.chars().collect::<Vec<char>>())
}
