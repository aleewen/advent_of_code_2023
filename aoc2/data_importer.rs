use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn import_txt_as_vector(file_path: &str) -> Vec<String> {
    // Set the file path as a Path struct
    let file_path_struct = Path::new(file_path);

    // Open the file; unwrap as the entire program will break if it cannot be read 
    let file = File::open(file_path_struct).unwrap();

    // Wrap the file in a buffered reader
    let buffered_reader = BufReader::new(file);

    // Instantiate a vector to hold each line as a String
    let mut file_vector: Vec<String> = Vec::new();

    // Push each line into the vector as a String
    for line in buffered_reader.lines() {
        // Attempt to read it; line is a Result
        let line_temp = line.unwrap();

        // Push the String into the vector
        file_vector.push(line_temp);
    }

    return file_vector;
}


