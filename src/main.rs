use std::collections::HashMap;
use std::{io, vec};
fn main(){
    let mut chosen_operation = false;
    let mut operation = String::new();
    while chosen_operation == false {
        println!("Would you like to encode or decode?");
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        if &operation.trim().to_lowercase() == "encode"{
            chosen_operation = true;
            encode();
        } else if &operation.trim().to_lowercase() == "decode"{
            chosen_operation = true;
            decode();
        }
    }


}



fn encode() {
    let mut input: String = String::new();
    println!("What's the message that you would like to encode?");
    // Reads user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Trims input string
    let input_string = { input.trim() };
    // Passes input to single char function
    let single_char_string = single_char(&input);
    // Creates a frequency table
    let mut letter_frequency = HashMap::new();
    let mut char_binary_codes: HashMap<String, Vec<char>> = HashMap::new();
    for char in &single_char_string {
        // Adds each letter as a key to the frequency table
        letter_frequency.insert(char, 0);
    }

    for char in &single_char_string {
        for letter in input_string.chars() {
            if letter == *char {
                *letter_frequency.get_mut(&letter).unwrap() += 1;
            }
        }
    }
    let mut letter_frequency_list: Vec<Vec<String>> = Vec::new();
    for (key, value) in &letter_frequency {
        let key = key.to_string();
        let value = value.to_string();
        let key_value_vec = vec![key, value];
        letter_frequency_list.push(key_value_vec);
    }
    let sorted_frequency = order_by_value_list(letter_frequency_list);
    let mut huffman_list: Vec<Vec<String>> = sorted_frequency.clone();
    while huffman_list.len() > 1 {
        let huffman_len = huffman_list.len();
        let mut key1 = huffman_list[huffman_len - 1][0].trim();
        let mut key2 = huffman_list[huffman_len - 2][0].trim();
        let value1 = huffman_list[huffman_len - 1][1].trim().parse::<i32>().unwrap();
        let value2 = huffman_list[huffman_len - 2][1].trim().parse::<i32>().unwrap();
        let weight = value1 + value2;
        let weight = weight.to_string();
        if key1 == "" {
            key1 = "\u{2423}";
        }
        if key2 == "" {
            key2 = "\u{2423}";
        }
        let key1chars: Vec<char> = key1.chars().collect();
        for char in key1chars {
            if char_binary_codes.contains_key(&char.to_string()) {
                let mut bc = char_binary_codes[&char.to_string()].clone();
                bc.reverse();
                bc.push('0');
                bc.reverse();
                char_binary_codes.insert(char.to_string(), bc);
            } else {
                char_binary_codes.insert(char.to_string(), vec!['0']);
            }
        }
        let key2chars: Vec<char> = key2.chars().collect();
        for char in key2chars {
            if char_binary_codes.contains_key(&char.to_string()) {
                let mut bc = char_binary_codes[&char.to_string()].clone();
                bc.reverse();
                bc.push('1');
                bc.reverse();
                char_binary_codes.insert(char.to_string(), bc);
            } else {
                char_binary_codes.insert(char.to_string(), vec!['1']);
            }
        }
        let key = String::from(format!("{}{}", key1, key2));
        let key_weight_list = vec![key, weight];
        huffman_list[huffman_len - 2] = key_weight_list;
        huffman_list.remove(huffman_len - 1);
        huffman_list = order_by_value_list(huffman_list);
    }
    for (key, value) in &char_binary_codes {
        let val = value.iter().collect::<String>();
        println!("{} {}", key, val);
    }
    let mut final_string_list = vec![];
    for char in input_string.chars() {
        let mut char = char;
        if char == ' '{
            char = '\u{2423}';
        }
        let code = &char_binary_codes[&char.to_string()];
        final_string_list.push(code)
    }
    let mut final_string_list_strings = vec![];
    for char in final_string_list {
        let a = char.iter().collect::<String>();
        final_string_list_strings.push(a);
    }
    let final_string = final_string_list_strings.join("");
    println!("{} {}", final_string, final_string.len());
    let final_string_b10 = i128::from_str_radix(&final_string, 2).unwrap();
    let mut leading_zeros = 0;
    for char in final_string.chars() {
        if char == '0' {
            leading_zeros = leading_zeros + 1;
        } else {
            break;
        }
    }
    let mut l_z_str = String::new();
    for _i in 1..=leading_zeros {
        l_z_str.push('0');
    }
    println!("{}{}", l_z_str, final_string_b10);
}

fn single_char(og_string: &String) -> Vec<char> {
    // Takes out whitespace
    let og_string = og_string.trim();
    // Makes a list
    let list_chars = og_string.chars().collect::<Vec<char>>();
    let mut list_clone = list_chars.clone();
    // Makes each char only returns once
    for element in list_chars.iter() {
        if list_chars.contains(element) {
            // Removes all instances of element
            list_clone.retain(|x| *x != *element);
            // Adds element back (but once)
            list_clone.push(*element);
        }
    }
    list_clone
}

fn decode (){
    let mut unique_chars = String::new();
    let mut binary_codes: HashMap<String, char> = HashMap::new();
    println!("How many unique characters in your encoded message?");
    io::stdin().read_line(&mut unique_chars).expect("Failed to read line");
    for _i in 1..=unique_chars.trim().parse::<i128>().unwrap() {
        let mut char = String::new();
        let mut binary_code = String::new();
        println!("What's the character?");
        io::stdin().read_line(&mut char).expect("Failed to read line");
        if char.trim().chars().count() == 0 {
            char = '\u{2423}'.to_string();
        }
        let char = char.trim();
        println!("What's the binary code for {}?",char);
        io::stdin().read_line(&mut binary_code).expect("Failed to read line");
        let key = binary_code.trim().to_string();
        binary_codes.insert(key, char.parse::<char>().unwrap());
    }
    let mut base = String::new();
    let mut encoded = String::new();
    println!("Would you like to enter your encoded message in base 2 (2) or base 10 (10)");
    io::stdin().read_line(&mut base).expect("Failed to read line");
    println!("Please enter your encoded message in base {}", base.trim());
    io::stdin().read_line(&mut encoded).expect("Failed to read line");
    if base.trim() == "10"{
        let mut zero_string = String::from("");
        for char in encoded.chars(){
            if char == '0'{
                zero_string.push(char);
            } else {
                break;
            }
        }
        encoded = String::from(format!("{}{:b}",zero_string, encoded.trim().parse::<i128>().unwrap()));

    }
    let encoded = encoded.trim();
    let mut message = String::new();
    let mut current_string = String::new();
    for char in encoded.chars() {
        current_string.push(char);
        let current_ref = &current_string;
        if binary_codes.contains_key(current_ref) {
            let mut letter = binary_codes[&current_string];
            if letter == '\u{2423}'{
                letter = ' ';
            }
            message.push(letter);
            current_string = String::new();
        }
    }
    println!("{}", message);

}
fn order_by_value_list(list: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut list_mut = list.clone();
    list_mut.sort_by(|a, b| a[0].cmp(&b[0]));
    list_mut.sort_by(|a, b| a[1].trim().parse::<i32>().unwrap().cmp(&b[1].trim().parse::<i32>().unwrap()));
    list_mut.reverse();
    list_mut
}