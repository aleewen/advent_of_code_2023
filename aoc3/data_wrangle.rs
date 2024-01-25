
#[derive(Debug, Clone)]
pub struct Number {
    pub index: Vec<usize>,
    pub number: u16
}

#[derive(Debug, Clone)]
pub struct Indexer {
    pub line_num: usize,
    pub numbers: Vec<Number>,
    pub symbol_indexes: Vec<usize>,
    pub gear_indexes: Vec<usize>
}

impl Indexer {
    pub fn new(string: &str, line_num: usize) -> Indexer {
        // Create empty Vectors to store the index locations of numbers and symbols
        let mut numbers: Vec<Number> = Vec::new();
        let mut symbol_indexes: Vec<usize> = Vec::new();
        let mut gear_indexes: Vec<usize> = Vec::new();

        // Create an index value
        let mut idx: usize = 0;
        let mut character: char;

        //for (idx, character) in string.chars().enumerate() {
        while idx < string.len() {
            character = string.chars().nth(idx).unwrap();

            if character.is_digit(10) == true {
                // If the current character is a number, we need to extract and save that number and its index positions
                // Create an empty Vector to store the number characters and their indexes (capacity = 3 because the text file has no numbers larger than this)
                let mut number_string: String = String::with_capacity(3);
                let mut index_vector: Vec<usize> = Vec::with_capacity(3);

                // Push the current character and index into the list
                number_string.push(character);
                index_vector.push(idx);

                // If the index is already at the end of the string, then break
                if idx == string.len() - 1 {
                    break;
                }

                // Continue pushing until the number ends
                let mut is_number: bool = true;
                idx += 1;
                
                while is_number == true {
                    character = string.chars().nth(idx).unwrap();

                    if character.is_digit(10) == true {
                        is_number = true;
                        number_string.push(character);
                        index_vector.push(idx);
                        
                        if idx == string.len() - 1 {
                            break;
                        }
                        else {
                            idx += 1;
                        }
                    }
                    else {
                        is_number = false;
                    }
                }
                
                numbers.push(Number{index: index_vector, number: number_string.parse::<u16>().unwrap() });
            }
            if character.is_digit(10) == false && character != '.' {
                symbol_indexes.push(idx);

                if character == '*' {
                    gear_indexes.push(idx);
                }
            }

            idx += 1;
        }

        return Indexer {
            line_num: line_num,
            numbers: numbers,
            symbol_indexes: symbol_indexes,
            gear_indexes: gear_indexes
        };
    }

    pub fn get_numbers_adjacent_to_symbols(all_indexers: Vec<Indexer>) -> Vec<u16> {
        fn check_adjacent(number_vector: &[usize], index_vector: &[usize]) -> bool {
            for &vec_num in number_vector {
                for &idx_num in index_vector {
                    if vec_num == idx_num || vec_num == idx_num + 1 || vec_num == idx_num - 1 {
                        return true;
                    }
                }
            }
            return false;
        }

        // Create an empty vector to store the valid numbers
        let mut valid_numbers: Vec<u16> = Vec::new();

        for (line_num, indexer) in all_indexers.iter().enumerate() {
            // First, get all numbers that are side-by-side a symbol
            if indexer.symbol_indexes.len() > 0 {
                for number_struct in indexer.numbers.iter() {
                    if check_adjacent(&number_struct.index, &indexer.symbol_indexes) == true {
                        valid_numbers.push(number_struct.number);
                    }
                }
            }

            // Second, get all numbers that are below a symbol; compare with the previous Indexer
            if line_num > 0 && all_indexers[line_num-1].symbol_indexes.len() > 0 {
                for number_struct in indexer.numbers.iter() {
                    if check_adjacent(&number_struct.index, &all_indexers[line_num-1].symbol_indexes) == true {
                        valid_numbers.push(number_struct.number);
                    }
                }
            }

            // Third, get all numbers that are above a symbol; compare with the next Indexer
            if line_num + 1 < all_indexers.len() && all_indexers[line_num+1].symbol_indexes.len() > 0 {
                for number_struct in indexer.numbers.iter() {
                    if check_adjacent(&number_struct.index, &all_indexers[line_num+1].symbol_indexes) == true {
                        valid_numbers.push(number_struct.number);
                    }
                }
            }
        }

        return valid_numbers;
    }


    // For Part 2: ONCE AGAIN, if I knew what Part 2 entailed, I would have written it a bit differently... but let's just finish the challenge and move on
    pub fn get_gear_ratios(all_indexers: Vec<Indexer>) -> Vec<u32> {
        fn check_adjacent(value: &usize, vector: &[usize]) -> bool {
            for &idx in vector {
                if idx == *value || idx == *value + 1 || (*value > 0 && idx == *value - 1) {
                    return true;
                }
            }
            return false;
        }
        // Create an empty vector to store the gear ratios
        let mut gear_ratios: Vec<u32> = Vec::new();
        let mut gear_numbers: Vec<u32> = Vec::with_capacity(2);

        for (line_num, indexer) in all_indexers.iter().enumerate() {
            if indexer.gear_indexes.len() > 0 {
                for gear in indexer.gear_indexes.iter() {
                    // First, get all numbers that are side-by-side the gear
                    for number_struct in indexer.numbers.iter() {
                        if check_adjacent(gear, &number_struct.index) == true {
                            gear_numbers.push(number_struct.number as u32);
                        }
                    }

                    // Second, get all numbers that are above a gear; compare with the previous Indexer
                    if line_num > 0 && all_indexers[line_num-1].numbers.len() > 0 {
                        for number_struct in all_indexers[line_num-1].numbers.iter() {
                            if check_adjacent(gear, &number_struct.index) == true {
                                gear_numbers.push(number_struct.number as u32);
                            }
                        }
                    }

                    // Third, get all numbers that are below a gear; compare with the next Indexer
                    if line_num + 1 < all_indexers.len() && all_indexers[line_num+1].numbers.len() > 0 {
                        for number_struct in all_indexers[line_num+1].numbers.iter() {
                            if check_adjacent(gear, &number_struct.index) == true {
                                gear_numbers.push(number_struct.number as u32);
                            }
                        }
                    }
                    // println!("{:?}", gear_numbers);
                    if gear_numbers.len() == 2 {
                        let gear_ratio = gear_numbers.iter().product();
                        gear_ratios.push(gear_ratio);
                    }
                    
                    // Truncate the gear_numbers Vector
                    gear_numbers.clear();
                }
            }
        }
    return gear_ratios;
    }
}





