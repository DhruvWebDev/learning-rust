
use std::fs;
use std::io;

pub fn error_handling() {
    let path = "main.rsss"; // Specify the file path

    let res: Result<String, io::Error> = fs::read_to_string(path);

    match res {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error: {}", err),
    }
}

