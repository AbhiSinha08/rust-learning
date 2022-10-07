use std::fs::File;
use std::io::{ErrorKind, Read};

pub fn read_file(file_path: &str) -> Result<String, &'static str> {
    let mut contents = String::new();
    let mut f = match File::open(file_path){
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                return Err("The specified file is not found.");
            },
            _ => {
                return Err("Unable to open the file to read.");
            }
        }
    };
    match f.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => {
            return Err("Cannot read the file because of unknown characters.");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_read_file() {
        let filepath = String::from("file_that_doesn't_exist");
        read_file(&filepath).unwrap();
    }
}