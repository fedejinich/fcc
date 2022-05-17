use std::fs::File;
use std::io::Write;

pub fn _emit(assembly_str: &str, file_name: &str) {
    let mut file = match File::create(file_name) {
        Ok(it) => it,
        Err(_) => panic!("couldn't emit assembly file"),
    };

    match write!(file, "{}", assembly_str) {
        Ok(_) => assembly_str,
        Err(_) => panic!("couldn't write"),
    };
}
