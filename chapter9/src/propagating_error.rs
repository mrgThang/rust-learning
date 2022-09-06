use std::fs::{self, File};
use std::io::{self, Read};

// Handle Propagating Errors traditionally
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? operator - a shortcut for Propagating Error
fn read_username_from_file_two() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Most ergonomic way
fn read_username_from_file_three() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// More example of ? operator
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    println!("{:?}", last_char_of_first_line(""));
}
