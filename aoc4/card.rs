
#[derive(Debug, Default)]
pub struct Card {
    pub winning_nums: Vec<i32>,
    pub given_nums: Vec<i32>,
    // Making these Option<T> types, since I don't know what Part 2 of the challenge will be
    pub matching_nums: Option<Vec<i32>>,
    pub copies: Option<u32>
}


impl Card {
    // Create a new Card struct given the a line from the text file
    pub fn new(txt_string: &str) -> Card {
        // Find the index of the colon
        let colon_idx = txt_string.find(':').expect("Could not find ':' in string.");

        // Remove all characters before the colon ':' and leading/trailing whitespace
        let nums_only = &txt_string[colon_idx+1..].trim();

        // Find the pip '|'
        let pipe_idx = nums_only.find('|').expect("Could not find '|' in string.");

        // Separate the winning numbers vs the given numbers
        let winning_nums_str = &nums_only[..pipe_idx].trim();
        let given_nums_str = &nums_only[pipe_idx+1..].trim();

        // Push each string into a vector
        let winning_nums: Vec<i32> = winning_nums_str
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
        
        let given_nums: Vec<i32> = given_nums_str
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

        // Create the card object
        let mut my_card = Card {
            winning_nums: winning_nums,
            given_nums: given_nums,
            ..Default::default()
        };

        // Set the matching numbers
        // (doing it this way in case Part 2 changes the way matching works)
        my_card.set_matching_nums();

        return my_card

    }

    fn set_matching_nums(&mut self) {
        // Create an empty vector to store all matching numbers in order
        let mut matching_nums: Vec<i32> = Vec::new();

        for my_num in self.given_nums.iter() {
            for winning_num in self.winning_nums.iter() {
                if my_num == winning_num {
                    matching_nums.push(*my_num);
                }
            }
        }

        self.matching_nums = Some(matching_nums);
    }

    pub fn set_copies(&mut self, copies: u32) {
        self.copies = Some(copies);
    }

    pub fn calc_points(&self) -> u32 {
        // Store the sum in a variable
        let mut card_points: u32 = 0;

        // Get the length of the matching numbers vector
        let num_matches: usize = match &self.matching_nums {
            Some(vec) => vec.len(),
            None => panic!("Matching nums weren't instantiated properly!")
        };

        // Calculate the number of points
        for idx in 0..num_matches {
            // One point for the first match
            if idx == 0 {
                card_points += 1;
            }
            // Double the points for each match after the first
            else {
                card_points *= 2;
            }
        }

        return card_points;
    }
}















