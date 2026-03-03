use std::collections::HashMap;
use std::{io, vec};
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
    let mut letter_frequency_map = HashMap::new();
    for (key,value) in &letter_frequency {
        let key = key.to_string();
        letter_frequency_map.insert(key,*value);
    }
    let sorted_frequency = order_by_value(&letter_frequency_map);
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
            key1 = " ";
        }
        if key2 == ""{
            key2 = " ";
        }
        let key = String::from(format!("({}{})", key1, key2));
        let key_weight_list = vec![key, weight];
        huffman_list[huffman_len-2] = key_weight_list;
        huffman_list.remove(huffman_len - 1);
        huffman_list = order_by_value_list(huffman_list);
    }
    println!("{} with total weight {}", huffman_list[0][0], huffman_list[0][1]);
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

fn order_by_value(frequency_map:&HashMap<String, i32>) -> Vec<Vec<String>>{
    let mut list_freq:Vec<Vec<String>> = Vec::new();
    let mut index = 0;
    for (key, value) in frequency_map{
        let key_string = key.to_string();
        let value_string = value.to_string();
        let kvi_list = vec![key_string, value_string];
        list_freq.insert(index, kvi_list);
        index += 1;
    }
    list_freq.sort_by(|a,b|a[1].trim().parse::<i32>().unwrap().cmp(&b[1].trim().parse::<i32>().unwrap()));
    list_freq.reverse();
    list_freq
}

fn order_by_value_list(list:Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut list_mut = list.clone();
    list_mut.sort_by(|a,b|a[1].trim().parse::<i32>().unwrap().cmp(&b[1].trim().parse::<i32>().unwrap()));
    list_mut.reverse();
    list_mut
}