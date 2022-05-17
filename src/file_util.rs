use std::error::Error;
use std::io::Read;
use std::{fs::File, io::Write};

pub struct FileUtil;

impl FileUtil {
    pub fn new() -> FileUtil {
        FileUtil {}
    }

    pub fn write_assembly_file(
        &self,
        assembly_str: &str,
        file_name: &str,
    ) -> Result<(), impl Error> {
        println!("Writing assembly to this filepath {}", file_name);

        let mut file = File::create(file_name)?;

        write!(file, "{}", assembly_str)
    }

    pub fn read_path_buff_to_string(&self, path_buf: &str) -> Vec<char> {
        let mut code = String::new();
        let input = File::open(path_buf); // todo(fedejinich) error handling

        let result: Option<Vec<char>> = if input.is_err() {
            panic!("could't read .c file {}", path_buf);
        } else {
            // reads to 'code'
            input.unwrap().read_to_string(&mut code).unwrap();

            Some(code.chars().collect::<Vec<char>>())
        };

        result.unwrap()
    }
}
