use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


pub fn read_file(file_name: &str) -> Vec<String> {
    // Create a path to the desired file
    let file_path = Path::new(file_name);

    // Open the file at the path
    let file = File::open(file_path).unwrap();

    // Wrap the file in a buffered reader
    let buff_reader = BufReader::new(file);

    // Instantiate a vector to hold each line as a String
    let mut file_vector: Vec<String> = Vec::new();

    // Push each line into the vector as a String
    for line in buff_reader.lines() {
        // Attempt to read it; line is a Result
        let line_temp = line.unwrap();

        // Push the String into the vector
        file_vector.push(line_temp);
    }

    return file_vector;
}










