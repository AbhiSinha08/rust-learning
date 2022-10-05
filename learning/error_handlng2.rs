use std::fs::File;
use std::io::{self, Read};

fn main() {
    let res = read_from_file("new.txt");
    let res = res.expect("Error returned from function");
    println!("{}", res);

    let res = read_from_file2("new.txt");
    let res = res.expect("Error returned from function");
    println!("{}", res);
}

fn read_from_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut s)?;

    Ok(s)
}
