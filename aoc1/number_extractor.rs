pub fn convert_word_to_number(word: &str) -> String {
    let number_words = [
        ("one", '1'), ("two", '2'), ("three", '3'),
        ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'),
        ("eight", '8'), ("nine", '9'),
    ];
    let mut converted_word = String::new();
    let mut replaced: bool;
    
    for i in 0..word.len() {
        replaced = false;
        for (number_word, actual_number) in number_words.iter() {
            if word[i..].starts_with(number_word) {
                replaced = true;
                converted_word.push(actual_number.clone());
            }
        }
        if replaced == false {
            converted_word.push(word.chars().nth(i).unwrap());
        }
    }

    return converted_word;
}

pub fn keep_only_numbers(my_string: &String) -> String {
    let mut my_number_string: String = String::new();

    for character in my_string.chars() {
        let is_digit_result = character.is_digit(10);

        if is_digit_result == true {
            my_number_string.push(character)
        }
    }

    return my_number_string;
}

pub fn get_first_and_last_chars(my_string: String) -> String {
    let mut my_two_chars: String = String::with_capacity(2);
    let my_string_length = my_string.len();

    let first_char = my_string.chars().nth(0).unwrap();
    let last_char = my_string.chars().nth(my_string_length-1).unwrap();

    my_two_chars.push(first_char);
    my_two_chars.push(last_char);

    return my_two_chars;
}



