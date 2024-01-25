use aoc1::data_importer;
use aoc1::number_extractor;


fn main() {
    // Get the contents of the file as a Vector of Strings
    let document = data_importer::read_file("/home/aidan/Documents/learning_rust/advent_of_code/aoc1/src/aoc_doc1.txt");

    // Create a new vector to store the integers of each line; pre-instantiate the length of the vector
    let mut int_vec_pt1: Vec<u32> = Vec::with_capacity(document.len());
    let mut int_vec_pt2: Vec<u32> = Vec::with_capacity(document.len());

    // Traverse each line
    for line in document.iter() {
        // For Part 2, convert any numbers written as words to actual numbers
        let converted_number = number_extractor::convert_word_to_number(line);

        // Get only the numbers in each line
        let number_string_pt1 = number_extractor::keep_only_numbers(line);
        let number_string_pt2 = number_extractor::keep_only_numbers(&converted_number);
        

        // Keep only the first and last character of each line
        let first_and_last_chars_pt1 = number_extractor::get_first_and_last_chars(number_string_pt1);
        let first_and_last_chars_pt2 = number_extractor::get_first_and_last_chars(number_string_pt2);

        // Cast the string as a u32 and push into the vector
        int_vec_pt1.push(first_and_last_chars_pt1.parse::<u32>().unwrap());
        int_vec_pt2.push(first_and_last_chars_pt2.parse::<u32>().unwrap());
    }

    // Sum each item in the vector
    let aoc_answer_pt1: u32 = int_vec_pt1.iter().sum();
    println!("Part 1: {}", aoc_answer_pt1);

    let aoc_answer_pt2: u32 = int_vec_pt2.iter().sum();
    println!("Part 2: {}", aoc_answer_pt2);

}




