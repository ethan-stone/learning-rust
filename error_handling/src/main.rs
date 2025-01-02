use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username = read_username_from_file().expect("Unable to read username");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // ? means to propagate the error
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
