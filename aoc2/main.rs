use aoc2::data_importer;
use aoc2::data_organizer::Game;

fn main() {
    // Get the txt file as a Vector of Strings
    let file_contents = data_importer::import_txt_as_vector("/home/aidan/Documents/learning_rust/advent_of_code/aoc2/src/aoc_doc2.txt");

    // Create a Vector to store all of the valid Game structs
    let mut all_game_vector: Vec<Game> = Vec::with_capacity(file_contents.len());
    let mut valid_only_game_vector: Vec<Game> = Vec::new();

    // Get the bag maximum
    let bag_maximum: Game = Game::get_bag_maximum();

    // Push each valid Game into the game_vector
    for line in file_contents.iter() {
        let line_game = Game::get_game_from_string(line);
        // println!("{:?}", (&line_game.game_num, &line_game.max_num_red, &line_game.max_num_green, &line_game.max_num_blue));

        if line_game.max_num_red <= bag_maximum.max_num_red && line_game.max_num_green <= bag_maximum.max_num_green && line_game.max_num_blue <= bag_maximum.max_num_blue {
            valid_only_game_vector.push(line_game.clone());
        }
        all_game_vector.push(line_game)
    }

    // Part 1: sum the game values
    let mut valid_game_sum: u32 = 0;

    for valid_game in valid_only_game_vector.iter() {
        valid_game_sum += valid_game.game_num as u32;
    }
    println!("Part 1: {}", valid_game_sum);

    // Part 2: calculate power: sum of each game's (max_red * max_blue * max_green)
    let mut power_sum: u32 = 0;

    for game in all_game_vector.iter() {
        power_sum += game.max_num_red * game.max_num_green * game.max_num_blue;
    }
    println!("Part 2: {}", power_sum);

}


