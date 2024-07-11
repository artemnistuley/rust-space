use std::io::Read;
use std::{fs, io};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = username_file_result?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    
    Ok(username)
}

fn main() {
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
