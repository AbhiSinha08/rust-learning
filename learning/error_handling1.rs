use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("new.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("File not found. Creating...");
                match File::create("new.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Error creating the file {:?}", e),
                }
            },
            _ => panic!("Error Opening the file"),
        }
    };

    let f = File::open("new.txt").unwrap();
    let f = File::open("new.txt").expect("File not found");
    let f = File::open("new.txt")?;

    Ok(())
}