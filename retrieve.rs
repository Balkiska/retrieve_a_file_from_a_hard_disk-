use std::fs;
use std::io;
use std::path::Path;

// Define a function to read file content
// This function returns a Result containing either the file content as a String or an io::Error
fn read_file(path: &str) -> Result<String, io::Error> {
    // Check if the file exists
    if Path::new(path).exists() {
        // If the file exists, read and return its content
        fs::read_to_string(path)
    } else {
        // If the file does not exist, return an error
        Err(io::Error::new(io::ErrorKind::NotFound, "The file does not exist !"))
    }
}

fn main() {
    // Specify the path to your file
    let file_path = "yourfile.txt";
    
    // Attempt to read the file and match the result
    match read_file(file_path) {
        Ok(content) => println!("{}", content), // If successful, print the content
        Err(e) => println!("Error reading the file: {}", e), // If an error occurs, print the error
    }
}
