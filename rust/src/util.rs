use std::{fs::File, io::Read, path::Path};

pub fn get_file_content(path: &str) -> String {
    let mut result = String::new();
    let path = Path::new(path);

    let mut file = File::open(path).expect("Unable to open the file");
    file.read_to_string(&mut result)
        .expect("Unable to read the contents of the string");

    result
}
