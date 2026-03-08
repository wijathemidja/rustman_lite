use std::collections::HashMap;
use std::{fs, io, vec};
use std::fs::File;
use std::io::Write;

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
    let mut hash_string = String::new();
    for (key, value) in &char_binary_codes {
        let val = value.iter().collect::<String>();
        println!("{} {}", key, val);
        let string = String::from(format!("{} {}\n", key, val));
        hash_string.push_str(&string);

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
    let final_string_clone = final_string.clone();
    input_to_file(final_string_clone,String::from("rustmanmsg"));
    input_to_file(hash_string,String::from("rustmanhash"));
    println!("{} {}", final_string, final_string.len());
    let mut final_string_int = vec![];
    for num in final_string.chars() {
        final_string_int.push(num.to_string().parse::<u8>().unwrap());
    }
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
    println!("Would you like to enter hash manually (m) or read hash file (r) ?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    if choice.trim() == "m"{
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
    } else {
        let hash_as_string = fs::read_to_string("rustmanhash.txt").expect("Failed to read rustmanhash.txt");
        for lines in hash_as_string.lines() {
            let list_of_chars = lines.chars().collect::<Vec<char>>();
            let letter = list_of_chars[0];
            let mut bc = String::new();
            for char in 2..list_of_chars.len(){
                bc.push(list_of_chars[char]);
            }
            binary_codes.insert(bc, letter);
        }
    }
    let mut base = String::new();
    let mut encoded = String::new();
    println!("Would you like to enter your encoded message in base 2 (2) or read from file (r)");
    io::stdin().read_line(&mut base).expect("Failed to read line");
    if base.trim() == "2" {
        println!("Please enter your encoded message");
        io::stdin().read_line(&mut encoded).expect("Failed to read line");
    }
     else if base.trim() == "r"{
        encoded = fs::read_to_string("rustmanmsg.txt").expect("Failed to read rustmanmsg.txt");
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

fn input_to_file(input:String, path:String) {
    let mut file = File::create(String::from(format!("{}.txt",path))).unwrap();
    file.write_all(input.as_bytes()).unwrap();
}
