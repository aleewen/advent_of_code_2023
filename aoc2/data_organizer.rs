#[derive(Clone)]
pub struct Game {
    pub game_num: u32,
    pub max_num_red: u32,
    pub max_num_green: u32,
    pub max_num_blue: u32,
}

impl Game {
    pub fn new(game_num: u32, max_num_red: u32, max_num_green: u32, max_num_blue: u32) -> Game {
        return Game {
            game_num: game_num,
            max_num_red: max_num_red,
            max_num_green: max_num_green,
            max_num_blue: max_num_blue,
        };
    }

    pub fn get_bag_maximum() -> Game {
        return Game {
            game_num: 0,
            max_num_red: 12,
            max_num_green: 13,
            max_num_blue: 14,
        };
    }

    pub fn get_game_from_string(line: &String) -> Game {
        fn extract_number(string_slice: &str) -> u32 {
            let mut number_string = String::with_capacity(3);

            for current_char in string_slice.chars() {
                if current_char.is_digit(10) {
                    number_string.push(current_char);
                }
            }

            return number_string.parse::<u32>().unwrap();
        }

        fn find_maximums(string_slice: &str) -> (u32, u32, u32) {   // returns (red, green, blue)
            let reveal_sequence: Vec<&str> = string_slice.split(';').collect();
            let mut each_reveal: Vec<&str> = Vec::new();

            let mut max_num_red: u32 = 0;
            let mut max_num_green: u32 = 0;
            let mut max_num_blue: u32 = 0;
            let mut num_temp: u32;

            for seq in reveal_sequence.iter() {
                let reveal_vec: Vec<&str> = seq.split(',').collect();

                for reveal in reveal_vec.iter() {
                    each_reveal.push(reveal);
                }
            }

            for reveal in each_reveal.iter() {
                if reveal.contains("red") {
                    num_temp = extract_number(reveal);
                    if num_temp > max_num_red {
                        max_num_red = num_temp.clone();
                    }
                }
                else if reveal.contains("green") {
                    num_temp = extract_number(reveal);
                    if num_temp > max_num_green {
                        max_num_green = num_temp.clone();
                    }
                }
                else if reveal.contains("blue") {
                    num_temp = extract_number(reveal);
                    if num_temp > max_num_blue {
                        max_num_blue = num_temp.clone();
                    }
                }

            }

            return (max_num_red, max_num_green, max_num_blue);

        }

        // Find the location of the colon (unwrap() because every line has one)
        let colon_loc = line.find(':').unwrap();
        let game_num = extract_number(&line[0..colon_loc]);

        let (max_num_red, max_num_green, max_num_blue) = find_maximums(&line[colon_loc+1..].trim());

        return Game {
            game_num: game_num as u32,
            max_num_red: max_num_red,
            max_num_green: max_num_green,
            max_num_blue: max_num_blue,
        };
    }
}
