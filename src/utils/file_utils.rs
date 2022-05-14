use std::fs;
use std::path;

pub fn create_dir(file_path: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let full_file_path = path::Path::new(file_path);
    let prefix = full_file_path.parent().unwrap();

    return match fs::create_dir_all(prefix) {
        Ok(_) => Ok(full_file_path.to_str().unwrap()),
        Err(e) => Err(Box::new(e)),
    };
}

pub fn create_and_write_file(
    file_path: &str,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    return match create_dir(file_path) {
        Ok(file_path) => match fs::write(file_path, content) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(e),
    };
}
