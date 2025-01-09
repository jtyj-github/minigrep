use std::env;
use std::fs;

fn main() {
    // Converts iterator of args inro a collection (Vector)
    // Only accept valid Unicode, else panic
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Error reading file path");

    println!("With text:\n{contents}");
}
