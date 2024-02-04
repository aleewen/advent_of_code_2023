use std::collections::HashMap;
use aoc4::data_extract::read_txt_to_vector;
use aoc4::card::Card;



fn main() {
    // Read in the text file as a vector of strings
    let txt_data = read_txt_to_vector("/home/aidan/Documents/learning_rust/advent_of_code/aoc4/src/aoc_doc4.txt");

    // Create a vector to store each card
    let mut card_vec: Vec<Card> = Vec::with_capacity(txt_data.len());

    for card_string in txt_data.iter() {
        let card = Card::new(card_string);

        card_vec.push(card);
    }

    // Create a variable to sum the card points
    let mut card_points: u32 = 0;

    for card in card_vec.iter() {
        card_points += card.calc_points()
    }

    println!("{}", card_points);

    // PART 2
    // Move the cards out of the vector and into a hashmap with card numbers as their keys
    let mut card_hashmap: HashMap<usize, Card> = HashMap::with_capacity(card_vec.len());

    for (i, card) in card_vec.into_iter().enumerate() {
        card_hashmap.insert(i+1, card);
    }

    // Calculate the number of copies each card has
    set_copies(&mut card_hashmap);
    
    // Calculate the number of scratchcards
    let mut total_cards: u32 = 0;

    for (_key, card) in card_hashmap.iter() {
        total_cards += card.copies.unwrap();
    }
    println!("{:?}", total_cards);

}

fn set_copies(card_hashmap: &mut HashMap<usize, Card>) {
    // Create a HashMap that stores the number of copies for each card
    let mut copy_update: HashMap<usize, u32> = HashMap::with_capacity(card_hashmap.len());

    // Copy the same keys as the card-hashmap but set keys to zero
    for (card_num, _card) in card_hashmap.iter() {
        copy_update.insert(*card_num, 1);
    }

    // Calculate the copies for each card
    for i in 1..card_hashmap.len() {
        let num_matches = card_hashmap.get(&i).unwrap().matching_nums.as_ref().unwrap().len();

        if num_matches > 0 {
            let upper_bound = i + num_matches;
            let copies = copy_update.get(&i).unwrap().clone();

            // Adjust the number of copies in future cards
            for j in i+1..=upper_bound {
                *copy_update.get_mut(&j).unwrap() += copies;
            }
        }
    }

    // Update the card-hashmap
    for (card_num, copies) in copy_update.into_iter() {
        if let Some(card) = card_hashmap.get_mut(&card_num) {
            card.copies = Some(copies);
        }
    }
}

