use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_txt_to_vector(file_path: &str) -> Vec<String> {
    // Write the file_path as a Path struct
    let path_struct = Path::new(file_path);

    // Open the file to read-only
    let file = File::open(path_struct).unwrap();

    // Wrap the file in a buffered reader
    let buffered_reader = BufReader::new(file);

    // Create an empty Vector to hold each line of txt as a String
    let mut file_contents: Vec<String> = Vec::new();

    // Push each line into the Vector
    for line in buffered_reader.lines() {
        // Unwrap the line
        let line_string = line.unwrap();
        file_contents.push(line_string);
    }

    return file_contents;
}




