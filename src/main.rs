use std::{
    collections::HashMap,
    env::args,
    fs::{File, read_to_string},
    io::Write,
    vec,
};
fn main() {
    // Collects arguments
    let args: Vec<String> = args().collect();
    if &args[1].trim().to_lowercase() == "encode" {
        encode(args[2].to_string(), args[3].to_string());
    } else if &args[1].trim().to_lowercase() == "decode" {
        decode(args[2].to_string());
    } else if &args[1].trim().to_lowercase() == "convert" {
        if &args[2].trim().to_lowercase() == "txt" {
            txt_to_rmt(args[3].to_string());
        } else if &args[2].trim().to_lowercase() == "rmt" {
            rmt_to_txt(args[3].to_string());
        } else if &args[2].trim().to_lowercase() == "legacy" {
            legacy_rmt(args[3].to_string());
        } else if &args[2].trim().to_lowercase() == "rm1" {
            rm1(args[3].to_string());
        }
    } else {
        println!("Unknown operation");
    }
    println!("Thanks for using Rustman.")
}
fn encode(mut input: String, path: String) {
    // Trims input string
    let mut input_chars: Vec<char> = input.trim().chars().collect();
    input_chars.reverse();
    let input_chars_ending = format!(
        "{}{}{}{}",
        input_chars[0], input_chars[1], input_chars[2], input_chars[3]
    );
    let input_string;
    if input_chars_ending == "txt." {
        input_string = read_to_string(&input).expect("Failed to read file");
        input = input_string.trim().to_string();
    } else {
        input_string = input.trim().to_string();
    }
    println!("{}", input_string);
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
        let value1 = huffman_list[huffman_len - 1][1]
            .trim()
            .parse::<i32>()
            .unwrap();
        let value2 = huffman_list[huffman_len - 2][1]
            .trim()
            .parse::<i32>()
            .unwrap();
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
        let string = String::from(format!("{} {}\n", key, val));
        hash_string.push_str(&string);
    }
    let mut lines_final_strings = Vec::new();
    let mut numlines = 0;
    for lines in input_string.lines() {
        numlines += 1;
        let mut final_string_list = vec![];
        for char in lines.chars() {
            let mut char = char;
            if char == ' ' {
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
        lines_final_strings.push(final_string);
    }
    let mut lines_final_strings_string = String::new();
    let mut first_line_string = true;
    for string in lines_final_strings {
        if first_line_string == false {
            lines_final_strings_string = format!("{}\n{}", lines_final_strings_string, string);
        } else {
            lines_final_strings_string = string;
            first_line_string = false;
        }
    }
    let rmt = String::from(format!(
        "{}\n{}\n{}",
        numlines, lines_final_strings_string, hash_string
    ));
    input_to_file(rmt, path, true);
    println!("Encoded message: \n {}", input_string);
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

fn decode(path: String) {
    let mut binary_codes: HashMap<String, char> = HashMap::new();
    let rmt =
        read_to_string(String::from(format!("{}.rmt", path))).expect("Failed to read rmt file");
    let rmt_lines: Vec<&str> = rmt.lines().collect();
    let num_lines = rmt_lines[0].parse::<usize>().unwrap();
    let mut encoded_list = Vec::new();
    for index in 1..=num_lines {
        encoded_list.push(rmt_lines[index])
    }

    for index in num_lines + 1..rmt_lines.len() {
        let lines = rmt_lines[index];
        let list_of_chars = lines.chars().collect::<Vec<char>>();
        let letter = list_of_chars[0];
        let mut bc = String::new();
        for char in 2..list_of_chars.len() {
            bc.push(list_of_chars[char]);
        }
        binary_codes.insert(bc, letter);
    }
    let mut first_line = true;
    let mut overall_message = String::new();
    for i in 0..=num_lines - 1 {
        let mut message = String::new();
        let mut current_string = String::new();
        for char in encoded_list[i].chars() {
            current_string.push(char);
            let current_ref = &current_string;
            if binary_codes.contains_key(current_ref) {
                let mut letter = binary_codes[&current_string];
                if letter == '\u{2423}' {
                    letter = ' ';
                }
                message.push(letter);
                current_string = String::new();
            }
        }
        if first_line == true {
            overall_message = message;
            first_line = false;
        } else {
            overall_message = format!("{}\n{}", overall_message, message);
        }
    }
    println!("{}", overall_message);
    input_to_file(
        overall_message,
        String::from(format!("{}rmtd", path)),
        false,
    );
}
fn order_by_value_list(list: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut list_mut = list.clone();
    list_mut.sort_by(|a, b| a[0].cmp(&b[0]));
    list_mut.sort_by(|a, b| {
        a[1].trim()
            .parse::<i32>()
            .unwrap()
            .cmp(&b[1].trim().parse::<i32>().unwrap())
    });
    list_mut.reverse();
    list_mut
}
fn input_to_file(input: String, path: String, rmt: bool) {
    if rmt == true {
        let mut file = File::create(String::from(format!("{}.rmt", path))).unwrap();
        file.write_all(input.as_bytes()).unwrap();
    } else {
        let mut file = File::create(String::from(format!("{}.txt", path))).unwrap();
        file.write_all(input.as_bytes()).unwrap();
    }
}

fn txt_to_rmt(path: String) {
    // Gets the binary message file
    let msg_file = read_to_string(String::from(format!("{}msg.txt", path))).unwrap();
    // Gets the hash file
    let hash_file = read_to_string(String::from(format!("{}hash.txt", path))).unwrap();
    // Stitches said Strings from files together
    let rmt = String::from(format!("1\n{}\n{}", msg_file, hash_file));
    // Writes string to file (*.rmt)
    input_to_file(rmt, path, true);
}
fn legacy_rmt(path: String) {
    let rmt = read_to_string(String::from(format!("{}.rmt", path))).unwrap();
    let fixed = format!("1\n{}", rmt);
    input_to_file(fixed, String::from(format!("{}fixed", path)), true);
}

fn rm1 (path: String) {
    let rmt = read_to_string(String::from(format!("{}.rmt", path))).unwrap();
    let mut lines = rmt.lines().collect::<Vec<&str>>();
    lines.remove(0);
    let mut fixed = String::new();
    let mut first_line = true;
    for line in lines {
        if first_line {
            fixed = line.to_string();
            first_line = false;
        } else {
            fixed = format!("{}\n{}", fixed, line);
        }
    }
    input_to_file(fixed, String::from(format!("{}legacy", path)), true);
}
fn rmt_to_txt(path: String) {
    // Gets the rmt file
    let rmt = read_to_string(String::from(format!("{}.rmt", &path))).unwrap();
    if rmt.lines().collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap() > 1{
        println!("WARNING! THIS OPERATION WILL FAIL. YOU ARE TRYING TO CONVERT A MULTI-LINE STRING RMT. TXTs DO NOT SUPPORT RUSTMAN MULTILINE!");
    }
    // Used for removing first line
    let mut index = 0;
    let mut hash_file = String::new();
    for lines in rmt.lines() {
        if index == 0 {
            index = 1;
        }
        else if index == 1 {
            // Writes the msg file
            input_to_file(
                lines.to_string(),
                String::from(format!("{}msg", path)),
                false,
            );
            index = 2;
        } else if index == 2 {
            // Otherwise there is an empty line at the top of the hash file
            hash_file = lines.to_string();
            index = 3;
        } else {
            // Then adds each line as a new line to the hash txt file
            hash_file = String::from(format!("{}\n{}", hash_file, lines));
        }
    }
    // Writes the hash string to the hash txt file
    input_to_file(hash_file, String::from(format!("{}hash", path)), false);
}
