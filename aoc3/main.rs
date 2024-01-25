use aoc3::data_extract;
use aoc3::data_wrangle::Indexer;


fn main() {
    // Read in the text file contents
    let file_contents = data_extract::read_txt_to_vector("/home/aidan/Documents/learning_rust/advent_of_code/aoc3/src/aoc_doc3.txt");

    // Create a vector to hold each line as an Indexer struct
    let mut indexer_vector: Vec<Indexer> = Vec::with_capacity(file_contents.len());
    
    // Push each line as an Indexer into the vector
    for (line_num, line_string) in file_contents.iter().enumerate() {
        let indexer = Indexer::new(&line_string, line_num);
        println!("{:?}", &indexer);
        println!("");
        indexer_vector.push(indexer);
        
    }

    // FOR PART 2: If I knew what Part 2 was, I wouldn't have done it this way, but let's just finish the challenge and move onto the next
    let mut indexer_vector_clone: Vec<Indexer> = Vec::with_capacity(indexer_vector.len());

    for value in indexer_vector.iter() {
        indexer_vector_clone.push((*value).clone());
    }

    // Get all the numbers that are adjacent to a symbol
    let valid_numbers = Indexer::get_numbers_adjacent_to_symbols(indexer_vector);

    // println!("{:?}", &valid_numbers);
    // println!("");

    // Calculate the sum of all the numbers
    let sum_valid_numbers: u32 = valid_numbers.iter().map(|&num| num as u32).sum();

    println!("Part 1: {:?}", sum_valid_numbers);


    // Part 2: 
    // Get all the gear ratios
    let gear_ratios = Indexer::get_gear_ratios(indexer_vector_clone);

    // println!("{:?}", &gear_ratios);
    // println!("");

    // Calculate the sum of all the numbers
    let sum_gear_ratios: u32 = gear_ratios.iter().sum();

    println!("Part 2: {:?}", sum_gear_ratios);

}





