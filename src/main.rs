use std::collections::HashMap;
use std::{io, vec};
use std::ptr::null;

fn main() {
    let mut input:String = String::new();
    // Reads user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Trims input string
    let input_string = { input.trim() };
    // Passes input to single char function
    let single_char_string = single_char(&input);
    // Creates a frequency table
    let mut letter_frequency = HashMap::new();
    let mut char_binary_codes:HashMap<String, Vec<char>> = HashMap::new();
    for char in &single_char_string{
        // Adds each letter as a key to the frequency table
        letter_frequency.insert(char, 0);
    }

    for char in &single_char_string{
        for letter in input_string.chars(){
            if letter == *char {
                *letter_frequency.get_mut(&letter).unwrap() += 1;
            }
        }
    }
    let mut letter_frequency_list:Vec<Vec<String>> = Vec::new();
    for (key,value) in &letter_frequency {
        let key = key.to_string();
        let value = value.to_string();
        let key_value_vec = vec![key, value];
        letter_frequency_list.push(key_value_vec);
    }
    let sorted_frequency = order_by_value_list(letter_frequency_list);
    let mut huffman_list:Vec<Vec<String>> = sorted_frequency.clone();
    while huffman_list.len() > 1 {
        let huffman_len = huffman_list.len();
        let mut key1 = huffman_list[huffman_len -1][0].trim();
        let mut key2 = huffman_list[huffman_len-2][0].trim();
        let value1 = huffman_list[huffman_len-1][1].trim().parse::<i32>().unwrap();
        let value2 = huffman_list[huffman_len-2][1].trim().parse::<i32>().unwrap();
        let weight = value1 + value2;
        let weight = weight.to_string();
        if key1 == ""{
            key1 = "\u{2423}";
        }
        if key2 == ""{
            key2 = "\u{2423}";
        }
        let key1chars:Vec<char> = key1.chars().collect();
        for char in key1chars{
            if char_binary_codes.contains_key(&char.to_string()){
                let mut bc = char_binary_codes[&char.to_string()].clone();
                bc.reverse();
                bc.push('0');
                bc.reverse();
                char_binary_codes.insert(char.to_string(), bc);
            } else {
                char_binary_codes.insert(char.to_string(), vec!['0']);
            }

        }
        let key2chars:Vec<char> = key2.chars().collect();
        for char in key2chars{
            if char_binary_codes.contains_key(&char.to_string()){
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
        huffman_list[huffman_len-2] = key_weight_list;
        huffman_list.remove(huffman_len - 1);
        huffman_list = order_by_value_list(huffman_list);
    }
    let mut binary_codes:HashMap<char, u128> = HashMap::new();
    for (key, value) in char_binary_codes{
        let key_as_char = key.parse::<char>().unwrap();
        let mut value_as_string = String::new();
        let mut value_as_binary:String = "0b";
        for char in value {
            value_as_string.push(char);
        }
        for number in value_as_string.chars(){
            value_as_binary.push(number);
        }
        let value_as_binary:u128 = value_as_binary.parse::<u128>().unwrap();
        binary_codes.insert(key_as_char, value_as_binary);
    }
    for (key, value) in binary_codes{
        println!("{} {}", key, value);
    }
}

fn single_char(og_string:&String) -> Vec<char>{
    // Takes out whitespace
    let og_string = og_string.trim();
    // Makes a list
    let list_chars = og_string.chars().collect::<Vec<char>>();
    let mut list_clone = list_chars.clone();
    // Makes each char only returns once
    for element in list_chars.iter(){
        if list_chars.contains(element){
            // Removes all instances of element
            list_clone.retain(|x| *x != *element);
            // Adds element back (but once)
            list_clone.push(*element);

        }

    }
    list_clone

}

fn order_by_value_list(list:Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut list_mut = list.clone();
    list_mut.sort_by(|a,b|a[1].trim().parse::<i32>().unwrap().cmp(&b[1].trim().parse::<i32>().unwrap()));
    list_mut.reverse();
    list_mut
}